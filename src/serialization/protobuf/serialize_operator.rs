use std::collections::HashMap;

use crate::query::{
    join::Join,
    operator::{Filter, Operator, Window},
};
use prost_types::Any;

use super::{
    nes::{
        serializable_operator::{
            source_details::SerializableLogicalSourceDescriptor, FilterDetails, JoinDetails,
            SourceDetails, UnionDetails, WindowDetails,
        },
        SerializableOperator,
    },
    serialize_expression::{serialize_expression, serialize_field},
    serialize_window::{
        serialize_aggregations, serialize_window_descriptor, serialize_window_keys,
    },
};

pub fn serialize_operator(operators: &Operator) -> u64 {
    todo!();
}

pub fn traverse_operators(
    operator: Option<&Operator>,
    id_counter: &mut u64,
    map: &mut HashMap<u64, SerializableOperator>,
) -> Option<u64> {
    let Some(op) = operator else { return None };
    let operator_id = *id_counter;
    *id_counter += 1;
    let mut child_ids = Vec::new();

    let child_id = traverse_operators(op.child(), id_counter, map);
    if let Some(id) = child_id {
        child_ids.push(id);
    }
    let joined_id = match op {
        Operator::Union(u) => traverse_operators(Some(&u.operators), id_counter, map),
        _ => None,
    };
    if let Some(id) = joined_id {
        child_ids.push(id);
    }

    log::trace!(
        "Serialized operator {}, with id {}, and children_ids {:?}",
        op,
        operator_id,
        &child_ids,
    );
    let serial_op = SerializableOperator {
        details: Some(serialize_operator_details(op)),
        operator_id,
        children_ids: child_ids,
        ..Default::default()
    };
    map.insert(operator_id, serial_op);

    Some(operator_id)
}

fn serialize_operator_details(operator: &Operator) -> prost_types::Any {
    match operator {
        Operator::LogicalSource { source_name } => {
            Any::from_msg(&logical_source_details(source_name))
        }
        Operator::Filter(filter) => Any::from_msg(&filter_details(filter)),
        Operator::Window(window) => Any::from_msg(&window_details(window)),
        Operator::Join(join) => Any::from_msg(&join_details(join)),
        Operator::Union(_) => Any::from_msg(&UnionDetails {}),
    }
    .unwrap()
}

fn logical_source_details(source_name: &String) -> SourceDetails {
    let descriptor = SerializableLogicalSourceDescriptor {
        logical_source_name: source_name.to_string(),
        ..Default::default()
    };
    let descriptor = Any::from_msg(&descriptor).unwrap();
    SourceDetails {
        source_descriptor: Some(descriptor),
        ..Default::default()
    }
}

fn filter_details(filter: &Filter) -> FilterDetails {
    FilterDetails {
        predicate: Some(serialize_expression(&filter.expression.0)),
        ..Default::default()
    }
}

fn window_details(window: &Window) -> WindowDetails {
    WindowDetails {
        window_type: Some(serialize_window_descriptor(&window.descriptor)),
        window_aggregations: serialize_aggregations(&window.aggregations),
        keys: window
            .key_fields
            .as_ref()
            .map_or(vec![], |keys| serialize_window_keys(keys)),
        ..Default::default()
    }
}

// use child of query to serialize the other operator chain
fn join_details(join: &Join) -> JoinDetails {
    JoinDetails {
        // FIXME: Join window is a special window because not all windows are supported
        window_type: Some(serialize_window_descriptor(&join.window)),
        number_of_input_edges_left: 1,
        number_of_input_edges_right: 1,
        left_source_type: Some(serialize_field(&join.lhs)),
        right_source_type: Some(serialize_field(&join.rhs)),
        ..Default::default()
    }
}

#[derive(Debug, Default, Clone)]
pub struct SerializableOperatorBuilder {
    details: Option<Any>,
    operator_id: Option<u64>,
    children_ids: Vec<u64>,
}

impl SerializableOperatorBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn details(mut self, details: Any) -> Self {
        self.details = Some(details);
        self
    }

    pub fn operator_id(mut self, id: u64) -> Self {
        self.operator_id = Some(id);
        self
    }

    pub fn add_child_id(mut self, id: u64) -> Self {
        self.children_ids.push(id);
        self
    }

    // FIXME: Add Error Handling
    pub fn build(self) -> SerializableOperator {
        SerializableOperator {
            details: self.details,
            operator_id: self.operator_id.unwrap(),
            children_ids: self.children_ids,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::serialize_operator;
    use crate::prelude::{ExprBuilder as EB, *};
    use std::collections::HashMap;

    #[test]
    fn union_test() {
        let query_sub = QueryBuilder::from_source("test").filter(
            EB::field("value")
                .greater_than(EB::literal(0))
                .not()
                .build_logical()
                .unwrap(),
        );

        let query = QueryBuilder::from_source("test")
            .filter(
                EB::field("value")
                    .greater_than(EB::literal(0))
                    .build_logical()
                    .unwrap(),
            )
            .union(query_sub)
            .sink(Sink::File {
                path: "./generated_files/result-1.csv".into(),
                format: "CSV_FORMAT".into(),
                append: false,
            });
        let mut id = 0;
        let mut operator_map = HashMap::new();
        serialize_operator(query.operator(), &mut id, &mut operator_map);
        assert_eq!(5, id);
        assert_eq!(6, id);
        // assert!(operator_map[0]., )
    }
}
