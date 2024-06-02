use crate::query::{
    expression::{expression::Expr, field::Field, LogicalExpr},
    operator::{Filter, Operator},
    window::{aggregation::Aggregation, window_descriptor::WindowDescriptor},
};
use prost_types::Any;

use super::{
    nes::{
        serializable_operator::{
            source_details::SerializableLogicalSourceDescriptor, FilterDetails, SourceDetails,
            WindowDetails,
        },
        SerializableOperator,
    }, serialize_expression::serialize_expression, serialize_window::{serialize_aggregations, serialize_window_descriptor, serialize_window_keys}
};

pub fn serialize_operator_details(operator: &Operator) -> prost_types::Any {
    match operator {
        Operator::LogicalSource { source_name } => {
            Any::from_msg(&logical_source_details(source_name)).unwrap()
        }
        Operator::Filter(Filter { expression, .. }) => {
            Any::from_msg(&filter_details(expression)).unwrap()
        }
        Operator::Window {
            descriptor,
            aggregations,
            key_fields,
            ..
        } => Any::from_msg(&window_details(
            descriptor,
            aggregations,
            key_fields.as_deref(),
        ))
        .unwrap(),
    }
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

fn filter_details(expr: &LogicalExpr) -> FilterDetails {
    FilterDetails {
        predicate: Some(serialize_expression(&expr.0)),
        ..Default::default()
    }
}

fn window_details(
    descriptor: &WindowDescriptor,
    aggregations: &[Aggregation],
    key_fields: Option<&[String]>,
) -> WindowDetails {
    WindowDetails {
        window_type: Some(serialize_window_descriptor(descriptor)),
        window_aggregations: serialize_aggregations(aggregations),
        keys: key_fields.map_or(vec![], |keys| serialize_window_keys(keys)),
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
