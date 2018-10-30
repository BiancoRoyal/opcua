// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use crate::encoding::*;
#[allow(unused_imports)]
use crate::basic_types::*;
use crate::service_types::impls::MessageInfo;
use crate::node_ids::ObjectId;
use crate::service_types::impls::RequestHeader;
use crate::service_types::enums::TimestampsToReturn;
use crate::service_types::MonitoredItemCreateRequest;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateMonitoredItemsRequest {
    pub request_header: RequestHeader,
    pub subscription_id: u32,
    pub timestamps_to_return: TimestampsToReturn,
    pub items_to_create: Option<Vec<MonitoredItemCreateRequest>>,
}

impl MessageInfo for CreateMonitoredItemsRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::CreateMonitoredItemsRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CreateMonitoredItemsRequest> for CreateMonitoredItemsRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.subscription_id.byte_len();
        size += self.timestamps_to_return.byte_len();
        size += byte_len_array(&self.items_to_create);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += self.timestamps_to_return.encode(stream)?;
        size += write_array(stream, &self.items_to_create)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let subscription_id = u32::decode(stream, decoding_limits)?;
        let timestamps_to_return = TimestampsToReturn::decode(stream, decoding_limits)?;
        let items_to_create: Option<Vec<MonitoredItemCreateRequest>> = read_array(stream, decoding_limits)?;
        Ok(CreateMonitoredItemsRequest {
            request_header,
            subscription_id,
            timestamps_to_return,
            items_to_create,
        })
    }
}
