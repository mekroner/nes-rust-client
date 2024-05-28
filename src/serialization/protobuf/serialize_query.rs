use crate::{query::Query, serialization::protobuf::nes::SerializableQueryPlan};
use std::collections::HashMap;

use super::{
    serialize_operator::{serialize_operator_details, SerializableOperatorBuilder},
    serialize_sink::serialize_sink_details,
};

pub fn serialize(query: Query) -> SerializableQueryPlan {
    let mut operator_map = HashMap::new();
    // serialize operator chain
    let mut last_id = 0;
    for (id, op) in query.operators().enumerate() {
        // fill operators details
        let serial_op_builder = SerializableOperatorBuilder::new()
            .details(serialize_operator_details(op))
            .operator_id(id as u64);
        // add childs id to child_ids filed
        let serial_op = match op.has_child() {
            true => serial_op_builder.add_child_id((id + 1) as u64),
            false => serial_op_builder,
        }
        .build();
        operator_map.insert(id as u64, serial_op);
        last_id = id;
    }

    // serialize_sink
    let sink_id = (last_id + 1) as u64;
    let serial_sink = SerializableOperatorBuilder::new()
        .details(serialize_sink_details(query.sink()))
        .operator_id(sink_id)
        .add_child_id(0)
        .build();
    operator_map.insert(sink_id, serial_sink);

    SerializableQueryPlan {
        operator_map,
        root_operator_ids: vec![sink_id],
        query_id: None,
    }
}
