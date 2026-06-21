mod bar;
mod hypr;
mod battery;
use bar::Bar;
use std::env;
use std::sync::{Arc, Mutex};
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let width: usize = args[1].parse().unwrap();
    let barc = Arc::new(Mutex::new(Bar::new(width)));
    tokio::spawn(hypr::listen(barc.clone()));
    tokio::spawn(battery::listen(barc.clone()));
    std::thread::park();
}

