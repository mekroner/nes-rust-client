#[derive(Debug)]
enum Expr {
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

#[derive(Debug)]
pub enum UnaryOp {
    And,
    Or,
    Equal,
    Add,
    Sub,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum BinaryOp {
    And,
    Or,
    Equal,
    Add,
    Sub,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct FieldExpr(Expr);
#[derive(Debug)]
pub struct LogicalExpr(Expr);
#[derive(Debug)]
pub struct ArithmeticExpr(Expr);

#[derive(Debug)]
pub enum NESType {
    Any,
}

#[derive(Debug)]
pub struct Field {
    name: String,
    nes_type: NESType,
}

#[derive(Debug)]
pub struct Literal {
    value: String,
    nes_type: NESType,
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
