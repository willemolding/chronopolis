use crate::clock_face::ClockFace;

pub mod analog;
pub mod burndial;
pub mod clocktown;
pub mod kelpforest;
pub mod mandala;
pub mod purpcryst;
pub mod scrimbles;
pub mod sisters;

pub fn all() -> Vec<Box<dyn ClockFace>> {
    vec![
        Box::new(analog::AnalogFace::new()),
        Box::new(scrimbles::ScrimblesFace::new()),
        Box::new(clocktown::ClocktownFace::new()),
        Box::new(burndial::BurndialFace::new()),
        Box::new(purpcryst::PurpcrystFace::new()),
        Box::new(kelpforest::KelpforestFace::new()),
        Box::new(mandala::MandalaFace::new()),
        Box::new(sisters::SistersFace::new()),
    ]
}
