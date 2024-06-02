use nes_rs::{
    query::{
        expression::{
            expression::{BinaryOp, Expr, NESType},
            field::Field,
            LogicalExpr,
        },
        sink::Sink,
        QueryBuilder,
    },
    NebulaStreamRuntime,
};

extern crate nebulastream_rust_client as nes_rs;

#[tokio::main]
async fn main() {
    let runtime = NebulaStreamRuntime::new("localhost".to_string(), 8081);
    let logical_sources = runtime.logical_sources().await;
    match logical_sources {
        Ok(sources) => println!("{:?}", sources),
        Err(err) => println!("{}", err),
    }
    let query = QueryBuilder::from_source("wind_turbines".to_string())
        .filter(LogicalExpr(Expr::Binary {
            lhs: Box::new(Expr::Field(Field::untyped("metadata_generated"))),
            rhs: Box::new(Expr::Literal(0i64.into())),
            operator: BinaryOp::LessEquals,
        }))
        .sink(Sink::Print);
    let result = runtime.execute_query(query, "BottomUp".to_string()).await;
    dbg!(result);
    //TODO
}
