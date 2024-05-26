extern crate nebulastream_rust_client as nes_rs;

use nes_rs::{runtime::query::QueryBuilder, *};

#[tokio::main]
async fn main() {
    let runtime = NebulaStreamRuntime::new("localhost".to_string(), 8081);
    let logical_sources = runtime.logical_sources().await;
    match logical_sources {
        Ok(sources) => println!("{:?}", sources),
        Err(err) => println!("{}", err),
    }
    // let query = runtime.from_source("default_logical");
    let query = QueryBuilder::from_source("wind_turbines".to_string()).sink(operator::Sink::Print);
    let result = runtime.execute_query(query, "BottomUp".to_string()).await;
    dbg!(result);
    //TODO
}
