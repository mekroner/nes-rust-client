use super::{
    binary_expression::BinaryExpr, field::Field, literal::Literal, unary_expression::UnaryExpr,
};

#[derive(Debug, PartialEq)]
pub enum RawExpr {
    Literal(Literal),
    Field(Field),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

impl RawExpr {
    pub fn data_type(&self) -> NESType {
        match self {
            RawExpr::Literal(literal) => literal.data_type(),
            RawExpr::Field(field) => field.data_type(),
            RawExpr::Unary(expr) => expr.data_type(),
            RawExpr::Binary(expr) => expr.data_type(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NESType {
    #[default]
    Undefined,
    Bool,
    Char,
    Int32,
    Int64,
    Float32,
    Float64,
}

impl NESType {
    // TODO: Add type conversion.
    pub fn try_resolve(data_type1: NESType, data_type2: NESType) -> Option<NESType> {
        use NESType as T;
        match (data_type1, data_type2) {
            (T::Undefined, T::Undefined) => Some(T::Undefined),
            (T::Undefined, t) | (t, T::Undefined) => Some(t),
            (a, b) if a == b => Some(a),
            _ => None,
        }
    }
}
