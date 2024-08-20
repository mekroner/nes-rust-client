use nes_types::NesType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Literal {
    value: String,
    data_type: NesType,
}

impl Literal {
    pub fn typed(value: impl Into<String>, data_type: NesType) -> Self {
        Self {
            value: value.into(),
            data_type,
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn data_type(&self) -> NesType {
        self.data_type
    }
}

impl Into<Literal> for i32 {
    fn into(self) -> Literal {
        Literal {
            value: self.to_string(),
            data_type: NesType::Int(nes_types::IntType::Signed32),
        }
    }
}

impl Into<Literal> for i64 {
    fn into(self) -> Literal {
        Literal {
            value: self.to_string(),
            data_type: NesType::Int(nes_types::IntType::Signed64),
        }
    }
}

impl Into<Literal> for bool {
    fn into(self) -> Literal {
        Literal {
            value: self.to_string(),
            data_type: NesType::Bool,
        }
    }
}
