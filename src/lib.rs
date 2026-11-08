//! Jade Lite cho Horizon: panel thông tin của block đang nhìn.
//! `/jade` bật hoặc tắt panel.

mod api;

use horizon_mod_sdk::{
    Mod, Registration, game,
    hud::{self, Corner, Line},
};

const REACH: f64 = 20.0;

const WHITE: u32 = 0xFFFFFF;
const GRAY: u32 = 0xAAAAAA;
const BLUE: u32 = 0x5555FF;
const GREEN: u32 = 0x55FF55;

struct JadeLite {
    shown: bool,
    last: Vec<(String, u32)>,
}

fn pretty(id: &str) -> (String, String) {
    let (ns, path) = id.split_once(':').unwrap_or(("minecraft", id));
    (title(path), title(ns))
}

fn title(s: &str) -> String {
    s.split(['_', '/'])
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn distance(px: f64, py: f64, pz: f64, t: api::Target) -> f64 {
    let dx = t.x as f64 + 0.5 - px;
    let dy = t.y as f64 + 0.5 - (py + 1.62);
    let dz = t.z as f64 + 0.5 - pz;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn build_lines() -> Option<Vec<(String, u32)>> {
    let p = game::get_player()?;
    let t = api::target(REACH)?;
    let id = api::block_id(t)?;
    if id == "minecraft:air" || id == "minecraft:cave_air" || id == "minecraft:void_air" {
        return None;
    }
    let (name, ns) = pretty(&id);
    let mut lines = vec![
        (name, WHITE),
        (id.clone(), GRAY),
        (ns, BLUE),
        (
            format!(
                "{} {} {}  ({:.1} m)",
                t.x,
                t.y,
                t.z,
                distance(p.x, p.y, p.z, t)
            ),
            GRAY,
        ),
    ];
    if let Some(b) = api::biome(t) {
        lines.push((format!("Biome: {}", pretty(&b).0), GREEN));
    }
    Some(lines)
}

impl Mod for JadeLite {
    fn init() -> Result<(Self, Registration), String> {
        let hooks = Registration::new().command("jade").on_tick();
        Ok((
            JadeLite {
                shown: true,
                last: Vec::new(),
            },
            hooks,
        ))
    }

    fn tick(&mut self, _tick: u64) {
        let now = if self.shown { build_lines() } else { None };
        match now {
            Some(lines) => {
                if lines != self.last {
                    let panel: Vec<Line> = lines
                        .iter()
                        .map(|(text, color)| Line {
                            text: text.clone(),
                            color: *color,
                        })
                        .collect();
                    hud::set_panel(Corner::TopLeft, &panel);
                    self.last = lines;
                }
            }
            None => {
                if !self.last.is_empty() {
                    hud::clear();
                    self.last.clear();
                }
            }
        }
    }

    fn command(&mut self, _name: &str, _args: &str) -> Result<Option<String>, String> {
        self.shown = !self.shown;
        Ok(Some(format!(
            "Jade Lite {}",
            if self.shown { "bật" } else { "tắt" }
        )))
    }
}

horizon_mod_sdk::export_mod!(JadeLite);
