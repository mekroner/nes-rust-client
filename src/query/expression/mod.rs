pub mod binary_expression;
pub mod expression;
pub mod expression_builder;
mod expression_builder_macros;
pub mod field;
pub mod literal;
pub mod unary_expression;

pub use expression_builder::{ExprBuildError, ExprBuilder};

#[derive(Debug, PartialEq)]
pub struct FieldExpr(pub expression::RawExpr);
#[derive(Debug, PartialEq)]
pub struct LogicalExpr(pub expression::RawExpr);
#[derive(Debug, PartialEq)]
pub struct ArithmeticExpr(pub expression::RawExpr);
