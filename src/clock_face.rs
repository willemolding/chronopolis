use std::fmt::Debug;

use crate::prelude::*;

pub trait ClockFace: Debug {
    /// Shown in the switcher / logs.
    fn name(&self) -> &str;

    /// Per-frame state update. Default no-op for purely stateless faces.
    fn update(&mut self, _ctx: &ClockContext) {}

    /// Draw the face with macroquad calls. The origin is the centre of the
    /// window and `+y` points down, so positive angles turn clockwise.
    fn view(&self, ctx: &ClockContext);
}
