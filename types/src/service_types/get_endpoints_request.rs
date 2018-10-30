// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use crate::encoding::*;
#[allow(unused_imports)]
use crate::basic_types::*;
use crate::service_types::impls::MessageInfo;
use crate::node_ids::ObjectId;
use crate::service_types::impls::RequestHeader;
use crate::string::UAString;

/// Gets the endpoints used by the server.
#[derive(Debug, Clone, PartialEq)]
pub struct GetEndpointsRequest {
    pub request_header: RequestHeader,
    pub endpoint_url: UAString,
    pub locale_ids: Option<Vec<UAString>>,
    pub profile_uris: Option<Vec<UAString>>,
}

impl MessageInfo for GetEndpointsRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::GetEndpointsRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<GetEndpointsRequest> for GetEndpointsRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.endpoint_url.byte_len();
        size += byte_len_array(&self.locale_ids);
        size += byte_len_array(&self.profile_uris);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.endpoint_url.encode(stream)?;
        size += write_array(stream, &self.locale_ids)?;
        size += write_array(stream, &self.profile_uris)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let endpoint_url = UAString::decode(stream, decoding_limits)?;
        let locale_ids: Option<Vec<UAString>> = read_array(stream, decoding_limits)?;
        let profile_uris: Option<Vec<UAString>> = read_array(stream, decoding_limits)?;
        Ok(GetEndpointsRequest {
            request_header,
            endpoint_url,
            locale_ids,
            profile_uris,
        })
    }
}
