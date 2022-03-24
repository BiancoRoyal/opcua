// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock
//
// This file was autogenerated from Opc.Ua.Types.bsd by tools/schema/gen_types.js
//
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
use std::io::{Read, Write};
#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    node_id::NodeId,
    service_types::enums::PerformUpdateType,
    data_value::DataValue,
};

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateDataDetails {
    pub node_id: NodeId,
    pub perform_insert_replace: PerformUpdateType,
    pub update_values: Option<Vec<DataValue>>,
}

impl BinaryEncoder<UpdateDataDetails> for UpdateDataDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.perform_insert_replace.byte_len();
        size += byte_len_array(&self.update_values);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.perform_insert_replace.encode(stream)?;
        size += write_array(stream, &self.update_values)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream, decoding_options)?;
        let perform_insert_replace = PerformUpdateType::decode(stream, decoding_options)?;
        let update_values: Option<Vec<DataValue>> = read_array(stream, decoding_options)?;
        Ok(UpdateDataDetails {
            node_id,
            perform_insert_replace,
            update_values,
        })
    }
}
