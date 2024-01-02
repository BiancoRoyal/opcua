// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock
// This file was autogenerated from Opc.Ua.NodeSet2.Part5.xml by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::{convert::TryFrom, str::FromStr};

#[allow(unused_imports)]
use crate::{
    server::address_space::{types::*, EventNotifier},
    types::{
        service_types::Argument, DataTypeId, ExtensionObject, LocalizedText, NodeId,
        ReferenceTypeId, UAString, Variant, VariantTypeId,
    },
};

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    add_method_1(address_space);
    add_method_2(address_space);
    add_method_3(address_space);
    add_method_4(address_space);
    add_method_5(address_space);
    add_method_6(address_space);
    add_method_7(address_space);
    add_method_8(address_space);
    add_method_9(address_space);
    add_method_10(address_space);
    add_method_11(address_space);
    add_method_12(address_space);
    add_method_13(address_space);
    add_method_14(address_space);
    add_method_15(address_space);
    add_method_16(address_space);
    add_method_17(address_space);
    add_method_18(address_space);
    add_method_19(address_space);
    add_method_20(address_space);
    add_method_21(address_space);
    add_method_22(address_space);
    add_method_23(address_space);
    add_method_24(address_space);
    add_method_25(address_space);
    add_method_26(address_space);
    add_method_27(address_space);
    add_method_28(address_space);
    add_method_29(address_space);
    add_method_30(address_space);
    add_method_31(address_space);
    add_method_32(address_space);
}

fn add_method_1(address_space: &mut AddressSpace) {
    // Method
    let name = "AddEndpoint";
    let node_id = NodeId::new(0, 16221);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16222),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15668),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_2(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveEndpoint";
    let node_id = NodeId::new(0, 16223);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16224),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15668),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_3(address_space: &mut AddressSpace) {
    // Method
    let name = "AddIdentity";
    let node_id = NodeId::new(0, 15684);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 15685),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15680),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_4(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveIdentity";
    let node_id = NodeId::new(0, 15686);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 15687),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15680),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_5(address_space: &mut AddressSpace) {
    // Method
    let name = "AddApplication";
    let node_id = NodeId::new(0, 16228);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16229),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15680),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_6(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveApplication";
    let node_id = NodeId::new(0, 16230);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16231),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15680),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_7(address_space: &mut AddressSpace) {
    // Method
    let name = "AddEndpoint";
    let node_id = NodeId::new(0, 16232);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16233),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15680),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_8(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveEndpoint";
    let node_id = NodeId::new(0, 16234);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16235),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15680),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_9(address_space: &mut AddressSpace) {
    // Method
    let name = "AddIdentity";
    let node_id = NodeId::new(0, 16041);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16042),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 16036),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_10(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveIdentity";
    let node_id = NodeId::new(0, 16043);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16044),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 16036),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_11(address_space: &mut AddressSpace) {
    // Method
    let name = "AddApplication";
    let node_id = NodeId::new(0, 16239);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16240),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 16036),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_12(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveApplication";
    let node_id = NodeId::new(0, 16241);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16242),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 16036),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_13(address_space: &mut AddressSpace) {
    // Method
    let name = "AddEndpoint";
    let node_id = NodeId::new(0, 16243);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16244),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 16036),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_14(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveEndpoint";
    let node_id = NodeId::new(0, 16245);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16246),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 16036),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_15(address_space: &mut AddressSpace) {
    // Method
    let name = "AddIdentity";
    let node_id = NodeId::new(0, 15696);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 15697),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15692),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_16(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveIdentity";
    let node_id = NodeId::new(0, 15698);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 15699),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15692),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_17(address_space: &mut AddressSpace) {
    // Method
    let name = "AddApplication";
    let node_id = NodeId::new(0, 16250);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16251),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15692),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_18(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveApplication";
    let node_id = NodeId::new(0, 16252);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16253),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15692),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_19(address_space: &mut AddressSpace) {
    // Method
    let name = "AddEndpoint";
    let node_id = NodeId::new(0, 16254);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16255),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15692),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_20(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveEndpoint";
    let node_id = NodeId::new(0, 16256);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16257),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15692),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_21(address_space: &mut AddressSpace) {
    // Method
    let name = "AddIdentity";
    let node_id = NodeId::new(0, 15720);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 15721),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15716),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_22(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveIdentity";
    let node_id = NodeId::new(0, 15722);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 15723),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15716),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_23(address_space: &mut AddressSpace) {
    // Method
    let name = "AddApplication";
    let node_id = NodeId::new(0, 16272);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16273),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15716),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_24(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveApplication";
    let node_id = NodeId::new(0, 16274);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16275),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15716),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_25(address_space: &mut AddressSpace) {
    // Method
    let name = "AddEndpoint";
    let node_id = NodeId::new(0, 16276);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16277),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15716),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_26(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveEndpoint";
    let node_id = NodeId::new(0, 16278);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16279),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15716),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_27(address_space: &mut AddressSpace) {
    // Method
    let name = "AddIdentity";
    let node_id = NodeId::new(0, 15708);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 15709),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15704),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_28(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveIdentity";
    let node_id = NodeId::new(0, 15710);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 15711),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15704),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_29(address_space: &mut AddressSpace) {
    // Method
    let name = "AddApplication";
    let node_id = NodeId::new(0, 16261);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16262),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15704),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_30(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveApplication";
    let node_id = NodeId::new(0, 16263);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16264),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15704),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_31(address_space: &mut AddressSpace) {
    // Method
    let name = "AddEndpoint";
    let node_id = NodeId::new(0, 16265);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16266),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15704),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}

fn add_method_32(address_space: &mut AddressSpace) {
    // Method
    let name = "RemoveEndpoint";
    let node_id = NodeId::new(0, 16267);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 16268),
                &ReferenceTypeId::HasProperty,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 15704),
                &ReferenceTypeId::HasComponent,
                ReferenceDirection::Inverse,
            ),
        ]),
    );
}
