extern crate nebulastream_rust_client as nes_rs;
use nes_rs::*;

#[tokio::main]
async fn main(){
    let runtime = NebulaStreamRuntime::new("localhost".to_string(), 8081);
    let response = runtime.check_connection().await;
    if let Err(err) = response {
        println!("{}", err);
    }
    // println!("Response: {}", response);
}
