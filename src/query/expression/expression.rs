use nes_types::NesType;
use serde::{Deserialize, Serialize};

use super::{
    binary_expression::{BinaryExpr, BinaryOp},
    field::Field,
    literal::Literal,
    unary_expression::{UnaryExpr, UnaryOp},
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RawExpr {
    Literal(Literal),
    Field(Field),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

impl RawExpr {
    pub fn unary(child: RawExpr, operator: UnaryOp, data_type: NesType) -> Self {
        Self::Unary(UnaryExpr {
            operator,
            data_type,
            expr: Box::new(child),
        })
    }

    pub fn binary(rhs: RawExpr, lhs: RawExpr, operator: BinaryOp, data_type: NesType) -> Self {
        Self::Binary(BinaryExpr {
            operator,
            data_type,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    pub fn data_type(&self) -> NesType {
        match self {
            RawExpr::Literal(literal) => literal.data_type(),
            RawExpr::Field(field) => field.data_type(),
            RawExpr::Unary(expr) => expr.data_type(),
            RawExpr::Binary(expr) => expr.data_type(),
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            RawExpr::Literal(_) => true,
            _ => false,
        }
    }

    pub fn is_field(&self) -> bool {
        match self {
            RawExpr::Field(_) => true,
            _ => false,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            RawExpr::Literal(_) | RawExpr::Field(_) => true,
            RawExpr::Unary(_) | RawExpr::Binary(_) => false,
        }
    }

    fn leafs_recursive(&self, vec: &mut Vec<RawExpr>) {
        match self {
            RawExpr::Literal(_) => vec.push(self.clone()),
            RawExpr::Field(_) => vec.push(self.clone()),
            RawExpr::Unary(UnaryExpr { expr, .. }) => expr.leafs_recursive(vec),
            RawExpr::Binary(BinaryExpr { lhs, rhs, .. }) => {
                lhs.leafs_recursive(vec);
                rhs.leafs_recursive(vec);
            }
        }
    }

    pub fn leafs(&self) -> Vec<RawExpr> {
        let mut vec = Vec::new();
        self.leafs_recursive(&mut vec);
        vec
    }

    fn leafs_parents_recursive(&self, vec: &mut Vec<RawExpr>) {
        match self {
            RawExpr::Unary(UnaryExpr { expr, .. }) => {
                if expr.is_leaf() {
                    vec.push(self.clone());
                }
                expr.leafs_parents_recursive(vec)
            }
            RawExpr::Binary(BinaryExpr { lhs, rhs, .. }) => {
                if lhs.is_leaf() || rhs.is_leaf() {
                    vec.push(self.clone());
                }
                lhs.leafs_parents_recursive(vec);
                rhs.leafs_parents_recursive(vec);
            }
            _ => ()
        }
    }

    pub fn leaf_parents(&self) -> Vec<RawExpr> {
        let mut vec = Vec::new();
        self.leafs_parents_recursive(&mut vec);
        vec
    }
}

impl From<Field> for RawExpr {
    fn from(field: Field) -> Self {
        RawExpr::Field(field)
    }
}

impl From<Literal> for RawExpr {
    fn from(literal: Literal) -> Self {
        RawExpr::Literal(literal)
    }
}

impl From<BinaryExpr> for RawExpr {
    fn from(expr: BinaryExpr) -> Self {
        RawExpr::Binary(expr)
    }
}

impl From<UnaryExpr> for RawExpr {
    fn from(expr: UnaryExpr) -> Self {
        RawExpr::Unary(expr)
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use nes_types::NesType;

    use crate::query::{
        expression::{binary_expression::BinaryOp, Field},
        stringify::stringify_expr,
    };

    use super::RawExpr;

    fn vec_eq(this: &Vec<RawExpr>, other: &Vec<RawExpr>) {
        assert_eq!(this.len(), other.len());
        for expr in this {
            assert!(
                other.contains(expr),
                "other does not contain {}",
                stringify_expr(expr)
            );
        }
    }

    fn common_values() -> (Vec<RawExpr>, Vec<RawExpr>) {
        let field0: RawExpr = Field::untyped("field0").into();
        let field1: RawExpr = Field::untyped("field1").into();
        let field2: RawExpr = Field::untyped("field2").into();
        let field3: RawExpr = Field::untyped("field3").into();
        let expr0 = RawExpr::binary(
            field0.clone(),
            field1.clone(),
            BinaryOp::And,
            NesType::Undefined,
        );
        let expr1 = RawExpr::binary(field2.clone(), field3.clone(), BinaryOp::Or, NesType::Bool);
        let expr2 = RawExpr::binary(expr0.clone(), expr1.clone(), BinaryOp::Or, NesType::Bool);
        let expr3 = RawExpr::binary(expr0.clone(), field0.clone(), BinaryOp::And, NesType::Bool);
        (
            vec![field0, field1, field2, field3],
            vec![expr0, expr1, expr2, expr3],
        )
    }

    #[test]
    fn test_leafs() {
        let (fields, exprs) = common_values();
        let expected_leafs0 = vec![fields[0].clone(), fields[1].clone()];
        let expected_leafs1 = vec![fields[2].clone(), fields[3].clone()];
        let expected_leafs2 = vec![
            fields[0].clone(),
            fields[1].clone(),
            fields[2].clone(),
            fields[3].clone(),
        ];
        let expected_leafs3 = vec![fields[0].clone(), fields[1].clone(), fields[0].clone()];

        vec_eq(&expected_leafs0, &exprs[0].leafs());
        vec_eq(&expected_leafs1, &exprs[1].leafs());
        vec_eq(&expected_leafs2, &exprs[2].leafs());
        vec_eq(&expected_leafs3, &exprs[3].leafs());
    }

    #[test]
    fn test_leaf_parents() {
        let (_, exprs) = common_values();
        let expected0 = vec![exprs[0].clone()];
        let expected1 = vec![exprs[1].clone()];
        let expected2 = vec![exprs[0].clone(), exprs[1].clone()];
        let expected3 = vec![exprs[0].clone(), exprs[3].clone()];
        vec_eq(&expected0, &exprs[0].leaf_parents());
        vec_eq(&expected1, &exprs[1].leaf_parents());
        vec_eq(&expected2, &exprs[2].leaf_parents());
        vec_eq(&expected3, &exprs[3].leaf_parents());
    }
}
