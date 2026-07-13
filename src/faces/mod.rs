use crate::clock_face::ClockFace;
use basic::BasicFace;

pub mod analog;
pub mod basic;
pub mod clocktown;

pub fn all() -> Vec<Box<dyn ClockFace>> {
    vec![
        Box::new(analog::AnalogFace::new()),
        Box::new(clocktown::ClocktownFace::new()),
        Box::new(BasicFace::new("Burndial", "burndial")),
        Box::new(BasicFace::new("Kelp Forest", "kelpforest")),
        Box::new(BasicFace::new("Sisters", "sisters")),
        Box::new(BasicFace::new("Scrimbles", "scrimbles")),
    ]
}
