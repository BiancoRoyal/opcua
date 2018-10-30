// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use crate::encoding::*;
#[allow(unused_imports)]
use crate::basic_types::*;
use crate::service_types::impls::MessageInfo;
use crate::node_ids::ObjectId;

#[derive(Debug, Clone, PartialEq)]
pub struct EndpointConfiguration {
    pub operation_timeout: i32,
    pub use_binary_encoding: bool,
    pub max_string_length: i32,
    pub max_byte_string_length: i32,
    pub max_array_length: i32,
    pub max_message_size: i32,
    pub max_buffer_size: i32,
    pub channel_lifetime: i32,
    pub security_token_lifetime: i32,
}

impl MessageInfo for EndpointConfiguration {
    fn object_id(&self) -> ObjectId {
        ObjectId::EndpointConfiguration_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<EndpointConfiguration> for EndpointConfiguration {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.operation_timeout.byte_len();
        size += self.use_binary_encoding.byte_len();
        size += self.max_string_length.byte_len();
        size += self.max_byte_string_length.byte_len();
        size += self.max_array_length.byte_len();
        size += self.max_message_size.byte_len();
        size += self.max_buffer_size.byte_len();
        size += self.channel_lifetime.byte_len();
        size += self.security_token_lifetime.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.operation_timeout.encode(stream)?;
        size += self.use_binary_encoding.encode(stream)?;
        size += self.max_string_length.encode(stream)?;
        size += self.max_byte_string_length.encode(stream)?;
        size += self.max_array_length.encode(stream)?;
        size += self.max_message_size.encode(stream)?;
        size += self.max_buffer_size.encode(stream)?;
        size += self.channel_lifetime.encode(stream)?;
        size += self.security_token_lifetime.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let operation_timeout = i32::decode(stream, decoding_limits)?;
        let use_binary_encoding = bool::decode(stream, decoding_limits)?;
        let max_string_length = i32::decode(stream, decoding_limits)?;
        let max_byte_string_length = i32::decode(stream, decoding_limits)?;
        let max_array_length = i32::decode(stream, decoding_limits)?;
        let max_message_size = i32::decode(stream, decoding_limits)?;
        let max_buffer_size = i32::decode(stream, decoding_limits)?;
        let channel_lifetime = i32::decode(stream, decoding_limits)?;
        let security_token_lifetime = i32::decode(stream, decoding_limits)?;
        Ok(EndpointConfiguration {
            operation_timeout,
            use_binary_encoding,
            max_string_length,
            max_byte_string_length,
            max_array_length,
            max_message_size,
            max_buffer_size,
            channel_lifetime,
            security_token_lifetime,
        })
    }
}
