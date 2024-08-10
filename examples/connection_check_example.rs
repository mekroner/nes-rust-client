extern crate nes_rust_client as nes_rs;
use nes_rs::prelude::*;

#[tokio::main]
async fn main(){
    simple_logger::init_with_level(log::Level::Trace).expect("Simple_logger should not fail!");
    log::info!("This example shows the is_connected function.");
    let runtime = NebulaStreamRuntime::new("localhost", 8081);
    let is_connected = runtime.check_connection().await;
    log::info!("NebulaStream is connected: {is_connected}");
}
