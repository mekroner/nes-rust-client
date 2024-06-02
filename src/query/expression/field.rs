use super::expression::NESType;

#[derive(Debug)]
pub struct Field {
    name: String,
    data_type: NESType,
}

impl Field {

    pub fn untyped(name: impl Into<String>) -> Self{
        Self {
            name: name.into(),
            data_type: NESType::Undefined,
        }
    }

    pub fn typed(name: impl Into<String>, data_type: NESType) -> Self{
        Self {
            name: name.into(),
            data_type,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> NESType {
        self.data_type
    }
}


