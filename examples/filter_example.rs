use nes_rs::{
    query::{expression::ExprBuilder as EB, sink::Sink, QueryBuilder},
    NebulaStreamRuntime,
};

extern crate nebulastream_rust_client as nes_rs;

#[tokio::main]
async fn main() {
    let runtime = NebulaStreamRuntime::new("localhost".to_string(), 8081);
    let query = QueryBuilder::from_source("wind_turbines".to_string())
        .filter(
            EB::field("metadata_generated")
                .greater_than(EB::literal(0i64))
                .build_logical()
                .unwrap(),
        )
        .sink(Sink::Print);
    let result = runtime.execute_query(query, "BottomUp".to_string()).await;
    dbg!(result);
    //TODO
}
