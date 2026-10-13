//! Everything a clock face needs, in one import:
//!
//! ```ignore
//! use crate::prelude::*;
//! ```

pub use std::f32::consts::{PI, TAU};

pub use macroquad::prelude::*;

pub use crate::clock_context::ClockContext;
pub use crate::clock_face::ClockFace;
// Offered to face authors whether or not a face in this repo uses it.
#[allow(unused_imports)]
pub use crate::clock_hand::{ClockHand, Easing};
pub use crate::draw::{draw_line_round, draw_text_centred, draw_texture_centred, polar};
