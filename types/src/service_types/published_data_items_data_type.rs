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
    service_types::PublishedVariableDataType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PublishedDataItemsDataType {
    pub published_data: Option<Vec<PublishedVariableDataType>>,
}

impl BinaryEncoder<PublishedDataItemsDataType> for PublishedDataItemsDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.published_data);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.published_data)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let published_data: Option<Vec<PublishedVariableDataType>> = read_array(stream, decoding_options)?;
        Ok(PublishedDataItemsDataType {
            published_data,
        })
    }
}
