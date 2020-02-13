// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    request_header::RequestHeader,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSubscriptionRequest {
    pub request_header: RequestHeader,
    pub requested_publishing_interval: f64,
    pub requested_lifetime_count: u32,
    pub requested_max_keep_alive_count: u32,
    pub max_notifications_per_publish: u32,
    pub publishing_enabled: bool,
    pub priority: u8,
}

impl MessageInfo for CreateSubscriptionRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::CreateSubscriptionRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CreateSubscriptionRequest> for CreateSubscriptionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.requested_publishing_interval.byte_len();
        size += self.requested_lifetime_count.byte_len();
        size += self.requested_max_keep_alive_count.byte_len();
        size += self.max_notifications_per_publish.byte_len();
        size += self.publishing_enabled.byte_len();
        size += self.priority.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.requested_publishing_interval.encode(stream)?;
        size += self.requested_lifetime_count.encode(stream)?;
        size += self.requested_max_keep_alive_count.encode(stream)?;
        size += self.max_notifications_per_publish.encode(stream)?;
        size += self.publishing_enabled.encode(stream)?;
        size += self.priority.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let requested_publishing_interval = f64::decode(stream, decoding_limits)?;
        let requested_lifetime_count = u32::decode(stream, decoding_limits)?;
        let requested_max_keep_alive_count = u32::decode(stream, decoding_limits)?;
        let max_notifications_per_publish = u32::decode(stream, decoding_limits)?;
        let publishing_enabled = bool::decode(stream, decoding_limits)?;
        let priority = u8::decode(stream, decoding_limits)?;
        Ok(CreateSubscriptionRequest {
            request_header,
            requested_publishing_interval,
            requested_lifetime_count,
            requested_max_keep_alive_count,
            max_notifications_per_publish,
            publishing_enabled,
            priority,
        })
    }
}
