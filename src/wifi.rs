use std::sync::{Arc, Mutex};
use futures_util::StreamExt;
use tokio;
use zbus::Connection;
use crate::bar::Bar;
use crate::network_manager::NetworkManagerProxy;
use crate::active::ActiveProxy;
use crate::access_point::AccessPointProxy;

#[derive(Clone)]
pub struct WiFi {
    pub id: String,
    pub strength: u8
}

impl WiFi {
    pub fn new() -> WiFi {
        WiFi {
            id: "Unknown".to_string(),
            strength: 100
        }
    }
}

pub async fn listen(barc: Arc<Mutex<Bar>>) {
    let mut wifi = WiFi::new();
    let conn = Connection::system().await.expect("Cannot connect to DBus");
    let nm = NetworkManagerProxy::new(&conn).await.unwrap();
    let active = ActiveProxy::builder(&conn)
        .path(nm.primary_connection().await.unwrap())
        .unwrap()
        .build()
        .await
        .expect("Cannot create proxy");
    let ap = AccessPointProxy::builder(&conn)
        .path(active.specific_object().await.unwrap())
        .unwrap()
        .build()
        .await
        .expect("Cannot create proxy");
    wifi.id = active.id().await.expect("Cannot get connection ID");
    wifi.strength = ap.strength().await.expect("Cannot get AP strength");
    barc.lock().unwrap().set_wifi(wifi);
}

