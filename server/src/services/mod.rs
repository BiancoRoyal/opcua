use opcua_types::{RequestHeader, ServiceFault, status_code::StatusCode};
use opcua_core::supported_message::SupportedMessage;

pub mod message_handler;

/// The implementation of a service, or a set of services will implement this trait
trait Service {
    fn name(&self) -> String;

    fn service_fault(&self, request_header: &RequestHeader, service_result: StatusCode) -> SupportedMessage {
        warn!("Service {}, request handle {} generated a service fault with status code {}", self.name(), request_header.request_handle, service_result);
        ServiceFault::new(request_header, service_result).into()
    }
}

pub mod attribute;
pub mod discovery;
pub mod method;
pub mod monitored_item;
pub mod node_management;
pub mod session;
pub mod subscription;
pub mod view;
