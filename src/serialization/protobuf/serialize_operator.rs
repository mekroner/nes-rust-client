use crate::query::{expression::LogicalExpr, operator::Operator};
use prost_types::Any;

use super::{nes::{
    serializable_operator::{
        source_details::SerializableLogicalSourceDescriptor, FilterDetails, SourceDetails,
    },
    statistic_window_descriptor_message::HyperLogLogDetails,
    SerializableOperator,
}, serialize_expression::serialize_expression};

pub fn serialize_operator_details(operator: &Operator) -> prost_types::Any {
    match operator {
        Operator::LogicalSource { source_name } => {
            Any::from_msg(&logical_source_details(source_name.clone())).unwrap()
        }
        Operator::Filter { expression, .. } => Any::from_msg(&filter_details(expression)).unwrap(),
        Operator::Window { descriptor, .. } => todo!(),
    }
}

pub fn logical_source_details(source_name: String) -> SourceDetails {
    let descriptor = SerializableLogicalSourceDescriptor {
        logical_source_name: source_name,
        ..Default::default()
    };
    let descriptor = Any::from_msg(&descriptor).unwrap();
    SourceDetails {
        source_descriptor: Some(descriptor),
        ..Default::default()
    }
}

pub fn filter_details(expr: &LogicalExpr) -> FilterDetails {
    FilterDetails {
        predicate: Some(serialize_expression(&expr.0)),
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
