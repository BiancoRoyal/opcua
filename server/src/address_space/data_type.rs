use opcua_types::service_types::DataTypeAttributes;

use crate::address_space::{base::Base, node::Node, node::NodeAttributes};

#[derive(Debug)]
pub struct DataType {
    base: Base,
    is_abstract: bool,
}

node_impl!(DataType);

impl NodeAttributes for DataType {
    fn get_attribute(&self, attribute_id: AttributeId, max_age: f64) -> Option<DataValue> {
        self.base.get_attribute(attribute_id, max_age).or_else(|| {
            match attribute_id {
                AttributeId::IsAbstract => Some(Variant::from(self.is_abstract())),
                _ => None
            }.map(|v| v.into())
        })
    }

    fn set_attribute(&mut self, attribute_id: AttributeId, value: Variant) -> Result<(), StatusCode> {
        if let Some(value) = self.base.set_attribute(attribute_id, value)? {
            match attribute_id {
                AttributeId::IsAbstract => {
                    if let Variant::Boolean(v) = value {
                        self.set_is_abstract(v);
                        Ok(())
                    } else {
                        Err(StatusCode::BadTypeMismatch)
                    }
                }
                _ => Err(StatusCode::BadAttributeIdInvalid)
            }
        } else {
            Ok(())
        }
    }
}

impl DataType {
    pub fn new<R, S>(node_id: &NodeId, browse_name: R, display_name: S, is_abstract: bool) -> DataType
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
    {
        DataType {
            base: Base::new(NodeClass::DataType, node_id, browse_name, display_name),
            is_abstract,
        }
    }

    pub fn from_attributes<S>(node_id: &NodeId, browse_name: S, attributes: DataTypeAttributes) -> Result<Self, ()>
        where S: Into<QualifiedName>
    {
        let mask = AttributesMask::from_bits(attributes.specified_attributes).ok_or(())?;
        if mask.contains(AttributesMask::DISPLAY_NAME | AttributesMask::IS_ABSTRACT) {
            let mut node = Self::new(node_id, browse_name, attributes.display_name, attributes.is_abstract);
            if mask.contains(AttributesMask::DESCRIPTION) {
                node.set_description(attributes.description);
            }
            if mask.contains(AttributesMask::WRITE_MASK) {
                node.set_write_mask(WriteMask::from_bits_truncate(attributes.write_mask));
            }
            if mask.contains(AttributesMask::USER_WRITE_MASK) {
                node.set_user_write_mask(WriteMask::from_bits_truncate(attributes.user_write_mask));
            }
            Ok(node)
        } else {
            error!("DataType cannot be created from attributes - missing mandatory values");
            Err(())
        }
    }

    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    pub fn set_is_abstract(&mut self, is_abstract: bool) {
        self.is_abstract = is_abstract;
    }
}
