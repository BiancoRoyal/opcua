use opcua_types::*;

use crate::{
    address_space::address_space::AddressSpace,
    events::event::Event,
};

use super::{
    session_events::AuditSessionEventType,
    AuditEvent,
};

pub struct AuditCancelEventType {
    base: AuditSessionEventType,
    request_handle: u32,
}

impl AuditEvent for AuditCancelEventType {}

impl Event for AuditCancelEventType {
    type Err = ();

    fn is_valid(&self) -> bool {
        self.base.is_valid()
    }

    fn raise(self, address_space: &mut AddressSpace) -> Result<NodeId, Self::Err> {
        let node_id = self.base.raise(address_space)?;
        let ns = node_id.namespace;
        Self::add_property(&node_id, NodeId::next_numeric(ns), "RequestHandle", "RequestHandle", self.request_handle, address_space);
        Ok(node_id)
    }
}

audit_session_event_impl!(AuditCancelEventType, base);

impl AuditCancelEventType {
    pub fn request_handle(mut self, request_handle: u32) -> Self {
        self.request_handle = request_handle;
        self
    }
}
