use std::sync::{Arc, Mutex};
use futures_util::StreamExt;
use tokio;
use zbus::Connection;
use zbus::proxy::PropertyStream;
use crate::bar::Bar;
use crate::network_manager::NetworkManagerProxy;
use crate::active::ActiveProxy;
use crate::access_point::AccessPointProxy;

#[derive(Default)]
pub struct WiFi {
    pub connected: bool,
    pub id: String,
    pub strength: u8
}

async fn listen_for_strength<'a>(
    barc: Arc<Mutex<Bar>>,
    mut stream:  PropertyStream<'a, u8>
) {
    while let Some(w) = stream.next().await {
        let strength = w.get().await.expect("Cannot get AP strength");
        barc.lock().unwrap().wifi.strength = strength;
        barc.lock().unwrap().draw();
    }
}

pub async fn listen(barc: Arc<Mutex<Bar>>) {
    let message = "Cannot create proxy";
    let conn = Connection::system().await.expect("Cannot connect to DBus");
    let nm = NetworkManagerProxy::new(&conn).await.expect(message);
    let mut active_stream = nm.receive_primary_connection_changed().await;
    let mut handle = tokio::spawn(async {});
    while let Some(v) = active_stream.next().await {
        handle.abort();
        let path = v.get().await.expect("Cannot get primary connection");
        if path.as_str() == "/" {
            barc.lock().unwrap().wifi.connected = false;
            barc.lock().unwrap().draw();
            continue;
        }
        barc.lock().unwrap().wifi.connected = true;
        let active = ActiveProxy::builder(&conn)
            .path(path).unwrap()
            .build().await.unwrap();
        barc.lock().unwrap().wifi.id = active.id().await.unwrap();
        barc.lock().unwrap().draw();
        let ap = AccessPointProxy::builder(&conn)
            .path(active.specific_object().await.unwrap()).unwrap()
            .build().await.unwrap();
        let strength_stream = ap.receive_strength_changed().await;
        handle = tokio::spawn(listen_for_strength(
            barc.clone(),
            strength_stream
        ));
    }
}

