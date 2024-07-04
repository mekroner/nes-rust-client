use nes_rust_client::{
    prelude::{ExprBuilder as EB, *},
    query::{
        stringify::stringify_query,
        time::{Duration, TimeCharacteristic, TimeUnit},
    },
};

fn main() {
    simple_logger::init_with_level(log::Level::Trace).expect("Simple_logger should not fail!");
    log::info!("This example shows query construction and pretty printing of queries.");

    let q_window = QueryBuilder::from_source("wind_turbines")
        .window(WindowDescriptor::TumblingWindow {
            duration: Duration::from_minutes(10),
            time_character: TimeCharacteristic::EventTime {
                field_name: "features_properties_updated".to_string(),
                unit: TimeUnit::Milliseconds,
            },
        })
        .by_key("features_properties_capacity")
        .apply(
            [Aggregation::sum("features_properties_mag").as_field("features_properties_mag_sum")],
        )
        .sink(Sink::Print);

    let q_filter = QueryBuilder::from_source("wind_turbines")
        .filter(
            EB::field("metadata_generated")
                .greater_than(EB::literal(0i64))
                .build_logical()
                .unwrap(),
        )
        .sink(Sink::Print);

    let q_union = QueryBuilder::from_source("test")
        .filter(
            EB::field("value1")
                .equals(EB::field("value2"))
                .not()
                .build_logical()
                .unwrap(),
        )
        .union(
            QueryBuilder::from_source("test").filter(
                EB::field("value1")
                    .equals(EB::field("value2"))
                    .build_logical()
                    .unwrap(),
            ),
        )
        .sink(Sink::csv_file("./result.csv", true));

    let q_join = QueryBuilder::from_source("orders")
        .join_with(QueryBuilder::from_source("products"))
        .where_field("products")
        .equals("id")
        .window(WindowDescriptor::TumblingWindow {
            duration: Duration::from_seconds(10),
            time_character: TimeCharacteristic::EventTime {
                field_name: "ts".to_string(),
                unit: TimeUnit::Milliseconds,
            },
        })
        .sink(Sink::null());
    let queries = [q_window, q_filter, q_union, q_join];
    for (i, q) in queries.iter().enumerate() {
        log::info!("Query {i}: {}", stringify_query(q));
    }
}
