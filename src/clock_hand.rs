// Presets are offered to face authors whether or not a face in this repo
// currently picks them.
#![allow(dead_code)]

use crate::prelude::*;

/// How a hand interpolates toward a new target. All variants finish on target;
/// `Bounce` and `Elastic` overshoot along the way.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Easing {
    Instant,
    Linear,
    #[default]
    Smooth,
    Bounce,
    Elastic,
}

impl Easing {
    /// Map linear progress in `0.0..=1.0` onto the eased curve. Every variant
    /// maps 0 to 0 and 1 to 1.
    fn apply(self, t: f32) -> f32 {
        match self {
            Easing::Instant => 1.0,
            Easing::Linear => t,
            Easing::Smooth => sine_in_out(t),
            Easing::Bounce => bounce_out(t),
            Easing::Elastic => elastic_out(t),
        }
    }
}

fn sine_in_out(t: f32) -> f32 {
    -((PI * t).cos() - 1.0) / 2.0
}

fn bounce_out(t: f32) -> f32 {
    const N: f32 = 7.5625;
    const D: f32 = 2.75;
    if t < 1.0 / D {
        N * t * t
    } else if t < 2.0 / D {
        let t = t - 1.5 / D;
        N * t * t + 0.75
    } else if t < 2.5 / D {
        let t = t - 2.25 / D;
        N * t * t + 0.9375
    } else {
        let t = t - 2.625 / D;
        N * t * t + 0.984375
    }
}

fn elastic_out(t: f32) -> f32 {
    if t <= 0.0 {
        0.0
    } else if t >= 1.0 {
        1.0
    } else {
        2f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * (TAU / 3.0)).sin() + 1.0
    }
}

/// A single animated clock hand. Configure it with a preset or the builder
/// methods, drive it once per frame with [`ClockHand::animate_to`], and read
/// the displayed angle with [`ClockHand::angle`].
#[derive(Clone, Copy, Debug)]
pub struct ClockHand {
    easing: Easing,
    duration: f32,
    goal: f32,
    from: f32,
    to: f32,
    elapsed: f32,
}

impl Default for ClockHand {
    fn default() -> Self {
        Self::new()
    }
}

impl ClockHand {
    pub fn new() -> Self {
        Self {
            easing: Easing::Smooth,
            duration: 0.3,
            goal: f32::NAN, // first `animate_to` snaps instead of sweeping
            from: 0.0,
            to: 0.0,
            elapsed: 0.0,
        }
    }

    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    pub fn duration(mut self, seconds: f32) -> Self {
        self.duration = seconds;
        self
    }

    pub fn instant() -> Self {
        Self::new().easing(Easing::Instant).duration(0.0)
    }

    pub fn sweeping() -> Self {
        Self::new().easing(Easing::Linear).duration(1.0)
    }

    pub fn smooth() -> Self {
        Self::new().easing(Easing::Smooth).duration(0.3)
    }

    pub fn bouncy() -> Self {
        Self::new().easing(Easing::Bounce).duration(0.8)
    }

    pub fn springy() -> Self {
        Self::new().easing(Easing::Elastic).duration(1.0)
    }

    /// Point the hand at `target` (radians) and advance by `dt` seconds. Call
    /// once per frame; restarts the easing only when `target` changes.
    pub fn animate_to(&mut self, target: f32, dt: f32) {
        if self.goal.is_nan() {
            self.goal = target;
            self.from = target;
            self.to = target;
        } else if target != self.goal {
            self.goal = target;
            self.from = self.angle();
            self.to = self.from + shortest_angle_delta(self.from, target);
            self.elapsed = 0.0;
        }
        self.elapsed += dt;
    }

    pub fn angle(&self) -> f32 {
        let progress = if self.duration <= f32::EPSILON {
            1.0
        } else {
            (self.elapsed / self.duration).clamp(0.0, 1.0)
        };
        self.from + (self.to - self.from) * self.easing.apply(progress)
    }
}

/// Shortest signed angular distance from `from` to `to`, in `(-PI, PI]`.
fn shortest_angle_delta(from: f32, to: f32) -> f32 {
    let delta = (to - from).rem_euclid(TAU);
    if delta > PI { delta - TAU } else { delta }
}
