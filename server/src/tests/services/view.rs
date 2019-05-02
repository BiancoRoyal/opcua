use std::sync::Weak;

use opcua_console_logging;
use crate::services::view::ViewService;

use super::*;

// View service tests

fn make_browse_request(nodes: &[NodeId], max_references_per_node: usize, browse_direction: BrowseDirection, reference_type: ReferenceTypeId) -> BrowseRequest {
    let request_header = make_request_header();
    let nodes_to_browse = nodes.iter().map(|n| {
        BrowseDescription {
            node_id: n.clone(),
            browse_direction,
            reference_type_id: reference_type.into(),
            include_subtypes: true,
            node_class_mask: 0xff,
            result_mask: 0xff,
        }
    }).collect();
    BrowseRequest {
        request_header,
        view: ViewDescription {
            view_id: NodeId::null(),
            timestamp: DateTime::now(),
            view_version: 0,
        },
        requested_max_references_per_node: max_references_per_node as u32,
        nodes_to_browse: Some(nodes_to_browse),
    }
}

fn make_browse_next_request(continuation_point: &ByteString, release_continuation_points: bool) -> BrowseNextRequest {
    let request_header = make_request_header();
    BrowseNextRequest {
        request_header,
        release_continuation_points,
        continuation_points: if continuation_point.is_null() { None } else { Some(vec![continuation_point.clone()]) },
    }
}

fn verify_references_to_many_vars(references: &[ReferenceDescription], expected_size: usize, start_idx: usize) {
    // Verify that the reference descriptions point at sequential vars
    assert_eq!(references.len(), expected_size);
    for (i, r) in references.iter().enumerate() {
        assert_eq!(r.node_id.node_id, var_node_id(i + start_idx));
    }
}

fn do_view_service_test<F>(f: F)
    where F: FnOnce(&mut super::ServerState, &mut Session, Arc<RwLock<Session>>, &mut AddressSpace, &ViewService)
{
    opcua_console_logging::init();
    let st = ServiceTest::new();

    let mut server_state = st.server_state.write().unwrap();
    let mut session = st.session.write().unwrap();
    let mut address_space = st.address_space.write().unwrap();

    f(&mut server_state, &mut session, st.session.clone(), &mut address_space, &ViewService::new());
}

fn do_browse(vs: &ViewService, session: &mut Session, address_space: &AddressSpace, nodes: &[NodeId], max_references_per_node: usize) -> BrowseResponse {
    let request = make_browse_request(nodes, max_references_per_node, BrowseDirection::Forward, ReferenceTypeId::Organizes);
    let result = vs.browse(session, address_space, &request);
    assert!(result.is_ok());
    supported_message_as!(result.unwrap(), BrowseResponse)
}

fn do_browse_next(vs: &ViewService, session: &mut Session, address_space: &AddressSpace, continuation_point: &ByteString, release_continuation_points: bool) -> BrowseNextResponse {
    let request = make_browse_next_request(continuation_point, release_continuation_points);
    let result = vs.browse_next(session, address_space, &request);
    assert!(result.is_ok());
    supported_message_as!(result.unwrap(), BrowseNextResponse)
}

#[test]
fn browse() {
    do_view_service_test(|_server_state, session, _, address_space, vs| {
        add_sample_vars_to_address_space(address_space);

        let nodes: Vec<NodeId> = vec![ObjectId::RootFolder.into()];
        let response = do_browse(&vs, session, &address_space, &nodes, 1000);
        assert!(response.results.is_some());

        let results = response.results.unwrap();
        assert_eq!(results.len(), 1);

        assert!(results[0].references.is_some());
        let references = results[0].references.as_ref().unwrap();
        assert_eq!(references.len(), 3);

        // Expect to see refs to
        // Objects/
        // Types/
        // Views/

        let r1 = &references[0];
        assert_eq!(r1.browse_name, QualifiedName::new(0, "Objects"));
        let r2 = &references[1];
        assert_eq!(r2.browse_name, QualifiedName::new(0, "Types"));
        let r3 = &references[2];
        assert_eq!(r3.browse_name, QualifiedName::new(0, "Views"));
    });
}

#[test]
fn browse_next() {
    do_view_service_test(|_server_state, session, _, address_space, vs| {
        let parent_node_id = add_many_vars_to_address_space(address_space, 100).0;
        let nodes = vec![parent_node_id.clone()];

        // Browse with requested_max_references_per_node = 101, expect 100 results, no continuation point
        {
            let response = do_browse(&vs, session, &address_space, &nodes, 101);
            assert!(response.results.is_some());
            let r1 = &response.results.unwrap()[0];
            let references = r1.references.as_ref().unwrap();
            assert!(r1.continuation_point.is_null());
            verify_references_to_many_vars(references, 100, 0);
        }

        // Browse with requested_max_references_per_node = 100, expect 100 results, no continuation point
        {
            let response = do_browse(&vs, session, &address_space, &nodes, 100);
            let r1 = &response.results.unwrap()[0];
            let references = r1.references.as_ref().unwrap();
            assert!(r1.continuation_point.is_null());
            verify_references_to_many_vars(references, 100, 0);
        }

        // Browse with requested_max_references_per_node = 99 expect 99 results and a continuation point
        // Browse next with continuation point, expect 1 result leaving off from last continuation point
        let continuation_point = {
            // Get first 99
            let response = do_browse(&vs, session, &address_space, &nodes, 99);
            let r1 = &response.results.unwrap()[0];
            let references = r1.references.as_ref().unwrap();
            assert!(!r1.continuation_point.is_null());
            verify_references_to_many_vars(references, 99, 0);

            // Expect continuation point and browse next to return last var and no more continuation point
            let response = do_browse_next(&vs, session, &address_space, &r1.continuation_point, false);
            let r2 = &response.results.unwrap()[0];
            assert!(r2.continuation_point.is_null());
            let references = r2.references.as_ref().unwrap();
            verify_references_to_many_vars(references, 1, 99);

            // Browse next again with same continuation point, expect same 1 result
            let response = do_browse_next(&vs, session, &address_space, &r1.continuation_point, false);
            let r2 = &response.results.unwrap()[0];
            assert!(r2.continuation_point.is_null());
            let references = r2.references.as_ref().unwrap();
            verify_references_to_many_vars(references, 1, 99);

            r1.continuation_point.clone()
        };

        // Browse next and release the previous continuation points, expect Null result
        {
            let response = do_browse_next(&vs, session, &address_space, &continuation_point, true);
            assert!(response.results.is_none());

            // Browse next again with same continuation point, expect BadContinuationPointInvalid
            let response = do_browse_next(&vs, session, &address_space, &continuation_point, false);
            let r1 = &response.results.unwrap()[0];
            assert_eq!(r1.status_code, StatusCode::BadContinuationPointInvalid);
        }

        // Browse with 35 expect continuation point cp1
        // Browse next with cp1 with 35 expect cp2
        // Browse next with cp2 expect 30 results
        {
            // Get first 35
            let response = do_browse(&vs, session, &address_space, &nodes, 35);
            let r1 = &response.results.unwrap()[0];
            let references = r1.references.as_ref().unwrap();
            assert!(!r1.continuation_point.is_null());
            verify_references_to_many_vars(references, 35, 0);

            // Expect continuation point and browse next to return last var and no more continuation point
            let response = do_browse_next(&vs, session, &address_space, &r1.continuation_point, false);
            let r2 = &response.results.unwrap()[0];
            assert!(!r2.continuation_point.is_null());
            let references = r2.references.as_ref().unwrap();
            verify_references_to_many_vars(references, 35, 35);

            // Expect continuation point and browse next to return last var and no more continuation point
            let response = do_browse_next(&vs, session, &address_space, &r2.continuation_point, false);
            let r3 = &response.results.unwrap()[0];
            assert!(r3.continuation_point.is_null());
            let references = r3.references.as_ref().unwrap();
            verify_references_to_many_vars(references, 30, 70);
        }

        // Modify address space so existing continuation point is invalid
        // Browse next with continuation point, expect BadContinuationPointInvalid
        {
            use std::thread;
            use std::time::Duration;

            // Sleep a bit, modify the address space so the old continuation point is out of date
            thread::sleep(Duration::from_millis(50));
            {
                let var_name = "xxxx";
                let node_id = NodeId::new(1, var_name);
                let var = Variable::new(&node_id, var_name, var_name, 200 as i32);
                let _ = address_space.add_variable(var, &parent_node_id);
            }

            // Browsing with the old continuation point should fail
            let response = do_browse_next(&vs, session, &address_space, &continuation_point, false);
            let r1 = &response.results.unwrap()[0];
            assert_eq!(r1.status_code, StatusCode::BadContinuationPointInvalid);
        }
    });
}

#[test]
fn translate_browse_paths_to_node_ids() {
    do_view_service_test(|server_state, _session, _, address_space, vs| {
        // This is a very basic test of this service. It wants to find the relative path from root to the
        // Objects folder and ensure that it comes back in the result

        let browse_paths = vec![
            BrowsePath {
                starting_node: ObjectId::RootFolder.into(),
                relative_path: RelativePath {
                    elements: Some(vec![
                        RelativePathElement {
                            reference_type_id: ReferenceTypeId::Organizes.into(),
                            is_inverse: false,
                            include_subtypes: true,
                            target_name: QualifiedName::new(0, "Objects"),
                        }
                    ]),
                },
            }
        ];

        let request = TranslateBrowsePathsToNodeIdsRequest {
            request_header: make_request_header(),
            browse_paths: Some(browse_paths),
        };

        let result = vs.translate_browse_paths_to_node_ids(&server_state, &address_space, &request);
        assert!(result.is_ok());
        let response: TranslateBrowsePathsToNodeIdsResponse = supported_message_as!(result.unwrap(), TranslateBrowsePathsToNodeIdsResponse);

        debug!("result = {:#?}", response);

        let results = response.results.unwrap();
        assert_eq!(results.len(), 1);
        let r1 = &results[0];
        let targets = r1.targets.as_ref().unwrap();
        assert_eq!(targets.len(), 1);
        let t1 = &targets[0];
        assert_eq!(&t1.target_id.node_id, &AddressSpace::objects_folder_id());
    });
}

#[test]
fn translate_browse_paths_to_node_ids2() {
    do_view_service_test(|server_state, _session, _, address_space, vs| {

        // Inputs and outputs taken from this testcase in Node OPCUA
        //
        // https://github.com/node-opcua/node-opcua/blob/68b1b57dec23a45148468fbea89ab71a39f9042f/test/end_to_end/u_test_e2e_translateBrowsePath.js

        let starting_node: NodeId = ObjectId::RootFolder.into();

        let browse_paths = [
            "/Objects/Server",
            "/Objects/Server.ServerStatus",
            "/Objects/Server.ServerStatus.BuildInfo",
            "/Objects/Server.ServerStatus.BuildInfo.ProductName",
            "/Objects/Server.ServerStatus.BuildInfo.",
            "/Objects.Server",
            "/Objects/2:MatrikonOPC Simulation Server (DA)",
        ].iter().map(|path|
            BrowsePath {
                starting_node: starting_node.clone(),
                relative_path: RelativePath::from_str(path, &RelativePathElement::default_node_resolver).unwrap(),
            }
        ).collect::<Vec<_>>();

        let request = TranslateBrowsePathsToNodeIdsRequest {
            request_header: make_request_header(),
            browse_paths: Some(browse_paths),
        };

        let browse_paths_len = request.browse_paths.as_ref().unwrap().len();

        let result = vs.translate_browse_paths_to_node_ids(&server_state, &address_space, &request);
        assert!(result.is_ok());
        let response: TranslateBrowsePathsToNodeIdsResponse = supported_message_as!(result.unwrap(), TranslateBrowsePathsToNodeIdsResponse);

        let results = response.results.unwrap();
        assert_eq!(results.len(), browse_paths_len);

        let mut idx = 0;

        // results[0]
        {
            let r = &results[idx];
            assert!(r.status_code.is_good());
            let targets = r.targets.as_ref().unwrap();
            trace!("targets for {} = {:#?}", idx, targets);
            assert_eq!(targets.len(), 1);
            assert_eq!(&targets[0].target_id, &ObjectId::Server.into());
            idx += 1;
        }

        // results[1]
        {
            let r = &results[idx];
            assert!(r.status_code.is_good());
            let targets = r.targets.as_ref().unwrap();
            trace!("targets for {} = {:#?}", idx, targets);
            assert_eq!(targets.len(), 1);
            assert_eq!(&targets[0].target_id, &VariableId::Server_ServerStatus.into());
            idx += 1;
        }

        // results[2]
        {
            let r = &results[idx];
            assert!(r.status_code.is_good());
            let targets = r.targets.as_ref().unwrap();
            trace!("targets for {} = {:#?}", idx, targets);
            assert_eq!(targets.len(), 1);
            assert_eq!(&targets[0].target_id, &VariableId::Server_ServerStatus_BuildInfo.into());
            idx += 1;
        }

        // results[3]
        {
            let r = &results[idx];
            assert!(r.status_code.is_good());
            let targets = r.targets.as_ref().unwrap();
            trace!("targets for {} = {:#?}", idx, targets);
            assert_eq!(&targets[0].target_id, &VariableId::Server_ServerStatus_BuildInfo_ProductName.into());
            idx += 1;
        }

        // results[4]
        {
            let r = &results[idx];
            assert_eq!(r.status_code, StatusCode::BadBrowseNameInvalid);
            idx += 1;
        }

        // results[5]
        {
            let r = &results[idx];
            assert_eq!(r.status_code, StatusCode::BadNoMatch);
            idx += 1;
        }

        // results[6]
        {
            let r = &results[idx];
            assert_eq!(r.status_code, StatusCode::BadNoMatch);
            // idx += 1;
        }
    });
}

struct RegisterNodesImpl {
    pub session: Weak<RwLock<Session>>
}

impl RegisterNodes for RegisterNodesImpl {
    fn register_nodes(&mut self, session: Arc<RwLock<Session>>, nodes_to_register: &[NodeId]) -> Result<Vec<NodeId>, StatusCode> {
        let bad_node = ObjectId::ObjectsFolder.into();
        let good_node = NodeId::new(1, 100);
        let alias_node = NodeId::new(1, 200);

        if nodes_to_register.contains(&bad_node) {
            Err(StatusCode::BadNodeIdInvalid)
        } else {
            // Simulate holding a weak ref to the session
            self.session = Arc::downgrade(&session);

            // The result will be the input except for the good node which will be aliased on its
            // way out.
            let result = nodes_to_register.iter().map(|n| {
                if *n == good_node {
                    &alias_node
                } else {
                    n
                }
            }).cloned().collect();
            Ok(result)
        }
    }
}

struct UnregisterNodesImpl;

impl UnregisterNodes for UnregisterNodesImpl {
    fn unregister_nodes(&mut self, _session: Arc<RwLock<Session>>, _nodes_to_unregister: &[NodeId]) -> Result<(), StatusCode>
    {
        Ok(())
    }
}

#[test]
fn register_nodes() {
    do_view_service_test(|server_state, _, session, _address_space, vs| {
        // Empty request
        let result = vs.register_nodes(server_state, session.clone(), &RegisterNodesRequest {
            request_header: make_request_header(),
            nodes_to_register: None,
        });
        let response: ServiceFault = supported_message_as!(result.unwrap(), ServiceFault);
        assert_eq!(response.response_header.service_result, StatusCode::BadNothingToDo);

        // Invalid request because impl has no registered handler
        let result = vs.register_nodes(server_state, session.clone(), &RegisterNodesRequest {
            request_header: make_request_header(),
            nodes_to_register: Some(vec![
                ObjectId::ObjectsFolder.into()
            ]),
        });
        let response: ServiceFault = supported_message_as!(result.unwrap(), ServiceFault);
        assert_eq!(response.response_header.service_result, StatusCode::BadNodeIdInvalid);

        // Register the callbacks
        server_state.set_register_nodes_callbacks(
            Box::new(RegisterNodesImpl { session: Weak::new() }),
            Box::new(UnregisterNodesImpl {}),
        );

        // Make a good call to register
        let result = vs.register_nodes(server_state, session.clone(), &RegisterNodesRequest {
            request_header: make_request_header(),
            nodes_to_register: Some(vec![
                NodeId::new(1, 99),
                NodeId::new(1, 100),
                NodeId::new(1, 101),
            ]),
        });
        let response: RegisterNodesResponse = supported_message_as!(result.unwrap(), RegisterNodesResponse);
        let registered_node_ids = response.registered_node_ids.unwrap();
        // The middle node should be aliased
        assert_eq!(registered_node_ids[0], NodeId::new(1, 99));
        assert_eq!(registered_node_ids[1], NodeId::new(1, 200));
        assert_eq!(registered_node_ids[2], NodeId::new(1, 101));

        // Make a bad call to register nodes
        let result = vs.register_nodes(server_state, session.clone(), &RegisterNodesRequest {
            request_header: make_request_header(),
            nodes_to_register: Some(vec![
                ObjectId::ObjectsFolder.into()
            ]),
        });
        let response: ServiceFault = supported_message_as!(result.unwrap(), ServiceFault);
        assert_eq!(response.response_header.service_result, StatusCode::BadNodeIdInvalid);
    });
}

#[test]
fn unregister_nodes() {
    do_view_service_test(|server_state, _, session, _address_space, vs| {
        // Empty request
        let result = vs.unregister_nodes(server_state, session.clone(), &UnregisterNodesRequest {
            request_header: make_request_header(),
            nodes_to_unregister: None,
        });
        let response: ServiceFault = supported_message_as!(result.unwrap(), ServiceFault);
        assert_eq!(response.response_header.service_result, StatusCode::BadNothingToDo);

        // Register the callbacks
        server_state.set_register_nodes_callbacks(
            Box::new(RegisterNodesImpl { session: Weak::new() }),
            Box::new(UnregisterNodesImpl {}),
        );

        // Not much to validate except that the function returns good
        let result = vs.unregister_nodes(server_state, session.clone(), &UnregisterNodesRequest {
            request_header: make_request_header(),
            nodes_to_unregister: Some(vec![
                NodeId::new(1, 99),
                ObjectId::ObjectsFolder.into(),
                NodeId::new(1, 100),
                NodeId::new(1, 101),
            ]),
        });
        let response: UnregisterNodesResponse = supported_message_as!(result.unwrap(), UnregisterNodesResponse);
        assert_eq!(response.response_header.service_result, StatusCode::Good);
    });
}