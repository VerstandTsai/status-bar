use std::env;
use std::io::{prelude::*, BufReader};
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::bar::Bar;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Default, Serialize, Deserialize)]
pub struct Workspace {
    pub id: usize,
    pub name: String,
    pub monitor: String,
    #[serde(rename = "monitorID")]
    pub monitor_id: u32,
    pub windows: u32,
    #[serde(rename = "hasfullscreen")]
    pub has_fullscreen: bool,
    #[serde(rename = "lastwindow")]
    pub last_window: String,
    #[serde(rename = "lastwindowtitle")]
    pub last_window_title: String,
    #[serde(rename = "ispersistent")]
    pub is_persistent: bool,
    #[serde(rename = "tiledLayout")]
    pub tiled_layout: String
}

#[derive(Default)]
pub struct Hyprland {
    pub active: Workspace,
    pub workspaces: Vec<Workspace>
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

fn workspaces() -> Result<Vec<Workspace>> {
    let res = control("workspaces")?;
    let wss: Vec<Workspace> = serde_json::from_str(&res)?;
    Ok(wss)
}

pub async fn listen(barc: Arc<Mutex<Bar>>) {
    barc.lock().unwrap().hypr.active = active_workspace().unwrap();
    barc.lock().unwrap().hypr.workspaces = workspaces().unwrap();
    barc.lock().unwrap().draw();
    let prefix = socket_prefix().expect("Cannot get Hyprland socket path");
    let path = prefix + "/.socket2.sock";
    let hyprsocket = UnixStream::connect(path)
        .expect("Cannot connect to Hyprland socket");
    for line in BufReader::new(hyprsocket).lines().map(|x| x.unwrap()) {
        let parts: Vec<&str> = line.split(">>").collect();
        match parts[0] {
            "workspace" =>
                barc.lock().unwrap().hypr.active = active_workspace().unwrap(),
            "createworkspace" | "destroyworkspace" =>
                barc.lock().unwrap().hypr.workspaces = workspaces().unwrap(),
            _ => ()
        }
        barc.lock().unwrap().draw();
    }
}

