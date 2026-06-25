use std::sync::{Arc, Mutex};
use futures_util::StreamExt;
use tokio;
use zbus::Connection;
use crate::bar::Bar;
use crate::device::DeviceProxy;

#[derive(Default, Clone, Copy)]
pub struct Battery {
    pub percentage: usize,
    pub charging: bool
}

pub async fn listen(barc: Arc<Mutex<Bar>>) {
    let conn = Connection::system().await.expect("Cannot connect to DBus");
    let proxy = DeviceProxy::new(&conn).await.expect("Cannot create proxy");
    let mut percentage_stream = proxy.receive_percentage_changed().await;
    let mut state_stream = proxy.receive_state_changed().await;
    loop {
        tokio::select! {
            Some(v) = percentage_stream.next() => {
                let percentage = v.get().await.unwrap();
                barc.lock().unwrap().battery.percentage = percentage as usize;
            },
            Some(v) = state_stream.next() => {
                let charging = v.get().await.unwrap() == 1;
                barc.lock().unwrap().battery.charging = charging;
            }
        }
        barc.lock().unwrap().draw();
    }
}

