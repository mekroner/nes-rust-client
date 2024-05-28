use crate::{
    query::Query,
    serialization::protobuf::nes::{SerializableQueryPlan, SubmitQueryRequest},
};
use prost::Message;
use std::collections::HashMap;

use super::{
    serialize_operator::{serialize_operator_details, SerializableOperatorBuilder},
    serialize_sink::serialize_sink_details,
};

pub async fn execute_query(
    query: Query,
    placement: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let query_plan = serialize(query);
    let placement = prost_types::Any {
        type_url: "type.googleapis.com/google.protobuf.StringValue".to_string(),
        value: placement.bytes().collect::<Vec<u8>>(),
    };
    let mut context = HashMap::new();
    context.insert("placement".to_string(), placement);
    let request = SubmitQueryRequest {
        query_plan: Some(query_plan),
        context,
        query_string: None,
    };
    let client = reqwest::Client::builder().build().unwrap();
    let response = client
        .post("http://127.0.0.1:8081/v1/nes/query/execute-query-ex")
        .body(request.encode_to_vec())
        .send()
        .await?;

    dbg!(response);
    Ok(())
}

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

    dbg!(&operator_map);

    SerializableQueryPlan {
        operator_map,
        root_operator_ids: vec![sink_id],
        query_id: None,
    }
}
