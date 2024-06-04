use super::expression::{RawExpr, NESType};

#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    pub expr: Box<RawExpr>,
    pub operator: UnaryOp,
    pub data_type: NESType,
}

impl UnaryExpr {
    pub fn data_type(&self) -> NESType {
        self.data_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Negate,
}

impl UnaryOp {
    pub const fn is_logical(&self) -> bool {
        match self {
            UnaryOp::Negate => true,
        }
    }
}
