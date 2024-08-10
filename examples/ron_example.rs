use nes_rust_client::prelude::ExprBuilder as EB;
use nes_rust_client::prelude::*;
use nes_rust_client::query::stringify;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Trace).expect("Init simple_logger should not fail!");
    let query = QueryBuilder::from_source("wind_turbines")
        .filter(
            EB::field("metadata_generated")
                .greater_than(EB::literal(0i64))
                .build_logical()
                .unwrap(),
        )
        .union(
            QueryBuilder::from_source("wind_turbines").filter(
                EB::field("metadata_generated")
                    .greater_than(EB::literal(0i64))
                    .build_logical()
                    .unwrap()
                    .not(),
            ),
        )
        .sink(Sink::Print);
    log::info!("Original Query: {}", stringify::stringify_query(&query));
    let ron_str = ron::to_string(&query).unwrap();
    log::info!("Serialized Ron Query: {}", ron_str);
    let deser_query = ron::from_str(&ron_str).unwrap();
    log::info!("Deserialized Query: {}", stringify::stringify_query(&deser_query));
}
