use nes_rust_client::prelude::*;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Trace).expect("Init simple_logger should not fail!");
    let runtime = NebulaStreamRuntime::new("localhost".to_string(), 8081);

    let query0 = QueryBuilder::from_source("wind_turbines");
    let query1 = QueryBuilder::from_source("wind_turbines")
        .union(query0)
        .sink(Sink::Print);
    let result = runtime.execute_query(query1, "BottomUp".to_string()).await;
    dbg!(result);
    //TODO
}