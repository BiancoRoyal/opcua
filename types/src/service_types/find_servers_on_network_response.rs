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
    response_header::ResponseHeader,
    date_time::DateTime,
    service_types::ServerOnNetwork,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FindServersOnNetworkResponse {
    pub response_header: ResponseHeader,
    pub last_counter_reset_time: DateTime,
    pub servers: Option<Vec<ServerOnNetwork>>,
}

impl MessageInfo for FindServersOnNetworkResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::FindServersOnNetworkResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<FindServersOnNetworkResponse> for FindServersOnNetworkResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += self.last_counter_reset_time.byte_len();
        size += byte_len_array(&self.servers);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += self.last_counter_reset_time.encode(stream)?;
        size += write_array(stream, &self.servers)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream, decoding_limits)?;
        let last_counter_reset_time = DateTime::decode(stream, decoding_limits)?;
        let servers: Option<Vec<ServerOnNetwork>> = read_array(stream, decoding_limits)?;
        Ok(FindServersOnNetworkResponse {
            response_header,
            last_counter_reset_time,
            servers,
        })
    }
}
