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
    service_types::ViewDescription,
    service_types::NodeTypeDescription,
    service_types::ContentFilter,
};

#[derive(Debug, Clone, PartialEq)]
pub struct QueryFirstRequest {
    pub request_header: RequestHeader,
    pub view: ViewDescription,
    pub node_types: Option<Vec<NodeTypeDescription>>,
    pub filter: ContentFilter,
    pub max_data_sets_to_return: u32,
    pub max_references_to_return: u32,
}

impl MessageInfo for QueryFirstRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::QueryFirstRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<QueryFirstRequest> for QueryFirstRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.view.byte_len();
        size += byte_len_array(&self.node_types);
        size += self.filter.byte_len();
        size += self.max_data_sets_to_return.byte_len();
        size += self.max_references_to_return.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.view.encode(stream)?;
        size += write_array(stream, &self.node_types)?;
        size += self.filter.encode(stream)?;
        size += self.max_data_sets_to_return.encode(stream)?;
        size += self.max_references_to_return.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let view = ViewDescription::decode(stream, decoding_limits)?;
        let node_types: Option<Vec<NodeTypeDescription>> = read_array(stream, decoding_limits)?;
        let filter = ContentFilter::decode(stream, decoding_limits)?;
        let max_data_sets_to_return = u32::decode(stream, decoding_limits)?;
        let max_references_to_return = u32::decode(stream, decoding_limits)?;
        Ok(QueryFirstRequest {
            request_header,
            view,
            node_types,
            filter,
            max_data_sets_to_return,
            max_references_to_return,
        })
    }
}
