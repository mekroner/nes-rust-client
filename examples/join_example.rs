use nes_rs::prelude::*;
use nes_rs::query::{
    time::{Duration, TimeCharacteristic, TimeUnit},
    window::window_descriptor::WindowDescriptor,
};

extern crate nes_rust_client as nes_rs;

#[tokio::main]
async fn main() {
    let runtime = NebulaStreamRuntime::new("localhost".to_string(), 8081);

    let query0 = QueryBuilder::from_source("wind_turbines");
    let query1 = QueryBuilder::from_source("wind_turbines")
        .join_with(query0)
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
    let result = runtime.execute_query(query1, "BottomUp".to_string()).await;
    dbg!(result);
    //TODO
}
