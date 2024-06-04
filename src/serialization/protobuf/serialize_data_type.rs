use prost_types::Any;

use crate::query::expression::expression::NESType;

use super::nes::{
    serializable_data_type::{FloatDetails, IntegerDetails, Type},
    SerializableDataType,
};

pub fn serialize_data_type(data_type: NESType) -> SerializableDataType {
    let (serial_type, details) = match data_type {
        NESType::Undefined => (Type::Undefined, None),
        NESType::Char => (Type::Char, None),
        NESType::Bool => (Type::Boolean, None),
        NESType::Int32 => (
            Type::Integer,
            Some(Any::from_msg(&int32_details()).unwrap()),
        ),
        NESType::Int64 => (
            Type::Integer,
            Some(Any::from_msg(&int64_details()).unwrap()),
        ),
        NESType::Float32 => (Type::Float, 
            Some(Any::from_msg(&float32_details()).unwrap()),
        ),
        NESType::Float64 => (Type::Float, 
            Some(Any::from_msg(&float64_details()).unwrap()),
        ),
    };
    SerializableDataType {
        r#type: serial_type.into(),
        details,
    }
}

const fn int32_details() -> IntegerDetails {
    IntegerDetails {
        bits: 32,
        upper_bound: i32::MAX as i64,
        lower_bound: i32::MIN as i64,
    }
}

const fn int64_details() -> IntegerDetails {
    IntegerDetails {
        bits: 64,
        upper_bound: i64::MAX,
        lower_bound: i64::MIN,
    }
}

const fn float32_details() -> FloatDetails {
    FloatDetails {
        bits: 32,
        upper_bound: f32::MAX as f64,
        lower_bound: f32::MIN as f64,
    }
}

const fn float64_details() -> FloatDetails {
    FloatDetails {
        bits: 64,
        upper_bound: f64::MAX,
        lower_bound: f64::MIN,
    }
}
