// This file was autogenerated by tools/schema/gen_supported_message.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use crate::{
    encoding::*,
    node_id::NodeId,
    service_types::*,
    node_ids::ObjectId,
    tcp_types::AcknowledgeMessage,
};

/// This macro helps avoid tedious repetition as new messages are added
/// The first form just handles the trailing comma after the last entry to save some pointless
/// editing when new messages are added to the list.
macro_rules! supported_messages_enum {
    [ $( $x:ident, ) * ] => (supported_messages_enum![ $( $x ),* ];);
    [ $( $x:ident ), * ] => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum SupportedMessage {
            /// An invalid request / response of some form
            Invalid(ObjectId),
            /// Acknowledge message
            AcknowledgeMessage(Box<AcknowledgeMessage>),
            /// Other messages
            $( $x(Box<$x>), )*
        }

        impl BinaryEncoder <SupportedMessage> for SupportedMessage {
            fn byte_len(&self) -> usize {
                match *self {
                    SupportedMessage::Invalid(object_id) => {
                        panic!("Unsupported message byte_len {:?}", object_id);
                    },
                    SupportedMessage::AcknowledgeMessage(ref value) => value.byte_len(),
                    $( SupportedMessage::$x(ref value) => value.byte_len(), )*
                }
            }

            fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
                match *self {
                    SupportedMessage::Invalid(object_id) => {
                        panic!("Unsupported message encode {:?}", object_id);
                    },
                    SupportedMessage::AcknowledgeMessage(ref value) => value.encode(stream),
                    $( SupportedMessage::$x(ref value) => value.encode(stream), )*
                }
            }

            fn decode<S: Read>(_: &mut S, _: &DecodingLimits) -> EncodingResult<Self> {
                // THIS WILL NOT DO ANYTHING
                panic!("Cannot decode a stream to a supported message type");
            }
        }

        impl Into<SupportedMessage> for AcknowledgeMessage{
            fn into(self) -> SupportedMessage { SupportedMessage::AcknowledgeMessage(Box::new(self)) }
        }

        $(
        impl Into<SupportedMessage> for $x {
            fn into(self) -> SupportedMessage { SupportedMessage::$x(Box::new(self)) }
        }
        )*

        impl SupportedMessage {
            pub fn node_id(&self) -> NodeId {
                match *self {
                    SupportedMessage::Invalid(object_id) => {
                        panic!("Unsupported message invalid, node_id {:?}", object_id);
                    },
                    SupportedMessage::AcknowledgeMessage(ref value) => {
                        panic!("Unsupported message node_id {:?}", value);
                    },
                    $( SupportedMessage::$x(ref value) => value.object_id().into(), )*
                }
            }
        }
    }
}

impl SupportedMessage {
    pub fn request_handle(&self) -> u32 {
        match *self {
            SupportedMessage::Invalid(_) | SupportedMessage::AcknowledgeMessage(_) => 0,
            SupportedMessage::ServiceFault(ref r) => r.response_header.request_handle,
            SupportedMessage::OpenSecureChannelRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::OpenSecureChannelResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CloseSecureChannelRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CloseSecureChannelResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::GetEndpointsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::GetEndpointsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::FindServersRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::FindServersResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::RegisterServerRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::RegisterServerResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CreateSessionRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CreateSessionResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CloseSessionRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CloseSessionResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CancelRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CancelResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::ActivateSessionRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::ActivateSessionResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::AddNodesRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::AddNodesResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::AddReferencesRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::AddReferencesResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::DeleteNodesRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::DeleteNodesResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::DeleteReferencesRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::DeleteReferencesResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CreateMonitoredItemsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CreateMonitoredItemsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::ModifyMonitoredItemsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::ModifyMonitoredItemsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::DeleteMonitoredItemsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::DeleteMonitoredItemsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::SetMonitoringModeRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::SetMonitoringModeResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::SetTriggeringRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::SetTriggeringResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CreateSubscriptionRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CreateSubscriptionResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::ModifySubscriptionRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::ModifySubscriptionResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::DeleteSubscriptionsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::DeleteSubscriptionsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::TransferSubscriptionsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::TransferSubscriptionsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::SetPublishingModeRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::SetPublishingModeResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::BrowseRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::BrowseResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::BrowseNextRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::BrowseNextResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::PublishRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::PublishResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::RepublishRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::RepublishResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::TranslateBrowsePathsToNodeIdsRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::TranslateBrowsePathsToNodeIdsResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::RegisterNodesRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::RegisterNodesResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::UnregisterNodesRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::UnregisterNodesResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::ReadRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::ReadResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::WriteRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::WriteResponse(ref r) => r.response_header.request_handle,
            SupportedMessage::CallRequest(ref r) => r.request_header.request_handle,
            SupportedMessage::CallResponse(ref r) => r.response_header.request_handle,
        }
    }

    pub fn decode_by_object_id<S: Read>(stream: &mut S, object_id: ObjectId, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        trace!("decoding object_id {:?}", object_id);
        let decoded_message = match object_id {
            ObjectId::ServiceFault_Encoding_DefaultBinary => {
                ServiceFault::decode(stream, decoding_limits)?.into()
            }
            ObjectId::OpenSecureChannelRequest_Encoding_DefaultBinary => {
                OpenSecureChannelRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::OpenSecureChannelResponse_Encoding_DefaultBinary => {
                OpenSecureChannelResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CloseSecureChannelRequest_Encoding_DefaultBinary => {
                CloseSecureChannelRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CloseSecureChannelResponse_Encoding_DefaultBinary => {
                CloseSecureChannelResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::GetEndpointsRequest_Encoding_DefaultBinary => {
                GetEndpointsRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::GetEndpointsResponse_Encoding_DefaultBinary => {
                GetEndpointsResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::FindServersRequest_Encoding_DefaultBinary => {
                FindServersRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::FindServersResponse_Encoding_DefaultBinary => {
                FindServersResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::RegisterServerRequest_Encoding_DefaultBinary => {
                RegisterServerRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::RegisterServerResponse_Encoding_DefaultBinary => {
                RegisterServerResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CreateSessionRequest_Encoding_DefaultBinary => {
                CreateSessionRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CreateSessionResponse_Encoding_DefaultBinary => {
                CreateSessionResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CloseSessionRequest_Encoding_DefaultBinary => {
                CloseSessionRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CloseSessionResponse_Encoding_DefaultBinary => {
                CloseSessionResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CancelRequest_Encoding_DefaultBinary => {
                CancelRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CancelResponse_Encoding_DefaultBinary => {
                CancelResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::ActivateSessionRequest_Encoding_DefaultBinary => {
                ActivateSessionRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::ActivateSessionResponse_Encoding_DefaultBinary => {
                ActivateSessionResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::AddNodesRequest_Encoding_DefaultBinary => {
                AddNodesRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::AddNodesResponse_Encoding_DefaultBinary => {
                AddNodesResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::AddReferencesRequest_Encoding_DefaultBinary => {
                AddReferencesRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::AddReferencesResponse_Encoding_DefaultBinary => {
                AddReferencesResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::DeleteNodesRequest_Encoding_DefaultBinary => {
                DeleteNodesRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::DeleteNodesResponse_Encoding_DefaultBinary => {
                DeleteNodesResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::DeleteReferencesRequest_Encoding_DefaultBinary => {
                DeleteReferencesRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::DeleteReferencesResponse_Encoding_DefaultBinary => {
                DeleteReferencesResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CreateMonitoredItemsRequest_Encoding_DefaultBinary => {
                CreateMonitoredItemsRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CreateMonitoredItemsResponse_Encoding_DefaultBinary => {
                CreateMonitoredItemsResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::ModifyMonitoredItemsRequest_Encoding_DefaultBinary => {
                ModifyMonitoredItemsRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::ModifyMonitoredItemsResponse_Encoding_DefaultBinary => {
                ModifyMonitoredItemsResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::DeleteMonitoredItemsRequest_Encoding_DefaultBinary => {
                DeleteMonitoredItemsRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::DeleteMonitoredItemsResponse_Encoding_DefaultBinary => {
                DeleteMonitoredItemsResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::SetMonitoringModeRequest_Encoding_DefaultBinary => {
                SetMonitoringModeRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::SetMonitoringModeResponse_Encoding_DefaultBinary => {
                SetMonitoringModeResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::SetTriggeringRequest_Encoding_DefaultBinary => {
                SetTriggeringRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::SetTriggeringResponse_Encoding_DefaultBinary => {
                SetTriggeringResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CreateSubscriptionRequest_Encoding_DefaultBinary => {
                CreateSubscriptionRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CreateSubscriptionResponse_Encoding_DefaultBinary => {
                CreateSubscriptionResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::ModifySubscriptionRequest_Encoding_DefaultBinary => {
                ModifySubscriptionRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::ModifySubscriptionResponse_Encoding_DefaultBinary => {
                ModifySubscriptionResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::DeleteSubscriptionsRequest_Encoding_DefaultBinary => {
                DeleteSubscriptionsRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::DeleteSubscriptionsResponse_Encoding_DefaultBinary => {
                DeleteSubscriptionsResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::TransferSubscriptionsRequest_Encoding_DefaultBinary => {
                TransferSubscriptionsRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::TransferSubscriptionsResponse_Encoding_DefaultBinary => {
                TransferSubscriptionsResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::SetPublishingModeRequest_Encoding_DefaultBinary => {
                SetPublishingModeRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::SetPublishingModeResponse_Encoding_DefaultBinary => {
                SetPublishingModeResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::BrowseRequest_Encoding_DefaultBinary => {
                BrowseRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::BrowseResponse_Encoding_DefaultBinary => {
                BrowseResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::BrowseNextRequest_Encoding_DefaultBinary => {
                BrowseNextRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::BrowseNextResponse_Encoding_DefaultBinary => {
                BrowseNextResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::PublishRequest_Encoding_DefaultBinary => {
                PublishRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::PublishResponse_Encoding_DefaultBinary => {
                PublishResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::RepublishRequest_Encoding_DefaultBinary => {
                RepublishRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::RepublishResponse_Encoding_DefaultBinary => {
                RepublishResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::TranslateBrowsePathsToNodeIdsRequest_Encoding_DefaultBinary => {
                TranslateBrowsePathsToNodeIdsRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::TranslateBrowsePathsToNodeIdsResponse_Encoding_DefaultBinary => {
                TranslateBrowsePathsToNodeIdsResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::RegisterNodesRequest_Encoding_DefaultBinary => {
                RegisterNodesRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::RegisterNodesResponse_Encoding_DefaultBinary => {
                RegisterNodesResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::UnregisterNodesRequest_Encoding_DefaultBinary => {
                UnregisterNodesRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::UnregisterNodesResponse_Encoding_DefaultBinary => {
                UnregisterNodesResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::ReadRequest_Encoding_DefaultBinary => {
                ReadRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::ReadResponse_Encoding_DefaultBinary => {
                ReadResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::WriteRequest_Encoding_DefaultBinary => {
                WriteRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::WriteResponse_Encoding_DefaultBinary => {
                WriteResponse::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CallRequest_Encoding_DefaultBinary => {
                CallRequest::decode(stream, decoding_limits)?.into()
            }
            ObjectId::CallResponse_Encoding_DefaultBinary => {
                CallResponse::decode(stream, decoding_limits)?.into()
            }

            _ => {
                debug!("decoding unsupported for object id {:?}", object_id);
                SupportedMessage::Invalid(object_id)
            }
        };
        Ok(decoded_message)
    }
}

// These are all the messages handled into and out of streams by the OPCUA server / client code
supported_messages_enum![
    ServiceFault,
    OpenSecureChannelRequest,
    OpenSecureChannelResponse,
    CloseSecureChannelRequest,
    CloseSecureChannelResponse,
    GetEndpointsRequest,
    GetEndpointsResponse,
    FindServersRequest,
    FindServersResponse,
    RegisterServerRequest,
    RegisterServerResponse,
    CreateSessionRequest,
    CreateSessionResponse,
    CloseSessionRequest,
    CloseSessionResponse,
    CancelRequest,
    CancelResponse,
    ActivateSessionRequest,
    ActivateSessionResponse,
    AddNodesRequest,
    AddNodesResponse,
    AddReferencesRequest,
    AddReferencesResponse,
    DeleteNodesRequest,
    DeleteNodesResponse,
    DeleteReferencesRequest,
    DeleteReferencesResponse,
    CreateMonitoredItemsRequest,
    CreateMonitoredItemsResponse,
    ModifyMonitoredItemsRequest,
    ModifyMonitoredItemsResponse,
    DeleteMonitoredItemsRequest,
    DeleteMonitoredItemsResponse,
    SetMonitoringModeRequest,
    SetMonitoringModeResponse,
    SetTriggeringRequest,
    SetTriggeringResponse,
    CreateSubscriptionRequest,
    CreateSubscriptionResponse,
    ModifySubscriptionRequest,
    ModifySubscriptionResponse,
    DeleteSubscriptionsRequest,
    DeleteSubscriptionsResponse,
    TransferSubscriptionsRequest,
    TransferSubscriptionsResponse,
    SetPublishingModeRequest,
    SetPublishingModeResponse,
    BrowseRequest,
    BrowseResponse,
    BrowseNextRequest,
    BrowseNextResponse,
    PublishRequest,
    PublishResponse,
    RepublishRequest,
    RepublishResponse,
    TranslateBrowsePathsToNodeIdsRequest,
    TranslateBrowsePathsToNodeIdsResponse,
    RegisterNodesRequest,
    RegisterNodesResponse,
    UnregisterNodesRequest,
    UnregisterNodesResponse,
    ReadRequest,
    ReadResponse,
    WriteRequest,
    WriteResponse,
    CallRequest,
    CallResponse,
];
