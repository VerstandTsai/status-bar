use std::io::{prelude::*, stdout};
use chrono::{Datelike, Local, Timelike};
use crate::battery::Battery;
use crate::hypr::Hyprland;
use crate::wifi::WiFi;

#[derive(Default)]
pub struct Bar {
    width: usize,
    pub hypr: Hyprland,
    pub battery: Battery,
    pub wifi: WiFi
}

struct Component {
    string: String,
    width: usize
}

fn merge(components: Vec<Component>) -> Component {
    Component {
        string: components.iter().map(|x| x.string.clone()).collect(),
        width: components.iter().map(|x| x.width).sum()
    }
}

fn glyphs(x: &str) -> Vec<char> {
    x.chars().collect()
}

fn to_mandarin(x: usize) -> String {
    let tens = glyphs("　十廿卅");
    let digits = glyphs("〇一二三四五六七八九");
    match x {
        10 => "　十".to_string(),
        20 => "二十".to_string(),
        30 => "三十".to_string(),
        _ => format!("{}{}", tens[x / 10], digits[x % 10])
    }
}

impl Bar {
    pub fn new(width: usize) -> Bar {
        Bar {
            width: width,
            ..Default::default()
        }
    }

    fn draw_workspace(&self) -> Component {
        let max_id = self.hypr.workspaces
            .iter()
            .map(|x| x.id)
            .max()
            .unwrap_or(0);
        let mut s = String::new();
        for i in 1..max_id + 1 {
            s.push_str(if i == self.hypr.active.id { "  " } else { "  " });
        }
        Component {
            string: format!("\x1b[36m{}\x1b[39m", s),
            width: 3 * max_id
        }
    }

    fn draw_datetime(&self) -> Component {
        let clocks = glyphs("󱑊󱐿󱑀󱑁󱑂󱑃󱑄󱑅󱑆󱑇󱑈󱑉");
        let week = glyphs("日月火水木金土");
        let stems = glyphs("甲乙丙丁戊己庚辛壬癸");
        let branches = glyphs("子丑寅卯辰巳午未申酉戌亥");
        let now = Local::now();
        let year = now.year() as usize;
        let i = (year - 4) % 10;
        let j = (year - 4) % 12;
        Component {
            string: format!(
                "{}年{}月{}日（{}）{} {}",
                format!("{}{}", stems[i], branches[j]),
                to_mandarin(now.month() as usize),
                to_mandarin(now.day() as usize),
                week[now.weekday().num_days_from_sunday() as usize],
                clocks[(now.hour() % 12) as usize],
                now.format("%H:%M")
            ),
            width: 31
        }
    }

    fn draw_wifi(&self) -> Component {
        if !self.wifi.connected {
            return Component {
                string: " 󰤮 ".to_string(),
                width: 3
            }
        }
        let icons = glyphs("󰤟󰤢󰤥󰤨󰤨");
        let i = self.wifi.strength / 25;
        let raw = format!(" {} {} ", icons[i as usize], self.wifi.id);
        Component {
            string: format!("\x1b[34m{}\x1b[39m", raw),
            width: raw.chars().count()
        }
    }

    fn draw_battery(&self) -> Component {
        let icons0 = glyphs("󰂎󰁺󰁻󰁼󰁽󰁾󰁿󰂀󰂁󰂂󰁹");
        let icons1 = glyphs("󰢟󰢜󰂆󰂇󰂈󰢝󰂉󰢞󰂊󰂋󰂅");
        let i = self.battery.percentage / 10;
        let icon = if self.battery.charging { icons1[i] } else { icons0[i] };
        Component {
            string: format!(
                "\x1b[32m {} {}% \x1b[39m",
                icon,
                if self.battery.percentage == 100 {
                    "滿".to_string()
                } else {
                    self.battery.percentage.to_string()
                }),
            width: 7
        }
    }

    pub fn draw(&self) {
        let ws = self.draw_workspace();
        let dt = self.draw_datetime();
        let wifi = self.draw_wifi();
        let bat = self.draw_battery();
        let left = merge(vec![ws]);
        let center = merge(vec![dt]);
        let right = merge(vec![wifi, bat]);
        let rest = (self.width - center.width) / 2;
        let lpad = " ".repeat(rest - left.width);
        let rpad = " ".repeat(rest - right.width);
        let lstr = left.string;
        let cstr = center.string;
        let rstr = right.string;
        print!("\x1b[?25l\r{lstr}{lpad}{cstr}{rpad}{rstr}");
        stdout().flush().unwrap();
    }
}

