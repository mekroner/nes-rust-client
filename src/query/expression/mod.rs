pub mod binary_expression;
pub mod expression;
pub mod expression_builder;
mod expression_builder_macros;
pub mod field;
pub mod literal;
pub mod unary_expression;

use expression::RawExpr;
pub use expression_builder::{ExprBuildError, ExprBuilder};
pub use field::Field;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LogicalExpr(pub expression::RawExpr);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ArithmeticExpr(pub expression::RawExpr);

impl LogicalExpr {
    /// Helper function to negate the wrapped `RawExpr`
    pub fn not(self) -> Self {
        LogicalExpr(RawExpr::Unary(unary_expression::UnaryExpr {
            expr: Box::new(self.0),
            operator: unary_expression::UnaryOp::Negate,
            data_type: nes_types::NesType::Bool,
        }))
    }
}
