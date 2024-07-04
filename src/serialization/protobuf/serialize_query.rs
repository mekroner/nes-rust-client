use crate::{
    prelude::PlacementStrategy, query::Query, serialization::protobuf::nes::SerializableQueryPlan,
};
use std::collections::HashMap;
use prost::Message;

use super::{nes::SubmitQueryRequest, serialize_operator::*, serialize_sink::serialize_sink_details};

pub fn serialize_query(query: &Query) -> SerializableQueryPlan {
    log::debug!("Serializing query: TODO!");
    let mut id = 0;
    let mut operator_map = HashMap::new();
    traverse_operators(Some(query.operator()), &mut id, &mut operator_map);
    let sink_id = id + 1;
    // serialize_sink
    log::trace! {"Serialize sink: {:?}", query.sink()}
    let serial_sink = SerializableOperatorBuilder::new()
        .details(serialize_sink_details(query.sink()))
        .operator_id(sink_id)
        .add_child_id(0)
        .build();
    operator_map.insert(sink_id, serial_sink);
    log::trace!("Serialized sink with id: {sink_id}, and child_id: 0.",);
    SerializableQueryPlan {
        operator_map,
        root_operator_ids: vec![sink_id],
        query_id: None,
    }
}

pub fn serialize_request(query: &Query, placement: PlacementStrategy) -> Vec<u8> {
    let query_plan = serialize_query(query);
    let placement = prost_types::Any {
        type_url: "type.googleapis.com/google.protobuf.StringValue".to_string(),
        value: placement.to_string().bytes().collect::<Vec<u8>>(),
    };
    let mut context = HashMap::new();
    context.insert("placement".to_string(), placement);
    let request = SubmitQueryRequest {
        query_plan: Some(query_plan),
        context,
        query_string: None,
    };
    request.encode_to_vec()
}
