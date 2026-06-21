use std::io::{prelude::*, stdout};

use crate::battery::Battery;

pub struct Bar {
    width: usize,
    workspace_id: u32,
    battery: Battery
}

impl Bar {
    pub fn new(width: usize) -> Bar {
        Bar {
            width: width,
            workspace_id: 1,
            battery: Battery::new()
        }
    }

    fn draw_battery(&self) -> String {
        let icons0 = ["󰂎", "󰁺", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"];
        let icons1 = ["󰢟", "󰢜", "󰂆", "󰂇", "󰂈", "󰢝", "󰂉", "󰢞", "󰂊", "󰂋", "󰂅"];
        let i = self.battery.percentage / 10;
        let icon = if self.battery.charging { icons1[i] } else { icons0[i] };
        format!("{} {}%", icon, self.battery.percentage)
    }

    fn draw(&self) {
        let ws = self.workspace_id.to_string();
        let bat = self.draw_battery();
        let pad = " ".repeat(self.width - ws.len() - bat.len());
        print!("\x1b[?25l\r{}{}{}", ws, pad, bat);
        stdout().flush().unwrap();
    }

    pub fn set_workspace_id(&mut self, id: u32) {
        self.workspace_id = id;
        self.draw();
    }

    pub fn set_battery(&mut self, battery: Battery) {
        self.battery = battery;
        self.draw();
    }
}

