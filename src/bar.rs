use std::io::{prelude::*, stdout};
use chrono::Local;
use crate::battery::Battery;
use crate::wifi::WiFi;

pub struct Bar {
    width: usize,
    n_workspaces: usize,
    workspace_id: usize,
    battery: Battery,
    wifi: WiFi
}

impl Bar {
    pub fn new(width: usize) -> Bar {
        Bar {
            width: width,
            workspace_id: 1,
            n_workspaces: 1,
            battery: Battery::new(),
            wifi: WiFi::new()
        }
    }

    fn draw_workspace(&self) -> String {
        let mut s = String::new();
        for i in 1..self.n_workspaces + 1 {
            s = s + if i == self.workspace_id { "  " } else { "  " };
        }
        s
    }

    fn draw_datetime(&self) -> String {
        let now = Local::now();
        format!(" {} ", now.format("%Y/%m/%d (%a) %H:%M:%S"))
    }

    fn draw_wifi(&self) -> String {
        let icons = ["󰤟", "󰤢", "󰤥", "󰤨", "󰤨"];
        let i = self.wifi.strength / 25;
        format!(" {} {} ", icons[i as usize], self.wifi.id)
    }

    fn draw_battery(&self) -> String {
        let icons0 = ["󰂎", "󰁺", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"];
        let icons1 = ["󰢟", "󰢜", "󰂆", "󰂇", "󰂈", "󰢝", "󰂉", "󰢞", "󰂊", "󰂋", "󰂅"];
        let i = self.battery.percentage / 10;
        let icon = if self.battery.charging { icons1[i] } else { icons0[i] };
        format!(" {} {}% ", icon, self.battery.percentage)
    }

    pub fn draw(&self) {
        let ws = self.draw_workspace();
        let dt = self.draw_datetime();
        let wifi = self.draw_wifi();
        let bat = self.draw_battery();
        let left = format!("{ws}");
        let center = format!("{dt}");
        let right = format!("{wifi}{bat}");
        let rest = (self.width - center.chars().count()) / 2;
        let lpad = " ".repeat(rest - left.chars().count());
        let rpad = " ".repeat(rest - right.chars().count());
        print!("\x1b[?25l\r{left}{lpad}{center}{rpad}{right}");
        stdout().flush().unwrap();
    }

    pub fn set_workspace_id(&mut self, id: usize) {
        self.workspace_id = id;
        self.draw();
    }

    pub fn set_n_workspaces(&mut self, n: usize) {
        self.n_workspaces = n;
        self.draw();
    }

    pub fn set_battery(&mut self, battery: Battery) {
        self.battery = battery;
        self.draw();
    }

    pub fn set_wifi(&mut self, wifi: WiFi) {
        self.wifi = wifi;
        self.draw();
    }
}

