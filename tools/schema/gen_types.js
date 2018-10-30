var _ = require("lodash");
var fs = require("fs");
var xml2js = require("xml2js");

var settings = require("./settings");

var types_xml = `${settings.schema_dir}/Opc.Ua.Types.bsd.xml`;

/// This code parses the OPC UA Binary types definitions and creates a generated .rs type.
/// Fields are converted to snake case as they are written. Code for serializing the struct is also generated

/// Any handwritten types are stripped from the output

var ignored_types = [
    // Handwritten so not autogenerated
    "ExtensionObject", "DataValue", "LocalizedText", "QualifiedName", "DiagnosticInfo", "Variant",
    "ExpandedNodeId", "NodeId", "ByteStringNodeId", "GuidNodeId", "StringNodeId", "NumericNodeId",
    "FourByteNodeId", "TwoByteNodeId", "XmlElement", "Union", "RequestHeader", "ResponseHeader",
    "Node", "InstanceNode", "TypeNode", "ObjectNode", "ObjectTypeNode", "VariableNode", "VariableTypeNode", "ReferenceTypeNode",
    "MethodNode", "ViewNode", "DataTypeNode", "ReferenceNode",
    // Excluded because they use unimplemented enums, or are used by unimplemented services
    "ModificationInfo", "HistoryModifiedData", "UpdateDataDetails", "UpdateEventDetails", "UpdateStructureDataDetails", "RedundantServerDataType",
    "ServerStatusDataType", "AxisInformation", "RegisterServer2Request", "RegisterServer2Response", "HistoryData", "HistoryEvent", "HistoryReadDetails",
    "HistoryEventFieldList", "HistoryReadRequest", "HistoryReadResponse", "HistoryReadResult", "HistoryReadValueId", "HistoryUpdateDetails",
    "HistoryUpdateRequest", "HistoryUpdateResponse", "HistoryUpdateResult", "SemanticChangeStructureDataType", "SemanticChangeStructureDataType",
    "ReadAtTimeDetails", "ReadProcessedDetails"
];

var basic_types_import_map = {
    // "basic_types": ["Boolean", "Int32", "UInt32", "Double", "Float", "Int16", "UInt16", "Byte", "SByte"],
    "string": ["UAString", "XmlElement"],
    "byte_string": ["ByteString"],
    "variant": ["Variant"],
    "basic_types": ["LocalizedText", "QualifiedName"],
    "diagnostic_info": ["DiagnosticInfo"],
    "extension_object": ["ExtensionObject"],
    "data_types": ["MessageSecurityMode", "Duration", "UtcTime", "MonitoringMode"],
    "service_types::impls": ["RequestHeader", "ResponseHeader"],
    "service_types::enums": ["TimestampsToReturn", "FilterOperator", "BrowseDirection", "NodeClass", "SecurityTokenRequestType", "ApplicationType", "UserTokenType", "DataChangeTrigger"],
    "node_id": ["NodeId", "ExpandedNodeId"],
    "data_value": ["DataValue"],
    "date_time": ["DateTime"],
    "status_codes": ["StatusCode"]
};

var serde_supported_types = ["ReadValueId", "DataChangeFilter", "MonitoredItemNotification", "ServerDiagnosticsSummaryDataType"];

// Contains a flattened reverse lookup of the import map
var basic_types_reverse_import_map = {}
_.each(basic_types_import_map, function (types, module) {
    _.each(types, function (type) {
        basic_types_reverse_import_map[type] = module;
    })
})


var type_name_mappings = {
    "String": "UAString",
    "Boolean": "bool",
    "SByte": "i8",
    "Byte": "u8",
    "Int16": "i16",
    "UInt16": "u16",
    "Int32": "i32",
    "UInt32": "u32",
    "Int64": "i64",
    "UInt64": "u64",
    "Float": "f32",
    "Double": "f64"
}

function massageTypeName(name) {
    if (_.has(type_name_mappings, name)) {
        return type_name_mappings[name];
    }
    else {
        return name;
    }
}

function convertFieldName(name) {
    // Convert field name to snake case
    return _.snakeCase(name);
}

var parser = new xml2js.Parser();
fs.readFile(types_xml, function (err, data) {
    parser.parseString(data, function (err, result) {
        var data = {
            structured_types: []
        };

        var structured_types = result["opc:TypeDictionary"]["opc:StructuredType"];
        _.each(structured_types, function (structured_type_element) {

            var name = structured_type_element["$"]["Name"];
            // if name in ignored_types, do nothing
            if (!_.includes(ignored_types, name)) {
                var fields_to_add = [];
                var fields_to_hide = [];
                _.each(structured_type_element["opc:Field"], function (field) {
                    // Convert field name to snake case
                    var field_name = convertFieldName(field["$"]["Name"]);

                    // Strip namespace off the type
                    var type = massageTypeName(field["$"]["TypeName"].split(":")[1]);

                    // Look for arrays
                    if (_.has(field["$"], "LengthField")) {
                        fields_to_add.push({
                            name: field_name,
                            type: `Option<Vec<${type}>>`,
                            contained_type: type,
                            inner_type: type,
                            is_array: true
                        });
                        fields_to_hide.push(convertFieldName(field["$"]["LengthField"]));
                    }
                    else {
                        fields_to_add.push({
                            name: field_name,
                            type: type,
                            contained_type: type
                        })
                    }
                });

                var structured_type = {
                    name: name,
                    fields_to_add: fields_to_add,
                    fields_to_hide: fields_to_hide
                };
                if (_.has(structured_type_element, "opc:Documentation")) {
                    structured_type.documentation = structured_type_element["opc:Documentation"];
                }
                if (_.has(structured_type_element["$"], "BaseType")) {
                    structured_type.base_type = structured_type_element["$"]["BaseType"];
                }
                data.structured_types.push(structured_type)
            }

        });
        generate_types(data);
    });
});

function generate_types(data) {
    // Output structured types
    _.each(data.structured_types, function (structured_type) {
        generate_structured_type_file(data.structured_types, structured_type);
    });
    generate_types_mod(data.structured_types);
}

function generate_types_mod(structured_types) {
    var file_name = "mod.rs";
    var file_path = `${settings.rs_types_dir}/${file_name}`;

    var contents = `// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

// The mods below are handwritten
mod enums;
mod impls;

pub use self::enums::*;
pub use self::impls::*;

// All of the remaining are generated by script

`;
    _.each(structured_types, function (structured_type) {
        var mod_name = _.snakeCase(structured_type.name);
        contents += `mod ${mod_name};
`
    });

    contents += "\n";

    _.each(structured_types, function (structured_type) {
        var mod_name = _.snakeCase(structured_type.name);
        contents += `pub use self::${mod_name}::*;
`
    });

    settings.write_to_file(file_path, contents);
}

function generate_type_imports(structured_types, fields_to_add, fields_to_hide, has_message_info) {
    var imports = `use crate::encoding::*;
#[allow(unused_imports)]
use crate::basic_types::*;
`;

    if (has_message_info) {
        imports += `use crate::service_types::impls::MessageInfo;
use crate::node_ids::ObjectId;
`;
    }

    // Basic types are any which are hand written
    var basic_types_to_import = {}

    // Service types are other generated types
    var service_types_used = {};

    // Make a set of the types that need to be imported. Referenced types are either handwritten or
    // other generated files so according to which they are, we build up a couple of tables.
    _.each(fields_to_add, function (field) {
        if (!_.includes(fields_to_hide, field.name)) {
            var type = _.find(structured_types, {name: field.contained_type});
            if (type) {
                // Machine generated type
                service_types_used[type.name] = type.name;
            }
            else if (_.has(basic_types_reverse_import_map, field.contained_type)) {
                // Handwritten type - use module lookup to figure where its implemented
                var type = massageTypeName(field.contained_type);
                var module = basic_types_reverse_import_map[field.contained_type];
                if (!_.has(basic_types_to_import, module)) {
                    basic_types_to_import[module] = {};
                }
                basic_types_to_import[module][type] = type;
            }
        }
    });

    // Hand written imports
    var basic_type_imports = "";
    _.each(basic_types_to_import, function (types, module) {
        _.each(types, function (type) {
            basic_type_imports += `use crate::${module}::${type};
`
        });
    });
    imports += basic_type_imports;

    // Service type imports
    var service_type_imports = "";
    _.each(service_types_used, function (value, key) {
        service_type_imports += `use crate::service_types::${key};
`;
    });
    imports += service_type_imports;

    return imports;
}

function generate_structured_type_file(structured_types, structured_type) {
    var file_name = _.snakeCase(structured_type.name) + ".rs";
    var file_path = `${settings.rs_types_dir}/${file_name}`;

    var has_message_info = _.has(structured_type, "base_type") && structured_type.base_type === "ua:ExtensionObject";

    console.log("Creating structured type file - " + file_path);

    var contents = `// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

`;
    contents += generate_type_imports(structured_types, structured_type.fields_to_add, structured_type.fields_to_hide, has_message_info);
    contents += "\n";

    if (_.has(structured_type, "documentation")) {
        contents += `/// ${structured_type.documentation}\n`;
    }

    var derivations = "Debug, Clone, PartialEq";
    if (_.includes(serde_supported_types, structured_type.name)) {
        derivations += ", Serialize";
    }

    contents += `#[derive(${derivations})]
pub struct ${structured_type.name} {
`;

    _.each(structured_type.fields_to_add, function (field) {
        if (!_.includes(structured_type.fields_to_hide, field.name)) {
            contents += `    pub ${field.name}: ${field.type},\n`;
        }
    });
    contents += `}

`;

    if (has_message_info) {
        contents += `impl MessageInfo for ${structured_type.name} {
    fn object_id(&self) -> ObjectId {
        ObjectId::${structured_type.name}_Encoding_DefaultBinary
    }
}

`;
    }

    contents += `impl BinaryEncoder<${structured_type.name}> for ${structured_type.name} {
    fn byte_len(&self) -> usize {
`;
    if (structured_type.fields_to_add.length > 0) {
        contents += `        let mut size = 0;\n`;

        _.each(structured_type.fields_to_add, function (field) {
            if (!_.includes(structured_type.fields_to_hide, field.name)) {
                if (_.has(field, 'is_array')) {
                    contents += `        size += byte_len_array(&self.${field.name});\n`;
                }
                else {
                    contents += `        size += self.${field.name}.byte_len();\n`;
                }
            }
        });

        contents += `        size\n`;
    }
    else {
        contents += `        0\n`;
    }

    contents += `    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
`;

    if (structured_type.fields_to_add.length > 0) {
        contents += `        let mut size = 0;\n`;

        _.each(structured_type.fields_to_add, function (field) {
            if (!_.includes(structured_type.fields_to_hide, field.name)) {
                if (_.has(field, 'is_array')) {
                    contents += `        size += write_array(stream, &self.${field.name})?;\n`;
                }
                else {
                    contents += `        size += self.${field.name}.encode(stream)?;\n`;
                }
            }
        });

        contents += `        Ok(size)\n`;
    }
    else {
        contents += `        Ok(0)\n`;
    }

    contents += `    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
`;

    _.each(structured_type.fields_to_add, function (field) {
        if (!_.includes(structured_type.fields_to_hide, field.name)) {
            if (_.has(field, 'is_array')) {
                contents += `        let ${field.name}: ${field.type} = read_array(stream, decoding_limits)?;\n`;
            }
            else {
                contents += `        let ${field.name} = ${field.type}::decode(stream, decoding_limits)?;\n`;
            }
        }
    });

    contents += `        Ok(${structured_type.name} {
`;

    _.each(structured_type.fields_to_add, function (field) {
        if (!_.includes(structured_type.fields_to_hide, field.name)) {
            contents += `            ${field.name},\n`;
        }
    });

    contents += `        })
    }
}
`;

    settings.write_to_file(file_path, contents);
}