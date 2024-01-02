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
    basic_types::*, encoding::*, node_ids::ObjectId, service_types::impls::MessageInfo,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct SubscribedDataSetDataType {}

impl MessageInfo for SubscribedDataSetDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::SubscribedDataSetDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SubscribedDataSetDataType> for SubscribedDataSetDataType {
    fn byte_len(&self) -> usize {
        0
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        Ok(SubscribedDataSetDataType {})
    }
}
