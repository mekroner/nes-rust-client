use nes_rust_client::{
    prelude::*,
    query::{expr_gen::expr_gen::generate_raw_expr, expression::Field, stringify::stringify_expr},
};
use nes_types::NesType;

fn main() {
    simple_logger::init_with_level(log::Level::Trace).expect("Simple_logger should not fail!");
    log::info!("This example shows the expression generator and pretty printing of expressions.");

    let fields = [
        Field::typed("Int64Value", NesType::Int64),
        Field::typed("Int32Value", NesType::Int32),
        Field::typed("Float64Value", NesType::Float64),
        Field::typed("BooleanValue", NesType::Bool),
    ];

    for i in 0..32 {
        let expr = stringify_expr(&generate_raw_expr(
            5,
            &fields,
            nes_types::NesType::Bool,
        ));
        log::info!("Expression {i}: {expr}");
    }
}
