use nes_rs::{
    query::{
        sink::Sink,
        time::{Duration, TimeCharacteristic, TimeUnit},
        window::{
            aggregation::{Aggregation, AggregationType},
            window_descriptor::WindowDescriptor,
        },
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
        .window(WindowDescriptor::TumblingWindow {
            duration: Duration {
                amount: 10_000,
                unit: TimeUnit::Milliseconds,
            },
            time_character: TimeCharacteristic::EventTime {
                field_name: "features_properties_time".to_string(),
                unit: TimeUnit::Milliseconds,
            },
        })
        .apply([Aggregation {
            field_name: "features_properties_mag".into(),
            projected_field_name: None,
            agg_type: AggregationType::Count,
        }])
        .sink(Sink::Print);
    let result = runtime.execute_query(query, "BottomUp".to_string()).await;
    dbg!(result);
    //TODO
}
