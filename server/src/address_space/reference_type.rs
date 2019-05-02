use opcua_types::service_types::ReferenceTypeAttributes;

use crate::address_space::{base::Base, node::Node, node::NodeAttributes};

#[derive(Debug)]
pub struct ReferenceType {
    base: Base,
    symmetric: bool,
    is_abstract: bool,
    inverse_name: Option<LocalizedText>,
}

node_impl!(ReferenceType);

impl NodeAttributes for ReferenceType {
    fn get_attribute(&self, attribute_id: AttributeId, max_age: f64) -> Option<DataValue> {
        self.base.get_attribute(attribute_id, max_age).or_else(|| {
            match attribute_id {
                AttributeId::Symmetric => Some(Variant::from(self.symmetric())),
                AttributeId::IsAbstract => Some(Variant::from(self.is_abstract())),
                AttributeId::InverseName => {
                    if let Some(v) = self.inverse_name() {
                        Some(Variant::from(v))
                    } else {
                        None
                    }
                }
                _ => None
            }.map(|v| v.into())
        })
    }

    fn set_attribute(&mut self, attribute_id: AttributeId, value: Variant) -> Result<(), StatusCode> {
        if let Some(value) = self.base.set_attribute(attribute_id, value)? {
            match attribute_id {
                AttributeId::Symmetric => {
                    if let Variant::Boolean(v) = value {
                        self.symmetric = v;
                        Ok(())
                    } else {
                        Err(StatusCode::BadTypeMismatch)
                    }
                }
                AttributeId::IsAbstract => {
                    if let Variant::Boolean(v) = value {
                        self.is_abstract = v;
                        Ok(())
                    } else {
                        Err(StatusCode::BadTypeMismatch)
                    }
                }
                AttributeId::InverseName => {
                    if let Variant::LocalizedText(v) = value {
                        self.inverse_name = Some(*v);
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

impl ReferenceType {
    pub fn new<R, S>(node_id: &NodeId, browse_name: R, display_name: S, inverse_name: Option<LocalizedText>, symmetric: bool, is_abstract: bool) -> ReferenceType
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
    {
        ReferenceType {
            base: Base::new(NodeClass::ReferenceType, node_id, browse_name, display_name),
            symmetric,
            is_abstract,
            inverse_name,
        }
    }

    pub fn from_attributes<S>(node_id: &NodeId, browse_name: S, attributes: ReferenceTypeAttributes) -> Result<Self, ()>
        where S: Into<QualifiedName>
    {
        let mandatory_attributes = AttributesMask::DISPLAY_NAME | AttributesMask::IS_ABSTRACT | AttributesMask::SYMMETRIC;
        let mask = AttributesMask::from_bits(attributes.specified_attributes).ok_or(())?;
        if mask.contains(mandatory_attributes) {
            let mut node = Self::new(node_id, browse_name, attributes.display_name, None, false, false);
            if mask.contains(AttributesMask::DESCRIPTION) {
                node.set_description(attributes.description);
            }
            if mask.contains(AttributesMask::WRITE_MASK) {
                node.set_write_mask(WriteMask::from_bits_truncate(attributes.write_mask));
            }
            if mask.contains(AttributesMask::USER_WRITE_MASK) {
                node.set_user_write_mask(WriteMask::from_bits_truncate(attributes.user_write_mask));
            }
            if mask.contains(AttributesMask::IS_ABSTRACT) {
                node.set_is_abstract(attributes.is_abstract);
            }
            if mask.contains(AttributesMask::SYMMETRIC) {
                node.set_symmetric(attributes.is_abstract);
            }
            if mask.contains(AttributesMask::INVERSE_NAME) {
                node.set_inverse_name(attributes.inverse_name);
            }
            Ok(node)
        } else {
            error!("ReferenceType cannot be created from attributes - missing mandatory values");
            Err(())
        }
    }

    pub fn symmetric(&self) -> bool {
        self.symmetric
    }

    pub fn set_symmetric(&mut self, symmetric: bool) {
        self.symmetric = symmetric;
    }

    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    pub fn set_is_abstract(&mut self, is_abstract: bool) {
        self.is_abstract = is_abstract;
    }

    pub fn inverse_name(&self) -> Option<LocalizedText> {
        self.inverse_name.clone()
    }

    pub fn set_inverse_name(&mut self, inverse_name: LocalizedText) {
        self.inverse_name = Some(inverse_name);
    }
}
