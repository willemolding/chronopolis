use std::collections::HashMap;

use chrono::{DateTime, Local, Timelike};
use nannou::prelude::*;
use nannou::{App, wgpu};

/// Read-only, host-computed data handed to every face each frame.
#[derive(Debug, Default)]
pub struct ClockContext {
    pub time: DateTime<Local>,
    /// Raw hand angles (radians, 12 o'clock = 0, clockwise positive).
    pub sec_angle: f32,
    pub min_angle: f32,
    pub hour_angle: f32,
    /// Radius of the circular canvas; draw outside this and it's off-face.
    pub radius: f32,
    /// Seconds elapsed since the previous frame. Pass to `ClockHand::animate_to`.
    pub dt: f32,
    pub textures: HashMap<String, wgpu::Texture>,
}

impl ClockContext {
    pub fn texture(&self, name: &str) -> &wgpu::Texture {
        self.textures
            .get(name)
            .unwrap_or_else(|| panic!("no texture `{name}` in assets/"))
    }
}

pub fn update_context(app: &App, ctx: &mut ClockContext) {
    let time = Local::now();
    ctx.time = time;
    ctx.sec_angle = (time.second() as f32 / 60.0) * TAU;
    ctx.min_angle = (time.minute() as f32 / 60.0) * TAU;
    ctx.hour_angle = ((time.hour() % 12) as f32 / 12.0) * TAU;
    ctx.dt = app.duration.since_prev_update.as_secs_f32();
    let win = app.window_rect();
    ctx.radius = win.w().min(win.h()) / 2.0 * 0.9; // leave some padding
}
