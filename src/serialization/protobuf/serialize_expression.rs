use prost_types::Any;

use super::{
    nes::{
        serializable_data_value::BasicValue,
        serializable_expression::{
            AbsExpression, AddExpression, AndExpression, ConstantValueExpression, DivExpression, EqualsExpression, FieldAccessExpression, FieldAssignmentExpression, GreaterEqualsExpression, GreaterExpression, LessEqualsExpression, LessExpression, ModExpression, MulExpression, NegateExpression, OrExpression, PowExpression, SubExpression
        },
        SerializableDataValue, SerializableExpression,
    },
    serialize_data_type::serialize_data_type,
};
use crate::expression::{
    binary_expression::{BinaryExpr, BinaryOp},
    expression::RawExpr,
    field::Field,
    literal::Literal,
    unary_expression::{UnaryExpr, UnaryOp},
};

pub fn serialize_expression(expr: &RawExpr) -> SerializableExpression {
    log::trace!("Serialize expression: {:?}", expr);
    let data_type = serialize_data_type(expr.data_type());
    let details = match expr {
        RawExpr::Literal(literal) => literal_details(literal),
        RawExpr::Field(field) => field_details(field),
        RawExpr::Unary(UnaryExpr { expr, operator, .. }) => {
            unary_operator_details(*operator, serialize_expression(expr))
        }
        RawExpr::Binary(BinaryExpr {
            lhs, rhs, operator, ..
        }) => binary_operator_details(
            *operator,
            serialize_expression(lhs),
            serialize_expression(rhs),
        ),
    };
    SerializableExpression {
        details: Some(details),
        children: vec![],
        stamp: Some(data_type),
    }
}

pub fn serialize_field_assignment(field: &Field, expr: &RawExpr) -> SerializableExpression {
    let data_type = serialize_data_type(field.data_type());
    let details = field_assignment_details(field, expr);
    SerializableExpression {
        details: Some(details),
        children: vec![],
        stamp: Some(data_type),
    }
}

pub fn serialize_field(field: &Field) -> SerializableExpression {
    let data_type = serialize_data_type(field.data_type());
    let details = field_details(field);
    SerializableExpression {
        details: Some(details),
        children: vec![],
        stamp: Some(data_type),
    }
}

fn literal_details(literal: &Literal) -> prost_types::Any {
    let value = BasicValue {
        r#type: Some(serialize_data_type(literal.data_type())),
        value: literal.value().to_string(),
    };
    let data_value = SerializableDataValue {
        value: Some(Any::from_msg(&value).unwrap()),
    };
    let expr = ConstantValueExpression {
        value: Some(data_value),
    };
    Any::from_msg(&expr).unwrap()
}

fn field_assignment_details(field: &Field, raw_expr: &RawExpr) -> prost_types::Any {
    let field_access = FieldAccessExpression {
        field_name: field.name().to_string(),
        r#type: Some(serialize_data_type(field.data_type())),
    };
    let expr = FieldAssignmentExpression {
        field: Some(field_access),
        assignment: Some(serialize_expression(raw_expr)),
    };
    Any::from_msg(&expr).unwrap()
}

fn field_details(field: &Field) -> prost_types::Any {
    let expr = FieldAccessExpression {
        field_name: field.name().to_string(),
        r#type: Some(serialize_data_type(field.data_type())),
    };
    Any::from_msg(&expr).unwrap()
}

macro_rules! unary_op {
    ($child:expr, $expr:ident) => {
        Any::from_msg(&$expr {
            child: Some($child),
        })
    };
}

fn unary_operator_details(operator: UnaryOp, child: SerializableExpression) -> prost_types::Any {
    match operator {
        UnaryOp::Negate => unary_op!(child, NegateExpression),
        UnaryOp::Absolute => unary_op!(child, AbsExpression),
    }
    .unwrap()
}

macro_rules! binary_op {
    ($lhs:expr, $rhs:expr, $expr:ident) => {
        Any::from_msg(&$expr {
            left: Some($lhs),
            right: Some($rhs),
        })
    };
}

fn binary_operator_details(
    operator: BinaryOp,
    lhs: SerializableExpression,
    rhs: SerializableExpression,
) -> prost_types::Any {
    match operator {
        // Logical
        BinaryOp::And => binary_op!(lhs, rhs, AndExpression),
        BinaryOp::Or => binary_op!(lhs, rhs, OrExpression),

        // Relational
        BinaryOp::Equals => binary_op!(lhs, rhs, EqualsExpression),
        BinaryOp::Greater => binary_op!(lhs, rhs, GreaterExpression),
        BinaryOp::GreaterEquals => binary_op!(lhs, rhs, GreaterEqualsExpression),
        BinaryOp::Less => binary_op!(lhs, rhs, LessExpression),
        BinaryOp::LessEquals => binary_op!(lhs, rhs, LessEqualsExpression),

        // Arithmetic
        BinaryOp::Add => binary_op!(lhs, rhs, AddExpression),
        BinaryOp::Sub => binary_op!(lhs, rhs, SubExpression),
        BinaryOp::Multiply => binary_op!(lhs, rhs, MulExpression),
        BinaryOp::Divide => binary_op!(lhs, rhs, DivExpression),
        BinaryOp::Remainder => binary_op!(lhs, rhs, ModExpression),
        BinaryOp::Power => binary_op!(lhs, rhs, PowExpression),
    }
    .unwrap()
}
