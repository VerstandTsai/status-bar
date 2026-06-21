use std::env;
use std::io::{prelude::*, BufReader};
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::bar::Bar;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize)]
struct Workspace {
    id: u32,
    name: String,
    monitor: String,
    #[serde(rename = "monitorID")]
    monitor_id: u32,
    windows: u32,
    #[serde(rename = "hasfullscreen")]
    has_fullscreen: bool,
    #[serde(rename = "lastwindow")]
    last_window: String,
    #[serde(rename = "lastwindowtitle")]
    last_window_title: String,
    #[serde(rename = "ispersistent")]
    is_persistent: bool,
    #[serde(rename = "tiledLayout")]
    tiled_layout: String
}

fn socket_prefix() -> Result<String> {
    let xdgrd = env::var("XDG_RUNTIME_DIR")?;
    let his = env::var("HYPRLAND_INSTANCE_SIGNATURE")?;
    Ok(format!("{xdgrd}/hypr/{his}"))
}

fn control(command: &str) -> Result<String> {
    let path = socket_prefix()? + "/.socket.sock";
    let mut hyprsocket = UnixStream::connect(path)?;
    hyprsocket.write_all(format!("j/{command}").as_bytes())?;
    let mut res = String::new();
    hyprsocket.read_to_string(&mut res)?;
    Ok(res)
}

fn active_workspace() -> Result<Workspace> {
    let res = control("activeworkspace")?;
    let ws: Workspace = serde_json::from_str(&res)?;
    Ok(ws)
}

pub async fn listen(barc: Arc<Mutex<Bar>>) {
    let ws = active_workspace().expect("Cannot get active workspace");
    barc.lock().unwrap().set_workspace_id(ws.id);
    let prefix = socket_prefix().expect("Cannot get Hyprland socket path");
    let path = prefix + "/.socket2.sock";
    let hyprsocket = UnixStream::connect(path)
        .expect("Cannot connect to Hyprland socket");
    for line in BufReader::new(hyprsocket).lines().map(|x| x.unwrap()) {
        let parts: Vec<&str> = line.split(">>").collect();
        let event = parts[0];
        let value = parts[1];
        match event {
            "workspace" => barc
                .lock()
                .unwrap()
                .set_workspace_id(value.parse().unwrap()),
            _ => ()
        }
    }
}

