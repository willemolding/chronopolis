use crate::clock_face::ClockFace;

pub mod analog;
pub mod burndial;
pub mod clocktown;
pub mod purpcryst;
pub mod scrimbles;

pub fn all() -> Vec<Box<dyn ClockFace>> {
    vec![
        Box::new(analog::AnalogFace::new()),
        Box::new(scrimbles::ScrimblesFace::new()),
        Box::new(clocktown::ClocktownFace::new()),
        Box::new(burndial::BurndialFace::new()),
        Box::new(purpcryst::PurpcrystFace::new()),
    ]
}
