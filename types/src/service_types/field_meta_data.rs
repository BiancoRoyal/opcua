// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2020 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
#![rustfmt::skip]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    string::UAString,
    localized_text::LocalizedText,
    service_types::enums::DataSetFieldFlags,
    node_id::NodeId,
    guid::Guid,
    service_types::KeyValuePair,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FieldMetaData {
    pub name: UAString,
    pub description: LocalizedText,
    pub field_flags: DataSetFieldFlags,
    pub built_in_type: u8,
    pub data_type: NodeId,
    pub value_rank: i32,
    pub array_dimensions: Option<Vec<u32>>,
    pub max_string_length: u32,
    pub data_set_field_id: Guid,
    pub properties: Option<Vec<KeyValuePair>>,
}

impl MessageInfo for FieldMetaData {
    fn object_id(&self) -> ObjectId {
        ObjectId::FieldMetaData_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<FieldMetaData> for FieldMetaData {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.name.byte_len();
        size += self.description.byte_len();
        size += self.field_flags.byte_len();
        size += self.built_in_type.byte_len();
        size += self.data_type.byte_len();
        size += self.value_rank.byte_len();
        size += byte_len_array(&self.array_dimensions);
        size += self.max_string_length.byte_len();
        size += self.data_set_field_id.byte_len();
        size += byte_len_array(&self.properties);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.field_flags.encode(stream)?;
        size += self.built_in_type.encode(stream)?;
        size += self.data_type.encode(stream)?;
        size += self.value_rank.encode(stream)?;
        size += write_array(stream, &self.array_dimensions)?;
        size += self.max_string_length.encode(stream)?;
        size += self.data_set_field_id.encode(stream)?;
        size += write_array(stream, &self.properties)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let name = UAString::decode(stream, decoding_options)?;
        let description = LocalizedText::decode(stream, decoding_options)?;
        let field_flags = DataSetFieldFlags::decode(stream, decoding_options)?;
        let built_in_type = u8::decode(stream, decoding_options)?;
        let data_type = NodeId::decode(stream, decoding_options)?;
        let value_rank = i32::decode(stream, decoding_options)?;
        let array_dimensions: Option<Vec<u32>> = read_array(stream, decoding_options)?;
        let max_string_length = u32::decode(stream, decoding_options)?;
        let data_set_field_id = Guid::decode(stream, decoding_options)?;
        let properties: Option<Vec<KeyValuePair>> = read_array(stream, decoding_options)?;
        Ok(FieldMetaData {
            name,
            description,
            field_flags,
            built_in_type,
            data_type,
            value_rank,
            array_dimensions,
            max_string_length,
            data_set_field_id,
            properties,
        })
    }
}
