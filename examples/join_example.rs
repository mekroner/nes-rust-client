use nes_rs::prelude::*;
use nes_rs::query::{
    time::{Duration, TimeCharacteristic, TimeUnit},
    window::window_descriptor::WindowDescriptor,
};

extern crate nes_rust_client as nes_rs;

#[tokio::main]
async fn main() {
    let runtime = NebulaStreamRuntime::new("localhost".to_string(), 8081);

    let query_to_join = QueryBuilder::from_source("wind_turbines");
    let query = QueryBuilder::from_source("wind_turbines")
        .join_with(query_to_join)
        .where_field("id")
        .equals("turbine_id")
        .window(WindowDescriptor::TumblingWindow {
            duration: Duration::from_seconds(10),
            time_character: TimeCharacteristic::EventTime {
                field_name: todo!(),
                unit: TimeUnit::Milliseconds,
            },
        })
        .sink(Sink::Print);
    let response = runtime
        .execute_query(&query, PlacementStrategy::BottomUp)
        .await;
    match response {
        Ok(query_id) => log::info!("Started Execution of query with id: {query_id}"),
        Err(err) => log::error!("Failed to execute query: {:?}", err),
    }
    //TODO
}
