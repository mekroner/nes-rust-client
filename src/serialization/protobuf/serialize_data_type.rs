use prost_types::Any;

use super::nes::{
    serializable_data_type::{FloatDetails, IntegerDetails, Type},
    SerializableDataType,
};
use nes_types::{FloatType, IntType, NesType};

pub fn serialize_data_type(data_type: NesType) -> SerializableDataType {
    let (serial_type, details) = match data_type {
        NesType::Undefined => (Type::Undefined, None),
        NesType::Char => (Type::Char, None),
        NesType::Bool => (Type::Boolean, None),
        NesType::Int(t) => (
            Type::Integer,
            Some(Any::from_msg(&int_detail_helper(t)).unwrap()),
        ),
        NesType::Float(t) => (
            Type::Float,
            Some(Any::from_msg(&float_detail_helper(t)).unwrap()),
        ),
    };
    SerializableDataType {
        r#type: serial_type.into(),
        details,
    }
}

macro_rules! int_details {
    ($type:ty) => {
        IntegerDetails {
            bits: (std::mem::size_of::<$type>() * 8) as u64,
            upper_bound: <$type>::MAX as i64,
            lower_bound: <$type>::MIN as i64,
        }
    };
}

const fn int_detail_helper(int_type: IntType) -> IntegerDetails {
    match int_type {
        IntType::Signed8 => int_details!(i8),
        IntType::Unsigned8 => int_details!(u8),
        IntType::Signed16 => int_details!(i16),
        IntType::Unsigned16 => int_details!(u16),
        IntType::Signed32 => int_details!(i32),
        IntType::Unsigned32 => int_details!(u32),
        IntType::Signed64 => int_details!(i64),
        IntType::Unsigned64 => int_details!(u64),
    }
}

macro_rules! float_details {
    ($type:ty) => {
        FloatDetails {
            bits: (std::mem::size_of::<$type>() * 8) as u64,
            upper_bound: <$type>::MAX as f64,
            lower_bound: <$type>::MIN as f64,
        }
    };
}

const fn float_detail_helper(float_type: FloatType) -> FloatDetails {
    match float_type {
        FloatType::Bit32 => float_details!(f32),
        FloatType::Bit64 => float_details!(u64),
    }
}
