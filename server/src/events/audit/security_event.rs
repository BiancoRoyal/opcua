use opcua_types::*;

use crate::{
    address_space::address_space::AddressSpace,
    events::event::Event,
};

use super::{
    AuditEvent,
    event::AuditEventType
};

/// Base type for audit security events. Do not raise events of this type
pub(super) struct AuditSecurityEventType {
    base: AuditEventType
}

impl AuditEvent for AuditSecurityEventType {}

impl Event for AuditSecurityEventType {
    type Err = ();

    fn is_valid(&self) -> bool {
        self.base.is_valid()
    }

    fn raise(self, address_space: &mut AddressSpace) -> Result<NodeId, Self::Err> {
        self.base.raise(address_space)
    }
}

audit_event_impl!(AuditSecurityEventType, base);

impl AuditSecurityEventType {
    pub fn new<R, E, S, T, U>(node_id: R, event_type_id: E, browse_name: S, display_name: T, parent_node: U, time: DateTime) -> Self
        where R: Into<NodeId>,
              E: Into<NodeId>,
              S: Into<QualifiedName>,
              T: Into<LocalizedText>,
              U: Into<NodeId>,
    {
        Self {
            base: AuditEventType::new(node_id, event_type_id, browse_name, display_name, parent_node, time),
        }
    }
}

macro_rules! audit_security_event_impl {
    ( $event:ident, $base:ident ) => {
        audit_event_impl!($event, $base);
    }
}
