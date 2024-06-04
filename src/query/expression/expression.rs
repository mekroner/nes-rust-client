use std::collections::HashMap;

use super::{field::Field, literal::Literal, ArithmeticExpr, FieldExpr, LogicalExpr};

#[derive(Debug, PartialEq)]
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
            Expr::Binary { operator, .. } => {
                if operator.is_logical() {
                    NESType::Bool
                } else {
                    todo!()
                }
            }
        }
    }

    // constructor
    pub fn literal(value: impl Into<Literal>) -> Self {
        Expr::Literal(value.into())
    }

    pub fn field(name: impl Into<String>) -> Self {
        Expr::Field(Field::untyped(name))
    }

    pub fn typed_field(name: impl Into<String>, data_type: NESType) -> Self {
        Expr::Field(Field::typed(name, data_type))
    }

    // unary logical
    pub fn not(expr: Expr) -> Expr {
        unimplemented!()
    }

    // binary logical
    pub fn equals(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            operator: BinaryOp::Equals,
        }
    }

    // unary arith
    pub fn abs(expr: Expr) -> Expr {
        unimplemented!()
    }

    // binary arith
    pub fn add(lhs: Expr, rhs: Expr) -> Expr {
        unimplemented!()
    }

    // builder
    pub fn build_field(self) -> Result<FieldExpr, ExprBuildError> {
        self.validate()?;
        if let Expr::Field(_) = self {
            return Ok(FieldExpr(self));
        }
        Err(ExprBuildError {})
    }

    pub fn build_arith(self) -> Result<ArithmeticExpr, ExprBuildError> {
        self.validate()?;
        if NESType::Bool == self.data_type() {
            return Err(ExprBuildError {});
        }
        Ok(ArithmeticExpr(self))
    }

    pub fn build_logical(self) -> Result<LogicalExpr, ExprBuildError> {
        self.validate()?;
        if NESType::Bool == self.data_type() {
            return Ok(LogicalExpr(self));
        }
        Err(ExprBuildError {})
    }

    fn validate(&self) -> Result<(), ExprBuildError> {
        unimplemented!();
        Ok(())
    }

    pub fn evaluate(&self, context: &HashMap<String, Literal>) -> Literal {
        unimplemented!()
    }
}

// TODO: Make this error type
#[derive(Debug)]
pub struct ExprBuildError {}

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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NESType {
    #[default]
    Undefined,
    Bool,
    Int32,
    Int64,
}

#[cfg(test)]
mod expression_validation_test {
    use crate::query::expression::expression::{BinaryOp as BO, Expr as E};
    macro_rules! binary_op_test {
        ($test_name:ident, $lhs:expr, $rhs:expr, $operator:expr, $success:expr) => {
            #[test]
            fn $test_name() {
                assert!(
                    E::Binary {
                        lhs: Box::new($lhs),
                        rhs: Box::new($rhs),
                        operator: $operator,
                    }
                    .validate()
                    .is_ok()
                        == $success
                );
            }
        };
    }

    binary_op_test!(
        eq_test_bool_literal,
        E::Literal(true.into()),
        E::Literal(true.into()),
        BO::Equals,
        true
    );

    binary_op_test!(
        eq_test_arith_literal,
        E::Literal(8.into()),
        E::Literal(8.into()),
        BO::Equals,
        true
    );

    binary_op_test!(
        eq_test_literal_error,
        E::Literal(true.into()),
        E::Literal(8.into()),
        BO::Equals,
        false
    );
}

#[cfg(test)]
mod expression_builder_test {
    use crate::query::expression::{
        expression::Expr as E, field::Field, ArithmeticExpr, FieldExpr, LogicalExpr,
    };

    use super::BinaryOp;

    #[test]
    fn field_test() {
        let expr = E::field("value").build_field().unwrap();
        let expected = FieldExpr(E::Field(Field::untyped("value")));
        assert_eq!(expected, expr);
        assert!(E::literal(8).build_field().is_err());
        assert!(E::literal(true).build_field().is_err());
    }

    #[test]
    fn logical_literal_test() {
        let expr = E::literal(true).build_logical().unwrap();
        let expected = LogicalExpr(E::Literal(true.into()));
        assert_eq!(expected, expr);
        assert!(E::literal(0).build_logical().is_err());
        assert!(E::field("value").build_logical().is_err());
    }

    #[test]
    fn arith_literal_test() {
        let expr = E::literal(0).build_arith().unwrap();
        let expected = ArithmeticExpr(E::Literal(0.into()));
        assert_eq!(expected, expr);
        assert!(E::literal(true).build_arith().is_err());
        assert!(E::field("value").build_arith().is_err());
    }

    #[test]
    fn eq_test() {
        let expr = E::equals(E::literal(0), E::literal(2))
            .build_logical()
            .unwrap();
        let expected = LogicalExpr(E::Binary {
            lhs: Box::new(E::Literal(0.into())),
            rhs: Box::new(E::Literal(2.into())),
            operator: BinaryOp::Equals,
        });
        assert_eq!(expected, expr);
        assert!(E::equals(E::literal(0), E::literal(true))
            .build_logical()
            .is_err());
        assert!(E::equals(E::literal(0), E::literal(2))
            .build_arith()
            .is_err());
        assert!(E::equals(E::literal(0), E::field("value"))
            .build_field()
            .is_err());
    }
}
