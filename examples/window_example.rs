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

    // The Cpp Query translates into the following query
    // Query::from("wind_turbines")
    //      .window(TumblingWindow::of(EventTime(Attribute("features_properties_updated")), Minutes(10)))
    //      .byKey(Attribute("metadata_id"))
    //      .apply(Sum(Attribute("features_properties_mag")))
    //      .sink(...);
    let query = QueryBuilder::from_source("wind_turbines".to_string())
        .window(WindowDescriptor::TumblingWindow {
            duration: Duration::from_minutes(10),
            time_character: TimeCharacteristic::EventTime {
                field_name: "features_properties_updated".to_string(),
                unit: TimeUnit::Milliseconds,
            },
        })
        .by_key("metadata_id")
        .apply([Aggregation::sum()
            .on_field("features_properties_mag")
            .as_field("features_properties_mag_sum")])
        .sink(Sink::Print);
    let result = runtime.execute_query(query, "BottomUp".to_string()).await;
    dbg!(result);
    //TODO
}
