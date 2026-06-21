use std::sync::{Arc, Mutex};
use futures_util::StreamExt;
use zbus::Connection;
use crate::bar::Bar;
use crate::device::DeviceProxy;

pub async fn listen(barc: Arc<Mutex<Bar>>) {
    let conn = Connection::system().await.expect("Cannot connect to DBus");
    let proxy = DeviceProxy::new(&conn).await.expect("Cannot create proxy");
    let percentage = proxy.percentage().await.expect("Cannot get percentage");
    barc.lock().unwrap().set_battery_percentage(percentage as u32);
    let mut stream = proxy.receive_percentage_changed().await;
    while let Some(x) = stream.next().await {
        let percentage = x.get().await.expect("Cannot get percentage");
        barc.lock().unwrap().set_battery_percentage(percentage as u32);
    }
}

