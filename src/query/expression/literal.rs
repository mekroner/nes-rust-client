use super::expression::NESType;

#[derive(Debug, PartialEq)]
pub struct Literal {
    value: String,
    data_type: NESType,
}

impl Literal {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn data_type(&self) -> NESType {
        self.data_type
    }
}

impl Into<Literal> for i32 {
    fn into(self) -> Literal {
        Literal {
            value: self.to_string(),
            data_type: NESType::Int32,
        }
    }
}

impl Into<Literal> for i64 {
    fn into(self) -> Literal {
        Literal {
            value: self.to_string(),
            data_type: NESType::Int64,
        }
    }
}

impl Into<Literal> for bool {
    fn into(self) -> Literal {
        Literal {
            value: self.to_string(),
            data_type: NESType::Bool,
        }
    }
}
