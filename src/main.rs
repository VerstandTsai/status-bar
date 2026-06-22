mod bar;
mod hypr;
mod battery;
mod device;
mod wifi;
mod access_point;
mod active;
mod network_manager;
use bar::Bar;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let width: usize = args[1].parse().expect("Cannot parse width");
    let barc = Arc::new(Mutex::new(Bar::new(width)));
    tokio::spawn(hypr::listen(barc.clone()));
    tokio::spawn(battery::listen(barc.clone()));
    tokio::spawn(wifi::listen(barc.clone()));
    loop {
        barc.lock().unwrap().draw();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

