use opcua_types::{
    NodeId, QualifiedName, LocalizedText, AttributeId, DataValue, WriteMask, Variant,
    service_types::NodeClass,
    status_code::StatusCode,
};

use crate::{
    address_space::types::{Base, Object, ObjectType, ReferenceType, Variable, VariableType, View, DataType, Method}
};

#[derive(Debug)]
pub enum NodeType {
    Object(Object),
    ObjectType(ObjectType),
    ReferenceType(ReferenceType),
    Variable(Variable),
    VariableType(VariableType),
    View(View),
    DataType(DataType),
    Method(Method),
}

pub trait HasNodeId {
    fn node_id(&self) -> NodeId;
}

impl HasNodeId for NodeType {
    fn node_id(&self) -> NodeId {
        self.as_node().node_id()
    }
}

impl NodeType {
    pub fn as_node(&self) -> &dyn NodeAttributes {
        match *self {
            NodeType::Object(ref value) => value,
            NodeType::ObjectType(ref value) => value,
            NodeType::ReferenceType(ref value) => value,
            NodeType::Variable(ref value) => value,
            NodeType::VariableType(ref value) => value,
            NodeType::View(ref value) => value,
            NodeType::DataType(ref value) => value,
            NodeType::Method(ref value) => value,
        }
    }

    pub fn as_mut_node(&mut self) -> &mut dyn NodeAttributes {
        match *self {
            NodeType::Object(ref mut value) => value,
            NodeType::ObjectType(ref mut value) => value,
            NodeType::ReferenceType(ref mut value) => value,
            NodeType::Variable(ref mut value) => value,
            NodeType::VariableType(ref mut value) => value,
            NodeType::View(ref mut value) => value,
            NodeType::DataType(ref mut value) => value,
            NodeType::Method(ref mut value) => value,
        }
    }
}

/// Implemented by Base and all derived Node types. Functions that return a result in an Option
/// do so because the attribute is optional and not necessarily there.
pub trait Node {
    fn base(&self) -> &Base;

    fn base_mut(&mut self) -> &mut Base;

    fn node_class(&self) -> NodeClass;

    fn node_id(&self) -> NodeId;

    fn browse_name(&self) -> QualifiedName;

    fn display_name(&self) -> LocalizedText;

    fn set_display_name(&mut self, display_name: LocalizedText);

    fn description(&self) -> Option<LocalizedText>;

    fn set_description(&mut self, description: LocalizedText);

    fn write_mask(&self) -> Option<WriteMask>;

    fn set_write_mask(&mut self, write_mask: WriteMask);

    fn user_write_mask(&self) -> Option<WriteMask>;

    fn set_user_write_mask(&mut self, write_mask: WriteMask);
}

/// This trait is for the benefit of the Attributes service set - Read and Write. Internal
/// callers should just call the setter / getter on the node itself if they have access to them.
pub trait NodeAttributes : Node {
    /// Finds the attribute and value. The param `max_age` is a hint in milliseconds:
    ///
    /// * value 0, server shall attempt to read a new value from the data source
    /// * value >= i32::max(), sever shall attempt to get a cached value
    ///
    /// If there is a getter registered with the node, then the getter will interpret
    /// `max_age` how it sees fit.
    fn get_attribute(&self, attribute_id: AttributeId, max_age: f64) -> Option<DataValue>;

    /// Sets the attribute with the new value
    fn set_attribute(&mut self, attribute_id: AttributeId, value: Variant) -> Result<(), StatusCode>;
}
