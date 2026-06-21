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
    id: usize,
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

fn workspaces() -> Result<Vec<Workspace>> {
    let res = control("workspaces")?;
    let wss: Vec<Workspace> = serde_json::from_str(&res)?;
    Ok(wss)
}

fn n_workspaces() -> Result<usize> {
    let wss = workspaces()?;
    let max_id = wss.iter().map(|x| x.id).max().unwrap();
    Ok(max_id)
}

pub async fn listen(barc: Arc<Mutex<Bar>>) {
    barc.lock().unwrap().set_workspace_id(active_workspace().unwrap().id);
    barc.lock().unwrap().set_n_workspaces(n_workspaces().unwrap());
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
            "createworkspace" => barc
                .lock()
                .unwrap()
                .set_n_workspaces(n_workspaces().unwrap()),
            _ => ()
        }
    }
}

