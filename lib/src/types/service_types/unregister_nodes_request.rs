// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock
//
// This file was autogenerated from Opc.Ua.Types.bsd by tools/schema/gen_types.js
//
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
#[allow(unused_imports)]
use crate::types::{
    basic_types::*, encoding::*, node_id::NodeId, node_ids::ObjectId,
    request_header::RequestHeader, service_types::impls::MessageInfo,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct UnregisterNodesRequest {
    pub request_header: RequestHeader,
    pub nodes_to_unregister: Option<Vec<NodeId>>,
}

impl MessageInfo for UnregisterNodesRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::UnregisterNodesRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<UnregisterNodesRequest> for UnregisterNodesRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += byte_len_array(&self.nodes_to_unregister);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += write_array(stream, &self.nodes_to_unregister)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_options)?;
        let nodes_to_unregister: Option<Vec<NodeId>> = read_array(stream, decoding_options)?;
        Ok(UnregisterNodesRequest {
            request_header,
            nodes_to_unregister,
        })
    }
}
