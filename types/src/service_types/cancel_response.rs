// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use crate::encoding::*;
#[allow(unused_imports)]
use crate::basic_types::*;
use crate::service_types::impls::MessageInfo;
use crate::node_ids::ObjectId;
use crate::service_types::impls::ResponseHeader;

/// Cancels an outstanding request.
#[derive(Debug, Clone, PartialEq)]
pub struct CancelResponse {
    pub response_header: ResponseHeader,
    pub cancel_count: u32,
}

impl MessageInfo for CancelResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::CancelResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CancelResponse> for CancelResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += self.cancel_count.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += self.cancel_count.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream, decoding_limits)?;
        let cancel_count = u32::decode(stream, decoding_limits)?;
        Ok(CancelResponse {
            response_header,
            cancel_count,
        })
    }
}
