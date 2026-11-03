use std::fmt::Debug;

use crate::prelude::*;
use crate::textures::TextureStreamer;

pub trait ClockFace: Debug {
    /// Shown in the switcher / logs.
    fn name(&self) -> &str;

    /// Per-frame state update. Default no-op for purely stateless faces.
    fn update(&mut self, _ctx: &ClockContext) {}

    /// Called once at startup with the artwork on disk, so a face can swap in
    /// stand-ins for anything its artist didn't supply.
    fn resolve_textures(&mut self, _textures: &TextureStreamer) {}

    /// Texture keys `view` draws, in load-priority order
    fn textures(&self) -> Vec<&str> {
        Vec::new()
    }

    /// Draw the face with macroquad calls. The origin is the centre of the
    /// window and `+y` points down, so positive angles turn clockwise.
    fn view(&self, ctx: &ClockContext);
}
