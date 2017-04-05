// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MonitoredItemCreateResult {
    pub status_code: StatusCode,
    pub monitored_item_id: UInt32,
    pub revised_sampling_interval: Double,
    pub revised_queue_size: UInt32,
    pub filter_result: ExtensionObject,
}

impl MessageInfo for MonitoredItemCreateResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::MonitoredItemCreateResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<MonitoredItemCreateResult> for MonitoredItemCreateResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += self.monitored_item_id.byte_len();
        size += self.revised_sampling_interval.byte_len();
        size += self.revised_queue_size.byte_len();
        size += self.filter_result.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += self.monitored_item_id.encode(stream)?;
        size += self.revised_sampling_interval.encode(stream)?;
        size += self.revised_queue_size.encode(stream)?;
        size += self.filter_result.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream)?;
        let monitored_item_id = UInt32::decode(stream)?;
        let revised_sampling_interval = Double::decode(stream)?;
        let revised_queue_size = UInt32::decode(stream)?;
        let filter_result = ExtensionObject::decode(stream)?;
        Ok(MonitoredItemCreateResult {
            status_code: status_code,
            monitored_item_id: monitored_item_id,
            revised_sampling_interval: revised_sampling_interval,
            revised_queue_size: revised_queue_size,
            filter_result: filter_result,
        })
    }
}