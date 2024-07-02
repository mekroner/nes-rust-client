use crate::{query::Query, serialization::protobuf::nes::SerializableQueryPlan};
use std::collections::HashMap;

use super::{serialize_operator::*, serialize_sink::serialize_sink_details};

pub fn serialize(query: Query) -> SerializableQueryPlan {
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
    log::trace!(
        "Serialized sink with id: {sink_id}, and child_id: 0.",
    );
    SerializableQueryPlan {
        operator_map,
        root_operator_ids: vec![sink_id],
        query_id: None,
    }
}
