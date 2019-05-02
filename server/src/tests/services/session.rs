use crate::tests::*;

use crate::builder::ServerBuilder;

#[test]
fn anonymous_user_token() {
    let server = ServerBuilder::new_sample().server().unwrap();
    let server_state = server.server_state();
    let server_state = server_state.read().unwrap();

    // Makes an anonymous token and sticks it into an extension object
    let token = AnonymousIdentityToken {
        policy_id: UAString::from(SecurityPolicy::None.to_uri())
    };
    let token = ExtensionObject::from_encodable(ObjectId::AnonymousIdentityToken_Encoding_DefaultBinary, &token);

    let server_nonce = ByteString::random(20);

    let result = server_state.authenticate_endpoint("opc.tcp://localhost:4855/", SecurityPolicy::None, MessageSecurityMode::None, &token, &server_nonce);
    trace!("result = {:?}", result);
    assert!(result.is_ok());

    let result = server_state.authenticate_endpoint("opc.tcp://localhost:4855/x", SecurityPolicy::None, MessageSecurityMode::None, &token, &server_nonce);
    trace!("result = {:?}", result);
    assert_eq!(result.unwrap_err(), StatusCode::BadTcpEndpointUrlInvalid);

    let result = server_state.authenticate_endpoint("opc.tcp://localhost:4855/noaccess", SecurityPolicy::None, MessageSecurityMode::None, &token, &server_nonce);
    trace!("result = {:?}", result);
    assert_eq!(result.unwrap_err(), StatusCode::BadIdentityTokenRejected);
}

fn make_user_name_identity_token(user: &str, pass: &[u8]) -> ExtensionObject {
    let token = UserNameIdentityToken {
        policy_id: UAString::from(SecurityPolicy::None.to_uri()),
        user_name: UAString::from(user),
        password: ByteString::from(pass),
        encryption_algorithm: UAString::null(),
    };
    ExtensionObject::from_encodable(ObjectId::UserNameIdentityToken_Encoding_DefaultBinary, &token)
}

#[test]
fn user_name_pass_token() {
    let server = ServerBuilder::new_sample().server().unwrap();
    let server_state = server.server_state();
    let server_state = server_state.read().unwrap();

    let server_nonce = ByteString::random(20);

    // Test that a good user authenticates
    let token = make_user_name_identity_token("sample", b"sample1");
    let result = server_state.authenticate_endpoint("opc.tcp://localhost:4855/", SecurityPolicy::None, MessageSecurityMode::None, &token, &server_nonce);
    assert!(result.is_ok());

    // Invalid tests
    let token = make_user_name_identity_token("samplex", b"sample1");
    let result = server_state.authenticate_endpoint("opc.tcp://localhost:4855/", SecurityPolicy::None, MessageSecurityMode::None, &token, &server_nonce);
    assert_eq!(result.unwrap_err(), StatusCode::BadIdentityTokenRejected);

    let token = make_user_name_identity_token("sample", b"sample");
    let result = server_state.authenticate_endpoint("opc.tcp://localhost:4855/", SecurityPolicy::None, MessageSecurityMode::None, &token, &server_nonce);
    assert_eq!(result.unwrap_err(), StatusCode::BadIdentityTokenRejected);

    let token = make_user_name_identity_token("", b"sample");
    let result = server_state.authenticate_endpoint("opc.tcp://localhost:4855/", SecurityPolicy::None, MessageSecurityMode::None, &token, &server_nonce);
    assert_eq!(result.unwrap_err(), StatusCode::BadIdentityTokenRejected);
}
