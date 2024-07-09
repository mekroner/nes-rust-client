use nes_types::NesType;

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
    pub fn data_type(&self) -> NesType {
        match self {
            RawExpr::Literal(literal) => literal.data_type(),
            RawExpr::Field(field) => field.data_type(),
            RawExpr::Unary(expr) => expr.data_type(),
            RawExpr::Binary(expr) => expr.data_type(),
        }
    }
}


