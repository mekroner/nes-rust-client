use std::{error::Error, fmt::Display};

use nes_types::NesType;

use super::{
    binary_expression::{BinaryExpr, BinaryOp},
    expression::RawExpr,
    expression_builder_macros::{boolean_operator, cmp_operator},
    field::Field,
    literal::Literal,
    unary_expression::{UnaryExpr, UnaryOp},
    ArithmeticExpr, LogicalExpr,
};

// TODO: Make this error type
#[derive(Debug)]
pub struct ExprBuildError {}

impl Display for ExprBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ExprBuildError {}

/// The primary way to construct LogicalExpr, ArithmeticExpr, and FieldExpr.
///
pub struct ExprBuilder {
    expr: RawExpr,
    error: Option<ExprBuildError>,
}

impl ExprBuilder {
    // constructor
    pub fn literal(value: impl Into<Literal>) -> Self {
        Self {
            expr: RawExpr::Literal(value.into()),
            error: None,
        }
    }

    pub fn field(name: impl Into<String>) -> Self {
        Self {
            expr: RawExpr::Field(Field::untyped(name)),
            error: None,
        }
    }

    pub fn typed_field(name: impl Into<String>, data_type: NesType) -> Self {
        Self {
            expr: RawExpr::Field(Field::typed(name, data_type)),
            error: None,
        }
    }

    // unary logical
    pub fn not(mut self) -> Self {
        let data_type = match self.expr.data_type() {
            NesType::Undefined | NesType::Bool => NesType::Bool,
            _ => {
                self.error = Some(ExprBuildError {});
                NesType::Undefined
            }
        };
        self.expr = RawExpr::Unary(UnaryExpr {
            expr: Box::new(self.expr),
            operator: UnaryOp::Negate,
            data_type,
        });
        self
    }

    // binary logical
    boolean_operator!(and, BinaryOp::And);
    boolean_operator!(or, BinaryOp::Or);
    cmp_operator!(equals, BinaryOp::Equals);
    cmp_operator!(greater_than, BinaryOp::Greater);
    cmp_operator!(greater_equals, BinaryOp::GreaterEquals);
    cmp_operator!(less_than, BinaryOp::Less);
    cmp_operator!(less_equals, BinaryOp::LessEquals);

    // arith

    // builder
    pub fn build_arith(self) -> Result<ArithmeticExpr, ExprBuildError> {
        if let Some(err) = self.error {
            return Err(err);
        }
        match self.expr.data_type() {
            NesType::Float32 | NesType::Float64 | NesType::Int64 | NesType::Int32 => {
                Ok(ArithmeticExpr(self.expr))
            }
            _ => Err(ExprBuildError {}),
        }
    }

    pub fn build_logical(self) -> Result<LogicalExpr, ExprBuildError> {
        if let Some(err) = self.error {
            return Err(err);
        }
        if NesType::Bool == self.expr.data_type() {
            return Ok(LogicalExpr(self.expr));
        }
        Err(ExprBuildError {})
    }
}

#[cfg(test)]
mod expression_builder_test {
    use nes_types::NesType;
    use crate::query::expression::binary_expression::BinaryExpr;
    use crate::query::expression::binary_expression::BinaryOp;
    use crate::query::expression::expression::RawExpr as RE;
    use crate::query::expression::field::Field;
    use crate::query::expression::unary_expression::{UnaryExpr, UnaryOp};
    use crate::query::expression::ArithmeticExpr;
    use crate::query::expression::{ExprBuilder as EB, LogicalExpr};

    // #[test]
    // fn field_test() {
    //     let expr = E::field("value").build_field().unwrap();
    //     let expected = FieldExpr(E::Field(Field::untyped("value")));
    //     assert_eq!(expected, expr);
    //     assert!(E::literal(8).build_field().is_err());
    //     assert!(E::literal(true).build_field().is_err());
    // }

    #[test]
    fn logical_literal_test() {
        let expr = EB::literal(true).build_logical().unwrap();
        let expected = LogicalExpr(RE::Literal(true.into()));
        assert_eq!(expected, expr);
        assert!(EB::literal(0).build_logical().is_err());
        assert!(EB::field("value").build_logical().is_err());
    }

    #[test]
    fn arith_literal_test() {
        let expr = EB::literal(0).build_arith().unwrap();
        let expected = ArithmeticExpr(RE::Literal(0.into()));
        assert_eq!(expected, expr);
        assert!(EB::literal(true).build_arith().is_err());
        assert!(EB::field("value").build_arith().is_err());
    }

    #[test]
    fn negate_test() {
        let expr = EB::literal(true).not().build_logical().unwrap();
        let expected = LogicalExpr(RE::Unary(UnaryExpr {
            expr: Box::new(RE::Literal(true.into())),
            operator: UnaryOp::Negate,
            data_type: NesType::Bool,
        }));
        assert_eq!(expected, expr);
        let expr = EB::field("value").not().build_logical().unwrap();
        let expected = LogicalExpr(RE::Unary(UnaryExpr {
            expr: Box::new(RE::Field(Field::untyped("value"))),
            operator: UnaryOp::Negate,
            data_type: NesType::Bool,
        }));
        assert_eq!(expected, expr);
        assert!(EB::literal(0).not().build_logical().is_err());
    }

    #[test]
    fn eq_test() {
        let expr = EB::literal(0)
            .equals(EB::literal(2))
            .build_logical()
            .unwrap();
        let expected = LogicalExpr(RE::Binary(BinaryExpr {
            lhs: Box::new(RE::Literal(0.into())),
            rhs: Box::new(RE::Literal(2.into())),
            operator: BinaryOp::Equals,
            data_type: NesType::Bool,
        }));
        assert_eq!(expected, expr);
        assert!(EB::literal(0)
            .equals(EB::literal(true))
            .build_logical()
            .is_err());
        assert!(EB::literal(0)
            .equals(EB::field("value"))
            .build_logical()
            .is_ok());
    }

    #[test]
    fn and_test() {
        let expr = EB::literal(true)
            .and(EB::literal(false))
            .build_logical()
            .unwrap();
        let expected = LogicalExpr(RE::Binary(BinaryExpr {
            lhs: Box::new(RE::Literal(true.into())),
            rhs: Box::new(RE::Literal(false.into())),
            operator: BinaryOp::And,
            data_type: NesType::Bool,
        }));
        assert_eq!(expected, expr);
        assert!(EB::literal(0).and(EB::literal(0)).build_logical().is_err());
        assert!(EB::literal(0)
            .and(EB::field("value"))
            .build_logical()
            .is_err());
        assert!(EB::literal(true)
            .and(EB::field("value"))
            .build_logical()
            .is_ok());
    }

    #[test]
    fn or_test() {
        let expr = EB::literal(true)
            .or(EB::literal(false))
            .build_logical()
            .unwrap();
        let expected = LogicalExpr(RE::Binary(BinaryExpr {
            lhs: Box::new(RE::Literal(true.into())),
            rhs: Box::new(RE::Literal(false.into())),
            operator: BinaryOp::Or,
            data_type: NesType::Bool,
        }));
        assert_eq!(expected, expr);
        assert!(EB::literal(0).or(EB::literal(0)).build_logical().is_err());
        assert!(EB::literal(0)
            .or(EB::field("value"))
            .build_logical()
            .is_err());
        assert!(EB::literal(true)
            .or(EB::field("value"))
            .build_logical()
            .is_ok());
    }
}
