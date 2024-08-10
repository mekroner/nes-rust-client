use nes_types::NesType;
use serde::{Deserialize, Serialize};

use super::expression::RawExpr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpr {
    pub expr: Box<RawExpr>,
    pub operator: UnaryOp,
    pub data_type: NesType,
}

impl UnaryExpr {
    pub fn data_type(&self) -> NesType {
        self.data_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
