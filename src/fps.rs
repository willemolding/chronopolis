//! Frame rate reporting, for tuning the installation on the target hardware.
//!
//! Off unless `CHRONOPOLIS_FPS` is set in the environment, because the clock
//! runs unattended for weeks and the log lands on a tmpfs. Set it in
//! `xinitrc` while tuning and take it back out afterwards.

use crate::prelude::*;

/// Averages the frame rate over a window and prints one line per window.
#[derive(Debug)]
pub struct FpsReporter {
    enabled: bool,
    frames: u32,
    elapsed: f32,
}

/// Long enough that a single slow frame does not dominate the average, short
/// enough to see the effect of a change without waiting around.
const WINDOW_SECONDS: f32 = 5.0;

impl FpsReporter {
    pub fn from_env() -> Self {
        Self {
            enabled: std::env::var_os("CHRONOPOLIS_FPS").is_some(),
            frames: 0,
            elapsed: 0.0,
        }
    }

    /// Call once per frame, after `update_context`.
    pub fn tick(&mut self, ctx: &ClockContext, face: &str) {
        if !self.enabled {
            return;
        }

        self.frames += 1;
        self.elapsed += ctx.dt;
        if self.elapsed < WINDOW_SECONDS {
            return;
        }

        // The canvas edge, not the window, is what the cost scales with: the
        // faces draw into the largest square that fits, and the rest of the
        // screen only gets cleared.
        println!(
            "{:.1} fps — {}x{} window, {:.0}px canvas, face {face}",
            self.frames as f32 / self.elapsed,
            screen_width(),
            screen_height(),
            ctx.canvas().x,
        );
        self.frames = 0;
        self.elapsed = 0.0;
    }
}
