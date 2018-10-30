// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use crate::encoding::*;
#[allow(unused_imports)]
use crate::basic_types::*;
use crate::service_types::impls::MessageInfo;
use crate::node_ids::ObjectId;
use crate::service_types::enums::FilterOperator;
use crate::extension_object::ExtensionObject;

#[derive(Debug, Clone, PartialEq)]
pub struct ContentFilterElement {
    pub filter_operator: FilterOperator,
    pub filter_operands: Option<Vec<ExtensionObject>>,
}

impl MessageInfo for ContentFilterElement {
    fn object_id(&self) -> ObjectId {
        ObjectId::ContentFilterElement_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ContentFilterElement> for ContentFilterElement {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.filter_operator.byte_len();
        size += byte_len_array(&self.filter_operands);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.filter_operator.encode(stream)?;
        size += write_array(stream, &self.filter_operands)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let filter_operator = FilterOperator::decode(stream, decoding_limits)?;
        let filter_operands: Option<Vec<ExtensionObject>> = read_array(stream, decoding_limits)?;
        Ok(ContentFilterElement {
            filter_operator,
            filter_operands,
        })
    }
}
