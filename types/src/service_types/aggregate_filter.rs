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
    date_time::DateTime,
    node_id::NodeId,
    service_types::AggregateConfiguration,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AggregateFilter {
    pub start_time: DateTime,
    pub aggregate_type: NodeId,
    pub processing_interval: f64,
    pub aggregate_configuration: AggregateConfiguration,
}

impl BinaryEncoder<AggregateFilter> for AggregateFilter {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.start_time.byte_len();
        size += self.aggregate_type.byte_len();
        size += self.processing_interval.byte_len();
        size += self.aggregate_configuration.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.start_time.encode(stream)?;
        size += self.aggregate_type.encode(stream)?;
        size += self.processing_interval.encode(stream)?;
        size += self.aggregate_configuration.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let start_time = DateTime::decode(stream, decoding_options)?;
        let aggregate_type = NodeId::decode(stream, decoding_options)?;
        let processing_interval = f64::decode(stream, decoding_options)?;
        let aggregate_configuration = AggregateConfiguration::decode(stream, decoding_options)?;
        Ok(AggregateFilter {
            start_time,
            aggregate_type,
            processing_interval,
            aggregate_configuration,
        })
    }
}
