// This file was autogenerated from Opc.Ua.NodeSet2.Part13.xml by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused_imports)]
use opcua_types::*;
use opcua_types::node_ids::*;
#[allow(unused_imports)]
use crate::address_space::types::*;

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    {
        // Object
        let browse_name = "Interpolative";
        let display_name = "Interpolative";
        let description = "At the beginning of each interval, retrieve the calculated value from the data points on either side of the requested timestamp.";
        let node_id = NodeId::new(0, 2341);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Average";
        let display_name = "Average";
        let description = "Retrieve the average value of the data over the interval.";
        let node_id = NodeId::new(0, 2342);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "TimeAverage";
        let display_name = "TimeAverage";
        let description = "Retrieve the time weighted average data over the interval using Interpolated Bounding Values.";
        let node_id = NodeId::new(0, 2343);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "TimeAverage2";
        let display_name = "TimeAverage2";
        let description = "Retrieve the time weighted average data over the interval using Simple Bounding Values.";
        let node_id = NodeId::new(0, 11285);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Total";
        let display_name = "Total";
        let description = "Retrieve the total (time integral) of the data over the interval using Interpolated Bounding Values.";
        let node_id = NodeId::new(0, 2344);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Total2";
        let display_name = "Total2";
        let description = "Retrieve the total (time integral) of the data over the interval using Simple Bounding Values.";
        let node_id = NodeId::new(0, 11304);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Minimum";
        let display_name = "Minimum";
        let description = "Retrieve the minimum raw value in the interval with the timestamp of the start of the interval.";
        let node_id = NodeId::new(0, 2346);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Maximum";
        let display_name = "Maximum";
        let description = "Retrieve the maximum raw value in the interval with the timestamp of the start of the interval.";
        let node_id = NodeId::new(0, 2347);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "MinimumActualTime";
        let display_name = "MinimumActualTime";
        let description = "Retrieve the minimum value in the interval and the Timestamp of the minimum value.";
        let node_id = NodeId::new(0, 2348);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "MaximumActualTime";
        let display_name = "MaximumActualTime";
        let description = "Retrieve the maximum value in the interval and the Timestamp of the maximum value.";
        let node_id = NodeId::new(0, 2349);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Range";
        let display_name = "Range";
        let description = "Retrieve the difference between the minimum and maximum Value over the interval.";
        let node_id = NodeId::new(0, 2350);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Minimum2";
        let display_name = "Minimum2";
        let description = "Retrieve the minimum value in the interval including the Simple Bounding Values.";
        let node_id = NodeId::new(0, 11286);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Maximum2";
        let display_name = "Maximum2";
        let description = "Retrieve the maximum value in the interval including the Simple Bounding Values.";
        let node_id = NodeId::new(0, 11287);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "MinimumActualTime2";
        let display_name = "MinimumActualTime2";
        let description = "Retrieve the minimum value with the actual timestamp including the Simple Bounding Values.";
        let node_id = NodeId::new(0, 11305);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "MaximumActualTime2";
        let display_name = "MaximumActualTime2";
        let description = "Retrieve the maximum value with the actual timestamp including the Simple Bounding Values.";
        let node_id = NodeId::new(0, 11306);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Range2";
        let display_name = "Range2";
        let description = "Retrieve the difference between the Minimum2 and Maximum2 value over the interval.";
        let node_id = NodeId::new(0, 11288);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "AnnotationCount";
        let display_name = "AnnotationCount";
        let description = "Retrieve the number of Annotations in the interval.";
        let node_id = NodeId::new(0, 2351);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Count";
        let display_name = "Count";
        let description = "Retrieve the number of raw values over the interval.";
        let node_id = NodeId::new(0, 2352);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "DurationInStateZero";
        let display_name = "DurationInStateZero";
        let description = "Retrieve the time a Boolean or numeric was in a zero state using Simple Bounding Values.";
        let node_id = NodeId::new(0, 11307);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "DurationInStateNonZero";
        let display_name = "DurationInStateNonZero";
        let description = "Retrieve the time a Boolean or numeric was in a non-zero state using Simple Bounding Values.";
        let node_id = NodeId::new(0, 11308);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "NumberOfTransitions";
        let display_name = "NumberOfTransitions";
        let description = "Retrieve the number of changes between zero and non-zero that a Boolean or Numeric value experienced in the interval.";
        let node_id = NodeId::new(0, 2355);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Start";
        let display_name = "Start";
        let description = "Retrieve the value at the beginning of the interval using Interpolated Bounding Values.";
        let node_id = NodeId::new(0, 2357);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "End";
        let display_name = "End";
        let description = "Retrieve the value at the end of the interval using Interpolated Bounding Values.";
        let node_id = NodeId::new(0, 2358);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "Delta";
        let display_name = "Delta";
        let description = "Retrieve the difference between the Start and End value in the interval.";
        let node_id = NodeId::new(0, 2359);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "StartBound";
        let display_name = "StartBound";
        let description = "Retrieve the value at the beginning of the interval using Simple Bounding Values.";
        let node_id = NodeId::new(0, 11505);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "EndBound";
        let display_name = "EndBound";
        let description = "Retrieve the value at the end of the interval using Simple Bounding Values.";
        let node_id = NodeId::new(0, 11506);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "DeltaBounds";
        let display_name = "DeltaBounds";
        let description = "Retrieve the difference between the StartBound and EndBound value in the interval.";
        let node_id = NodeId::new(0, 11507);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "DurationGood";
        let display_name = "DurationGood";
        let description = "Retrieve the total duration of time in the interval during which the data is good.";
        let node_id = NodeId::new(0, 2360);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "DurationBad";
        let display_name = "DurationBad";
        let description = "Retrieve the total duration of time in the interval during which the data is bad.";
        let node_id = NodeId::new(0, 2361);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "PercentGood";
        let display_name = "PercentGood";
        let description = "Retrieve the percent of data (0 to 100) in the interval which has a good StatusCode.";
        let node_id = NodeId::new(0, 2362);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "PercentBad";
        let display_name = "PercentBad";
        let description = "Retrieve the percent of data (0 to 100) in the interval which has a bad StatusCode.";
        let node_id = NodeId::new(0, 2363);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "WorstQuality";
        let display_name = "WorstQuality";
        let description = "Retrieve the worst StatusCode of data in the interval.";
        let node_id = NodeId::new(0, 2364);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "WorstQuality2";
        let display_name = "WorstQuality2";
        let description = "Retrieve the worst StatusCode of data in the interval including the Simple Bounding Values.";
        let node_id = NodeId::new(0, 11292);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "StandardDeviationSample";
        let display_name = "StandardDeviationSample";
        let description = "Retrieve the standard deviation for the interval for a sample of the population (n-1).";
        let node_id = NodeId::new(0, 11426);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "StandardDeviationPopulation";
        let display_name = "StandardDeviationPopulation";
        let description = "Retrieve the standard deviation for the interval for a complete population (n) which includes Simple Bounding Values.";
        let node_id = NodeId::new(0, 11427);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "VarianceSample";
        let display_name = "VarianceSample";
        let description = "Retrieve the variance for the interval as calculated by the StandardDeviationSample.";
        let node_id = NodeId::new(0, 11428);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // Object
        let browse_name = "VariancePopulation";
        let display_name = "VariancePopulation";
        let description = "Retrieve the variance for the interval as calculated by the StandardDeviationPopulation which includes Simple Bounding Values.";
        let node_id = NodeId::new(0, 11429);
        let node = Object::new(&node_id, browse_name, display_name, description);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 2340), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        ]));
    }
    {
        // ObjectType
        let browse_name = "AggregateConfigurationType";
        let display_name = "AggregateConfigurationType";
        let description = "";
        let node_id = NodeId::new(0, 11187);
        let node = ObjectType::new(&node_id, browse_name, display_name, description, false);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 11188), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
            (&NodeId::new(0, 11189), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
            (&NodeId::new(0, 11190), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
            (&NodeId::new(0, 11191), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
            (&NodeId::new(0, 58), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
        ]));
    }
    {
        // Variable
        let data_value = DataValue::null();
        let browse_name = "TreatUncertainAsBad";
        let display_name = "TreatUncertainAsBad";
        let description = "";
        let node_id = NodeId::new(0, 11188);
        let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::Boolean, data_value);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 11187), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
            (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
            (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
            (&NodeId::new(0, 11187), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
        ]));
    }
    {
        // Variable
        let data_value = DataValue::null();
        let browse_name = "PercentDataBad";
        let display_name = "PercentDataBad";
        let description = "";
        let node_id = NodeId::new(0, 11189);
        let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::Byte, data_value);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 11187), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
            (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
            (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
            (&NodeId::new(0, 11187), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
        ]));
    }
    {
        // Variable
        let data_value = DataValue::null();
        let browse_name = "PercentDataGood";
        let display_name = "PercentDataGood";
        let description = "";
        let node_id = NodeId::new(0, 11190);
        let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::Byte, data_value);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 11187), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
            (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
            (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
            (&NodeId::new(0, 11187), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
        ]));
    }
    {
        // Variable
        let data_value = DataValue::null();
        let browse_name = "UseSlopedExtrapolation";
        let display_name = "UseSlopedExtrapolation";
        let description = "";
        let node_id = NodeId::new(0, 11191);
        let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::Boolean, data_value);
        address_space.insert(node, Some(&[
            (&NodeId::new(0, 11187), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
            (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
            (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
            (&NodeId::new(0, 11187), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
        ]));
    }
}
