use nannou::ease;
use nannou::prelude::*;

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
    fn apply(self, t: f32) -> f32 {
        let (t, b, c, d) = (t as f64, 0.0, 1.0, 1.0);
        let v = match self {
            Easing::Instant => 1.0,
            Easing::Linear => t,
            Easing::Smooth => ease::sine::ease_in_out(t, b, c, d),
            Easing::Bounce => ease::bounce::ease_out(t, b, c, d),
            Easing::Elastic => ease::elastic::ease_out(t, b, c, d),
        };
        v as f32
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
