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
    service_types::impls::MessageInfo,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct NodeReference {
    pub node_id: NodeId,
    pub reference_type_id: NodeId,
    pub is_forward: bool,
    pub referenced_node_ids: Option<Vec<NodeId>>,
}

impl MessageInfo for NodeReference {
    fn object_id(&self) -> ObjectId {
        ObjectId::NodeReference_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<NodeReference> for NodeReference {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.reference_type_id.byte_len();
        size += self.is_forward.byte_len();
        size += byte_len_array(&self.referenced_node_ids);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_forward.encode(stream)?;
        size += write_array(stream, &self.referenced_node_ids)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream, decoding_options)?;
        let reference_type_id = NodeId::decode(stream, decoding_options)?;
        let is_forward = bool::decode(stream, decoding_options)?;
        let referenced_node_ids: Option<Vec<NodeId>> = read_array(stream, decoding_options)?;
        Ok(NodeReference {
            node_id,
            reference_type_id,
            is_forward,
            referenced_node_ids,
        })
    }
}
