use std::io::{prelude::*, stdout};

pub struct Bar {
    width: usize,
    workspace_id: u32,
    battery_percentage: u32
}

impl Bar {
    pub fn new(width: usize) -> Bar {
        Bar {
            width: width,
            workspace_id: 1,
            battery_percentage: 100
        }
    }

    fn draw(&self) {
        let ws = self.workspace_id.to_string();
        let bat = self.battery_percentage.to_string();
        let pad = " ".repeat(self.width - ws.len() - bat.len());
        print!("\x1b[?25l\r{}{}{}", ws, pad, bat);
        stdout().flush().unwrap();
    }

    pub fn set_workspace_id(&mut self, id: u32) {
        self.workspace_id = id;
        self.draw();
    }

    pub fn set_battery_percentage(&mut self, percentage: u32) {
        self.battery_percentage = percentage;
        self.draw();
    }
}

