use crate::query::{
    operator::Operator,
    window::{aggregation::{Aggregation, AggregationType}, window_descriptor::WindowDescriptor},
};
use prost_types::Any;

use super::nes::{
    serializable_operator::{
        source_details::SerializableLogicalSourceDescriptor,
        window_details::{self, aggregation},
        SourceDetails, WindowDetails,
    },
    SerializableOperator,
};

pub fn serialize_operator_details(operator: &Operator) -> prost_types::Any {
    match operator {
        Operator::LogicalSource { source_name } => {
            Any::from_msg(&logical_source_details(source_name)).unwrap()
        }
        Operator::Filter { expression, .. } => todo!(),
        Operator::Window {
            descriptor,
            aggregations,
            key_fields,
            ..
        } => Any::from_msg(&window_details(descriptor, aggregations, key_fields)).unwrap(),
    }
}

pub fn logical_source_details(source_name: &String) -> SourceDetails {
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

pub fn window_details(
    descriptor: &WindowDescriptor,
    aggregations: &[Aggregation],
    key_field: &[String],
) -> WindowDetails {
    WindowDetails {
        window_type: todo!(),
        window_aggregations: serialize_aggregations(aggregations),
        distr_char: todo!(),
        keys: todo!(),
        allowed_lateness: todo!(),
        origin: todo!(),
    }
}

pub fn serialize_aggregations(aggregations: &[Aggregation]) -> Vec<window_details::Aggregation> {
    aggregations
        .iter()
        .map(|agg| window_details::Aggregation {
            r#type: serialize_aggregation_type(agg.agg_type).into(),
            on_field: todo!(),
            as_field: None,
        })
        .collect()
}

pub const fn serialize_aggregation_type(agg_type: AggregationType) -> window_details::aggregation::Type {
    use window_details::aggregation::Type as T;
    use AggregationType as AT;
    match agg_type {
        AT::Sum => T::Sum,
        AT::Average => T::Avg,
        AT::Min => T::Min,
        AT::Max => T::Max,
        AT::Median => T::Median,
        AT::Count => T::Count,
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
