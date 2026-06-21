use std::sync::{Arc, Mutex};
use futures_util::StreamExt;
use tokio;
use zbus::Connection;
use crate::bar::Bar;
use crate::device::DeviceProxy;

#[derive(Copy, Clone)]
pub struct Battery {
    pub percentage: usize,
    pub charging: bool
}

impl Battery {
    pub fn new() -> Battery {
        Battery {
            percentage: 100,
            charging: false
        }
    }
}

pub async fn listen(barc: Arc<Mutex<Bar>>) {
    let mut battery = Battery::new();
    let conn = Connection::system().await.expect("Cannot connect to DBus");
    let proxy = DeviceProxy::new(&conn).await.expect("Cannot create proxy");
    battery.percentage = proxy.percentage().await.unwrap() as usize;
    barc.lock().unwrap().set_battery(battery);
    let mut percentage_stream = proxy.receive_percentage_changed().await;
    let mut state_stream = proxy.receive_state_changed().await;
    loop {
        tokio::select! {
            Some(v) = percentage_stream.next() =>
                battery.percentage = v.get().await.unwrap() as usize,
            Some(v) = state_stream.next() =>
                battery.charging = v.get().await.unwrap() == 1
        }
        barc.lock().unwrap().set_battery(battery);
    }
}

