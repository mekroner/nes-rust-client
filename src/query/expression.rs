#[derive(Debug)]
pub enum LogicalExpression {
    Attribute(String),
    Literal(i32),
    Equal(Box<LogicalExpression>, Box<LogicalExpression>),
    NotEqual(Box<LogicalExpression>, Box<LogicalExpression>),
    And(Box<LogicalExpression>, Box<LogicalExpression>),
    Or(Box<LogicalExpression>, Box<LogicalExpression>),
}
//
// #[derive(Debug)]
// pub enum ArithmeticExpression {
//     Attribute(String),
//     Literal(i32),
// }
