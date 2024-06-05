use crate::query::{
    expression::{expression::RawExpr, field::Field},
    time::TimeCharacteristic,
    window::{
        aggregation::{Aggregation, AggregationType},
        window_descriptor::WindowDescriptor,
    },
};
use prost_types::Any;

use super::{
    nes::{
        serializable_operator::{
            time_characteristic,
            window_details::{aggregation::Type, Aggregation as SerializableAggregation},
            TimeCharacteristic as STimeCharacter, TumblingWindow,
        },
        SerializableExpression,
    },
    serialize_expression::{self, serialize_expression, serialize_field},
};

pub fn serialize_window_descriptor(descriptor: &WindowDescriptor) -> Any {
    match descriptor {
        WindowDescriptor::TumblingWindow {
            duration,
            time_character,
        } => Any::from_msg(&TumblingWindow {
            time_characteristic: Some(serialize_time_characteristic(time_character)),
            size: duration.to_milliseconds(),
        }),
    }
    .unwrap()
}

pub fn serialize_time_characteristic(time_character: &TimeCharacteristic) -> STimeCharacter {
    let TimeCharacteristic::EventTime { field_name, unit } = time_character else {
        unimplemented!();
    };
    STimeCharacter {
        r#type: time_characteristic::Type::EventTime.into(),
        field: field_name.to_string(),
        multiplier: unit.to_scalar().into(),
    }
}

pub fn serialize_aggregations(aggregations: &[Aggregation]) -> Vec<SerializableAggregation> {
    aggregations
        .iter()
        .map(|agg| SerializableAggregation {
            r#type: serialize_aggregation_type(agg.agg_type()).into(),
            on_field: agg.field().map(|f| serialize_field(f)),
            as_field: agg.projected_field().map(|f| serialize_field(f)),
        })
        .collect()
}

const fn serialize_aggregation_type(agg_type: AggregationType) -> Type {
    use AggregationType as AT;
    use Type as T;
    match agg_type {
        AT::Sum => T::Sum,
        AT::Average => T::Avg,
        AT::Min => T::Min,
        AT::Max => T::Max,
        AT::Median => T::Median,
        AT::Count => T::Count,
    }
}

pub fn serialize_window_keys(key_field: &[String]) -> Vec<SerializableExpression> {
    key_field
        .iter()
        .map(|key| serialize_expression(&RawExpr::Field(Field::untyped(key))))
        .collect()
}
