use nes_rust_client::prelude::ExprBuilder as EB;
use nes_rust_client::prelude::*;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Trace).expect("Init simple_logger should not fail!");
    let runtime = NebulaStreamRuntime::new("localhost", 8081);
    let query = QueryBuilder::from_source("wind_turbines".to_string())
        .filter(
            EB::field("metadata_generated")
                .greater_than(EB::literal(0i64))
                .build_logical()
                .unwrap(),
        )
        .sink(Sink::Print);
    let response = runtime.execute_query(&query, PlacementStrategy::BottomUp).await;
    match response {
        Ok(query_id) => log::info!("Started Execution of query with id: {query_id}"),
        Err(err) => log::error!("Failed to execute query: {:?}", err),
    }
}
