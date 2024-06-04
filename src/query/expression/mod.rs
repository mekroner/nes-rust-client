pub mod expression;
pub mod field;
pub mod literal;

#[derive(Debug, PartialEq)]
pub struct FieldExpr(pub expression::Expr);
#[derive(Debug, PartialEq)]
pub struct LogicalExpr(pub expression::Expr);
#[derive(Debug, PartialEq)]
pub struct ArithmeticExpr(pub expression::Expr);
