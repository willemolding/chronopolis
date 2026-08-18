use std::collections::HashMap;

use chrono::{DateTime, Local, Timelike};

use crate::prelude::*;

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
    pub textures: HashMap<String, Texture2D>,
}

impl ClockContext {
    pub fn texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }

    /// The largest square that fits in the window
    pub fn canvas(&self) -> Vec2 {
        Vec2::splat(screen_width().min(screen_height()))
    }
}

pub fn update_context(ctx: &mut ClockContext) {
    let time = Local::now();
    ctx.time = time;
    ctx.sec_angle = (time.second() as f32 / 60.0) * TAU;
    ctx.min_angle = (time.minute() as f32 / 60.0) * TAU;
    ctx.hour_angle = ((time.hour() % 12) as f32 / 12.0) * TAU;
    ctx.dt = get_frame_time();
    ctx.radius = screen_width().min(screen_height()) / 2.0 * 0.9; // leave some padding
}
