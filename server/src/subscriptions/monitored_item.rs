use std::result::Result;
use std::collections::{VecDeque, BTreeSet};

use opcua_types::{
    *,
    status_code::StatusCode,
    node_ids::ObjectId,
    service_types::{
        TimestampsToReturn, DataChangeFilter, ReadValueId, MonitoredItemCreateRequest, MonitoredItemModifyRequest, MonitoredItemNotification,
    },
};

use crate::{constants, address_space::AddressSpace};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) enum FilterType {
    None,
    DataChangeFilter(DataChangeFilter),
}

impl FilterType {
    pub fn from_filter(filter: &ExtensionObject) -> Result<FilterType, StatusCode> {
        // Check if the filter is a supported filter type
        let filter_type_id = &filter.node_id;
        if filter_type_id.is_null() {
            // No data filter was passed, so just a dumb value comparison
            Ok(FilterType::None)
        } else if filter_type_id == &ObjectId::DataChangeFilter_Encoding_DefaultBinary.into() {
            let decoding_limits = DecodingLimits::minimal();
            Ok(FilterType::DataChangeFilter(filter.decode_inner::<DataChangeFilter>(&decoding_limits)?))
        } else {
            error!("Requested data filter type is not supported, {:?}", filter_type_id);
            Err(StatusCode::BadFilterNotAllowed)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct MonitoredItem {
    monitored_item_id: u32,
    item_to_monitor: ReadValueId,
    monitoring_mode: MonitoringMode,
    // Triggered items are other monitored items in the same subscription which are reported if this
    // monitored item changes.
    triggered_items: BTreeSet<u32>,
    client_handle: u32,
    sampling_interval: Duration,
    filter: FilterType,
    discard_oldest: bool,
    queue_size: usize,
    /// The notification queue is arranged from oldest to newest, i.e. pop front gets the oldest
    /// message, pop back gets the most recent.
    notification_queue: VecDeque<MonitoredItemNotification>,
    queue_overflow: bool,
    timestamps_to_return: TimestampsToReturn,
    last_sample_time: DateTimeUtc,
    last_data_value: Option<DataValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TickResult {
    /// The value changed and it should be reported
    ReportValueChanged,
    /// The value changed and it should not be reported (sampling)
    ValueChanged,
    /// The value did not change
    NoChange,
}

impl MonitoredItem {
    pub fn new(now: &DateTimeUtc, monitored_item_id: u32, timestamps_to_return: TimestampsToReturn, request: &MonitoredItemCreateRequest) -> Result<MonitoredItem, StatusCode> {
        let filter = FilterType::from_filter(&request.requested_parameters.filter)?;
        let sampling_interval = Self::sanitize_sampling_interval(request.requested_parameters.sampling_interval);
        let queue_size = Self::sanitize_queue_size(request.requested_parameters.queue_size as usize);
        Ok(MonitoredItem {
            monitored_item_id,
            item_to_monitor: request.item_to_monitor.clone(),
            monitoring_mode: request.monitoring_mode,
            triggered_items: BTreeSet::new(),
            client_handle: request.requested_parameters.client_handle,
            sampling_interval,
            filter,
            discard_oldest: request.requested_parameters.discard_oldest,
            timestamps_to_return,
            last_sample_time: now.clone(),
            last_data_value: None,
            queue_size,
            notification_queue: VecDeque::with_capacity(queue_size),
            queue_overflow: false,
        })
    }

    /// Modifies the existing item with the values of the modify request. On success, the result
    /// holds the filter result.
    pub fn modify(&mut self, timestamps_to_return: TimestampsToReturn, request: &MonitoredItemModifyRequest) -> Result<ExtensionObject, StatusCode> {
        self.timestamps_to_return = timestamps_to_return;
        self.filter = FilterType::from_filter(&request.requested_parameters.filter)?;
        self.sampling_interval = Self::sanitize_sampling_interval(request.requested_parameters.sampling_interval);
        self.queue_size = Self::sanitize_queue_size(request.requested_parameters.queue_size as usize);
        self.client_handle = request.requested_parameters.client_handle;
        self.discard_oldest = request.requested_parameters.discard_oldest;

        // Shrink / grow the notification queue to the new threshold
        if self.notification_queue.len() > self.queue_size {
            // Discard old notifications
            let discard = self.queue_size - self.notification_queue.len();
            let _ = self.notification_queue.drain(0..discard);
            // TODO potential edge case with discard oldest behaviour
            // Shrink the queue
            self.notification_queue.shrink_to_fit();
        } else if self.notification_queue.capacity() < self.queue_size {
            // Reserve space for more elements
            let extra_capacity = self.queue_size - self.notification_queue.capacity();
            self.notification_queue.reserve(extra_capacity);
        }

        // DataChangeFilter has no result but if the impl adds support for EventFilter, AggregateFilter
        // then this filter result will have to be filled in.
        let filter_result = ExtensionObject::null();

        Ok(filter_result)
    }

    /// Adds or removes other monitored items which will be triggered when this monitored item changes
    pub fn set_triggering(&mut self, items_to_add: &[u32], items_to_remove: &[u32]) {
        // Spec says to process remove items before adding new ones.
        items_to_remove.iter().for_each(|i| { self.triggered_items.remove(i); });
        items_to_add.iter().for_each(|i| { self.triggered_items.insert(*i); });
    }

    /// Called repeatedly on the monitored item.
    ///
    /// If the monitored item has a negative interval and subscription interval has elapsed,
    /// the value is tested immediately. Otherwise, the monitored items sampling interval is enforced
    /// the subscriptions and controls the rate.
    ///
    /// Function returns a `TickResult` denoting if the value changed or not, and whether it should
    /// be reported.
    pub fn tick(&mut self, now: &DateTimeUtc, address_space: &AddressSpace, publishing_interval_elapsed: bool, resend_data: bool) -> TickResult {
        if self.monitoring_mode == MonitoringMode::Disabled {
            TickResult::NoChange
        } else {
            let check_value = if resend_data {
                // Always check for resend_data flag
                true
            } else if self.sampling_interval < 0f64 {
                // -1 means use the subscription publishing interval so if the publishing interval elapsed,
                // then this monitored item is evaluated otherwise it won't be.
                publishing_interval_elapsed
            } else if self.sampling_interval == 0f64 {
                // 0 means fastest practical rate, i.e. the tick quantum itself
                // 0 is also used for clients subscribing for events.
                true
            } else {
                // Compare sample interval to the time elapsed
                let sampling_interval = super::duration_from_ms(self.sampling_interval);
                let elapsed = now.signed_duration_since(self.last_sample_time);
                elapsed >= sampling_interval
            };

            // Test the value (or don't)
            let value_changed = check_value && {
                // Indicate a change if reporting is enabled
                let first_tick = self.last_data_value.is_none();
                let value_changed = self.check_value(address_space, now, resend_data);
                first_tick || value_changed || !self.notification_queue.is_empty()
            };

            if value_changed {
                if self.monitoring_mode == MonitoringMode::Reporting {
                    TickResult::ReportValueChanged
                } else {
                    TickResult::ValueChanged
                }
            } else {
                TickResult::NoChange
            }
        }
    }

    /// Fetches the most recent value of the monitored item from the source and compares
    /// it to the last value. If the value has changed according to a filter / equality
    /// check, the latest value and its timestamps will be stored in the monitored item.
    ///
    /// The function will return true if the value was changed, false otherwise.
    pub fn check_value(&mut self, address_space: &AddressSpace, now: &DateTimeUtc, resend_data: bool) -> bool {
        if self.monitoring_mode == MonitoringMode::Disabled {
            panic!("Should not check value while monitoring mode is disabled");
        }
        self.last_sample_time = *now;
        if let Some(node) = address_space.find_node(&self.item_to_monitor.node_id) {
            let node = node.as_node();
            let attribute_id = AttributeId::from_u32(self.item_to_monitor.attribute_id);
            if attribute_id.is_err() {
                trace!("Item has no attribute_id {:?} so it hasn't changed, node {:?}", attribute_id, self.item_to_monitor.node_id);
                return false;
            }
            let attribute_id = attribute_id.unwrap();
            let data_value = node.get_attribute(attribute_id, 0.0);
            if let Some(mut data_value) = data_value {
                // Test for data change
                let data_change = if resend_data {
                    true
                } else if let Some(ref last_data_value) = self.last_data_value {
                    // If there is a filter on the monitored item then the filter determines
                    // if the value is considered to have changed, otherwise it is a straight
                    // equality test.
                    if let FilterType::DataChangeFilter(ref filter) = self.filter {
                        !filter.compare(&data_value, last_data_value, None)
                    } else {
                        data_value.value != last_data_value.value
                    }
                } else {
                    // There is no previous data value so yes consider it changed
                    trace!("No last data value so item has changed, node {:?}", self.item_to_monitor.node_id);
                    true
                };
                if data_change {
                    trace!("Data change on item -, node {:?}, data_value = {:?}", self.item_to_monitor.node_id, data_value);

                    // Store current data value to compare against on the next tick
                    self.last_data_value = Some(data_value.clone());

                    // Strip out timestamps that subscriber is not interested in
                    match self.timestamps_to_return {
                        TimestampsToReturn::Neither => {
                            data_value.source_timestamp = None;
                            data_value.source_picoseconds = None;
                            data_value.server_timestamp = None;
                            data_value.server_picoseconds = None
                        }
                        TimestampsToReturn::Server => {
                            data_value.source_timestamp = None;
                            data_value.source_picoseconds = None;
                        }
                        TimestampsToReturn::Source => {
                            data_value.server_timestamp = None;
                            data_value.server_picoseconds = None
                        }
                        TimestampsToReturn::Both => {
                            // DO NOTHING
                        }
                    }

                    // Enqueue notification message
                    let client_handle = self.client_handle;
                    self.enqueue_notification_message(MonitoredItemNotification {
                        client_handle,
                        value: data_value,
                    });

                    trace!("Monitored item state = {:?}", self);
                } else {
                    trace!("No data change on item, node {:?}", self.item_to_monitor.node_id);
                }
                data_change
            } else {
                false
            }
        } else {
            trace!("Cannot find item to monitor, node {:?}", self.item_to_monitor.node_id);
            false
        }
    }

    /// Enqueues a notification message for the monitored item
    pub fn enqueue_notification_message(&mut self, mut notification: MonitoredItemNotification) {
        // test for overflow
        let overflow = if self.notification_queue.len() == self.queue_size {
            trace!("Data change overflow, node {:?}", self.item_to_monitor.node_id);
            // Overflow behaviour
            if self.discard_oldest {
                // Throw away oldest item (the one at the start) to make space at the end
                let _ = self.notification_queue.pop_front();
            } else {
                // Remove the latest notification
                self.notification_queue.pop_back();
            }
            // Overflow only affects queues > 1 element
            self.queue_size > 1
        } else {
            false
        };
        if overflow {
            // Set the overflow bit on the data value's status
            let mut status_code = notification.value.status();
            status_code = status_code | StatusCode::OVERFLOW.bits();
            notification.value.status = Some(status_code);
            self.queue_overflow = true;
        }
        self.notification_queue.push_back(notification);
    }

    /// Gets the oldest notification message from the notification queue
    #[cfg(test)]
    pub fn oldest_notification_message(&mut self) -> Option<MonitoredItemNotification> {
        if self.notification_queue.is_empty() {
            None
        } else {
            self.queue_overflow = false;
            self.notification_queue.pop_front()
        }
    }

    /// Retrieves all the notification messages from the queue, oldest to newest
    pub fn all_notifications(&mut self) -> Option<Vec<MonitoredItemNotification>> {
        if self.notification_queue.is_empty() {
            None
        } else {
            // Removes all the queued notifications to the output
            self.queue_overflow = false;
            Some(self.notification_queue.drain(..).collect())
        }
    }

    /// Takes the requested sampling interval value supplied by client and ensures it is within
    /// the range supported by the server
    fn sanitize_sampling_interval(requested_sampling_interval: f64) -> f64 {
        if requested_sampling_interval < 0.0 {
            // From spec "any negative number is interpreted as -1"
            // -1 means monitored item's sampling interval defaults to the subscription's publishing interval
            -1.0
        } else if requested_sampling_interval == 0.0 || requested_sampling_interval < constants::MIN_SAMPLING_INTERVAL {
            constants::MIN_SAMPLING_INTERVAL
        } else {
            requested_sampling_interval
        }
    }

    /// Takes the requested queue size and ensures it is within the range supported by the server
    fn sanitize_queue_size(requested_queue_size: usize) -> usize {
        if requested_queue_size == 0 {
            // For data monitored items 0 -> 1
            1
            // Future - for event monitored items, queue size should be the default queue size for event notifications
        } else if requested_queue_size == 1 {
            1
            // Future - for event monitored items, the minimum queue size the server requires for event notifications
        } else if requested_queue_size > constants::MAX_DATA_CHANGE_QUEUE_SIZE {
            constants::MAX_DATA_CHANGE_QUEUE_SIZE
            // Future - for event monitored items MaxUInt32 returns the maximum queue size the server support
            // for event notifications
        } else {
            requested_queue_size
        }
    }

    pub fn monitored_item_id(&self) -> u32 {
        self.monitored_item_id
    }

    pub fn client_handle(&self) -> u32 {
        self.client_handle
    }

    pub fn sampling_interval(&self) -> Duration {
        self.sampling_interval
    }

    pub fn triggered_items(&self) -> &BTreeSet<u32> {
        &self.triggered_items
    }

    pub fn set_monitoring_mode(&mut self, monitoring_mode: MonitoringMode) {
        self.monitoring_mode = monitoring_mode;
    }

    pub fn monitoring_mode(&self) -> MonitoringMode {
        self.monitoring_mode
    }

    pub fn queue_size(&self) -> usize {
        self.queue_size
    }

    #[cfg(test)]
    pub fn queue_overflow(&self) -> bool {
        self.queue_overflow
    }

    #[cfg(test)]
    pub fn notification_queue(&self) -> &VecDeque<MonitoredItemNotification> {
        &self.notification_queue
    }

    #[cfg(test)]
    pub(crate) fn set_discard_oldest(&mut self, discard_oldest: bool) {
        self.discard_oldest = discard_oldest;
    }
}