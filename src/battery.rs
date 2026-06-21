use std::sync::{Arc, Mutex};
use zbus::{zvariant::Value, Connection};
use crate::bar::Bar;

pub async fn listen(barc: Arc<Mutex<Bar>>) {
    let conn = Connection::system().await.expect("Cannot connect to dbus");
    let body = conn.call_method(
        Some("org.freedesktop.UPower"),
        "/org/freedesktop/UPower/devices/battery_BAT1",
        Some("org.freedesktop.DBus.Properties"),
        "Get",
        &("org.freedesktop.UPower.Device", "Percentage")
    ).await.unwrap().body();
    let res: Value = body.deserialize().unwrap();
    let percentage: f64 = res.try_into().unwrap();
    barc.lock().unwrap().set_battery_percentage(percentage as u32);
}

