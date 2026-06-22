use std::fmt::Debug;

use nannou::prelude::*;

use crate::ClockContext;

pub trait ClockFace: Debug {
    /// Shown in the switcher / logs.
    fn name(&self) -> &str;

    /// Per-frame state update. Default no-op for purely stateless faces.
    fn update(&mut self, _app: &App, _ctx: &ClockContext) {}

    /// Draw the face. `draw` is pre-made and flushed for you — just draw.
    fn view(&self, app: &App, ctx: &ClockContext, draw: &Draw);
}
