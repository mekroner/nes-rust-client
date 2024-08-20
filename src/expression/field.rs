use nes_types::NesType;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Field {
    name: String,
    projected_name: Option<String>,
    data_type: NesType,
}

impl Field {
    pub fn untyped(name: impl Into<String>) -> Self{
        Self {
            name: name.into(),
            projected_name: None,
            data_type: NesType::Undefined,
        }
    }

    pub fn typed(name: impl Into<String>, data_type: NesType) -> Self{
        Self {
            name: name.into(),
            projected_name: None,
            data_type,
        }
    }

    pub fn rename(mut self, new_name: impl Into<String>) -> Self {
        self.projected_name = Some(new_name.into());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn projected_name(&self) -> Option<&str> {
        self.projected_name.as_deref()
    }

    pub fn data_type(&self) -> NesType {
        self.data_type
    }
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        Field::untyped(value)
    }
}

impl From<String> for Field {
    fn from(value: String) -> Self {
        Field::untyped(value)
    }
}
