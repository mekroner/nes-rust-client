use super::{field::Field, literal::Literal};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Field(Field),
    Unary {
        expr: Box<Expr>,
        operator: UnaryOp,
    },
    Binary {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        operator: BinaryOp,
    },
}

impl Expr {
    pub fn data_type(&self) -> NESType {
        match self {
            Expr::Literal(literal) => literal.data_type(),
            Expr::Field(field) => field.data_type(),
            Expr::Unary { expr, operator } => todo!(),
            Expr::Binary { lhs, rhs, operator } => {
                if operator.is_logical() {
                    NESType::Bool
                } else {
                    todo!()
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum NESType {
    Undefined,
    Bool,
    Int32,
    Int64,
}

mod expression_test {
    #[test]
    fn macro_test() {
        // let expr0 = expr!(|user_id| user_id >= 1 && user_id < 10000);
        // dbg!(expr!(3 + 5));
        // dbg!(expr!(3 - 5));
        // dbg!(expr!(3 * 5));
        // dbg!(expr!(3 / 5));
        assert!(false);
    }
}
