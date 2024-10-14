use nes_rust_client::prelude::*;
use nes_rust_client::query::time::{Duration, TimeCharacteristic, TimeUnit};

#[tokio::main]
async fn main() {
    let runtime = NebulaStreamRuntime::new("localhost".to_string(), 8081);

    // Build a query to be send
    // The Cpp Query translates into the following query
    // Query::from("wind_turbines")
    //      .window(TumblingWindow::of(EventTime(Attribute("features_properties_updated")), Minutes(10)))
    //      .byKey(Attribute("metadata_id"))
    //      .apply(Sum(Attribute("features_properties_mag")))
    //      .sink(...);
    let query = runtime.from_source("wind_turbines")
        .window(WindowDescriptor::TumblingWindow {
            duration: Duration::from_minutes(10),
            time_character: TimeCharacteristic::EventTime {
                field_name: "features_properties_updated".to_string(),
                unit: TimeUnit::Milliseconds,
            },
        })
        // FIXME: Fails for some reason metadata_id fails
        // .by_key("metadata_id")
        .by_key("features_properties_capacity")
        .apply(
            [Aggregation::sum("features_properties_mag").as_field("features_properties_mag_sum")],
        )
        .sink(Sink::Print);
    // send query

    let response = runtime.execute_query(&query, PlacementStrategy::BottomUp).await;
    match response {
        Ok(query_id) => log::info!("Started Execution of query with id: {query_id}"),
        Err(err) => log::error!("Failed to execute query: {}", err),
    }
}
