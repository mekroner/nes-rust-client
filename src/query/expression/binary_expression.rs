use super::expression::{RawExpr, NESType};

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub lhs: Box<RawExpr>,
    pub rhs: Box<RawExpr>,
    pub operator: BinaryOp,
    pub data_type: NESType,
}

impl BinaryExpr {
    pub fn data_type(&self) -> NESType {
        self.data_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    And,
    Or,
    Equals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
    // Arithmetic
    Add,
    Sub,
    Multiply,
    Divide,
}

impl BinaryOp {
    pub const fn is_logical(&self) -> bool {
        match self {
            Self::And
            | Self::Or
            | Self::Equals
            | Self::Greater
            | Self::GreaterEquals
            | Self::Less
            | Self::LessEquals => true,
            _ => false,
        }
    }
}
