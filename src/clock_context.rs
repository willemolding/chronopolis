use chrono::{DateTime, Local, Timelike};

use crate::prelude::*;
use crate::textures::TextureStreamer;

/// Read-only, host-computed data handed to every face each frame.
#[derive(Debug, Default)]
pub struct ClockContext {
    pub time: DateTime<Local>,
    /// Raw hand angles (radians, 12 o'clock = 0, clockwise positive), whole
    /// units only, so they tick. Feed these to an eased `ClockHand`.
    pub sec_angle: f32,
    pub min_angle: f32,
    pub hour_angle: f32,
    /// The same angles including the fraction of the unit, so they move
    /// continuously. Draw these directly for a sweeping hand: they come from the
    /// clock rather than summed frame times, so a late frame can't make the
    /// hand stutter.
    pub sec_sweep: f32,
    pub min_sweep: f32,
    pub hour_sweep: f32,
    /// Radius of the circular canvas; draw outside this and it's off-face.
    pub radius: f32,
    /// Seconds elapsed since the previous frame. Pass to `ClockHand::animate_to`.
    pub dt: f32,
    pub textures: TextureStreamer,
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
    // Steps with each minute, as a real hour hand creeps toward the next hour.
    ctx.hour_angle = (((time.hour() % 12) as f32 + time.minute() as f32 / 60.0) / 12.0) * TAU;

    // `nanosecond` exceeds 1e9 during a leap second; clamp so the hand waits.
    let secs = time.second() as f32 + (time.nanosecond() as f32 / 1e9).min(1.0);
    let mins = time.minute() as f32 + secs / 60.0;
    let hours = (time.hour() % 12) as f32 + mins / 60.0;
    ctx.sec_sweep = (secs / 60.0) * TAU;
    ctx.min_sweep = (mins / 60.0) * TAU;
    ctx.hour_sweep = (hours / 12.0) * TAU;
    ctx.dt = get_frame_time();
    ctx.radius = screen_width().min(screen_height()) / 2.0 * 0.9; // leave some padding
}
