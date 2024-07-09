use nes_types::NesType;
use rand::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    prelude::ExprBuilder,
    query::{
        expression::{
            binary_expression::{BinaryExpr, BinaryOp},
            expression::RawExpr,
            literal::Literal,
            unary_expression::{UnaryExpr, UnaryOp},
            Field, LogicalExpr,
        },
        operator,
    },
};

pub fn generate_logical_expr(depth: u32, fields: &[Field]) -> LogicalExpr {
    LogicalExpr(generate_raw_expr(depth, fields, NesType::Bool))
}

/// Generates a random expression. Leaf nodes are `Fields` or `Literal`s and non leaf nodes are
/// `BinaryExpr` or `UnaryExpr`. Each branch has the specified depth. Fields are selected from the List
/// of `fields`. The `data_type` specifies the return type of the expression. Undefined results in a
/// random return type.
pub fn generate_raw_expr(depth: u32, fields: &[Field], data_type: NesType) -> RawExpr {
    if depth == 0 {
        return RawExpr::Field(generate_field(fields, data_type).expect("No valid field found!"));
    }
    let left_child = generate_raw_expr(depth - 1, fields, NesType::Undefined);
    let input_type = left_child.data_type();
    let right_child = generate_raw_expr(depth - 1, fields, input_type);
    let operator = generate_binary_op(input_type, data_type).expect("No valid binary operator found!");
    let output_type = binary_op_output_type(operator, input_type);
    let binary = BinaryExpr {
        lhs: Box::new(left_child),
        rhs: Box::new(right_child),
        data_type: output_type,
        operator,
    };
    let expr = RawExpr::Binary(binary);
    expr
}

fn generate_field(fields: &[Field], data_type: NesType) -> Option<Field> {
    let mut rng = rand::thread_rng();
    if data_type == NesType::Undefined {
        return fields.choose(&mut rng).cloned();
    }
    fields
        .iter()
        .filter(|field| field.data_type() == data_type)
        .choose(&mut rng)
        .cloned()
}

fn generate_binary_op(input_type: NesType, output_type: NesType) -> Option<BinaryOp> {
    let mut rng = rand::thread_rng();
    BinaryOp::iter()
        .filter(|&operator| input_type == NesType::Undefined || binary_op_accepted_input_types(operator).contains(&input_type))
        .filter(|&operator| {
            output_type == NesType::Undefined
                || binary_op_output_type(operator, input_type) == output_type
        })
        .choose(&mut rng)
}

fn binary_op_accepted_input_types(operator: BinaryOp) -> Vec<NesType> {
    match operator {
        BinaryOp::And | BinaryOp::Or => logical_types(),
        BinaryOp::Equals
        | BinaryOp::Greater
        | BinaryOp::GreaterEquals
        | BinaryOp::Less
        | BinaryOp::LessEquals
        | BinaryOp::Add
        | BinaryOp::Sub
        | BinaryOp::Multiply
        | BinaryOp::Divide => arithmetic_types(),
    }
}

fn binary_op_output_type(operator: BinaryOp, input_type: NesType) -> NesType {
    match operator {
        BinaryOp::And
        | BinaryOp::Or
        | BinaryOp::Equals
        | BinaryOp::Greater
        | BinaryOp::GreaterEquals
        | BinaryOp::Less
        | BinaryOp::LessEquals => NesType::Bool,
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Multiply | BinaryOp::Divide => input_type,
    }
}

fn arithmetic_types() -> Vec<NesType> {
    vec![
        NesType::Int32,
        NesType::Int64,
        NesType::Float32,
        NesType::Float64,
    ]
}

fn logical_types() -> Vec<NesType> {
    vec![NesType::Bool]
}
