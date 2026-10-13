use crate::clock_face::ClockFace;
use basic::{BasicFace, Layer::*};

pub mod analog;
pub mod basic;
pub mod clocktown;

pub fn all() -> Vec<Box<dyn ClockFace>> {
    vec![
        Box::new(clocktown::ClocktownFace::new()),
        Box::new(analog::AnalogFace::new()),
        Box::new(BasicFace::new("Scrimbles", "scrimbles")),
        // Community submissions. Layers are listed bottom first.
        Box::new(BasicFace::new("Purple Crystal", "purplecrystal").layers([Bg, Secs, Mins, Hours])),
        Box::new(BasicFace::new("Burnoff Time", "burnofftime").layers([Mins, Bg, Secs, Hours])),
        Box::new(
            BasicFace::new("Mushroom Forest", "mushroomforest").layers([Bg, Mins, Secs, Hours]),
        ),
        Box::new(BasicFace::new("Groovy Gaze", "groovygaze")),
        Box::new(BasicFace::new("Karl's Space Disco", "karlsspacedisco")),
        Box::new(BasicFace::new("Who is Mark Dofo?", "whoismarkdofo")),
        Box::new(BasicFace::new("Anama", "anama")),
        Box::new(BasicFace::new(
            "House of a Thousand Memories",
            "houseofathousandmemories",
        )),
        Box::new(BasicFace::new("Kelp Forest", "kelpforest").layers([Mins, Bg, Hours, Secs])),
        Box::new(BasicFace::new("Temple Time", "templetime").layers([Bg, Mins, Secs, Hours])),
        Box::new(BasicFace::new("Turbo Time", "turbotime")),
        Box::new(BasicFace::new("Glynis Mandala", "glynismandala").layers([Bg, Mins, Hours, Secs])),
        Box::new(BasicFace::new("Glynis Sisters", "glynissisters").layers([Bg, Secs, Mins, Hours])),
        Box::new(BasicFace::new("Glynis Peek", "glynispeek")),
        Box::new(BasicFace::new("Glynis Gumnuts", "glynisgumnuts")),
        Box::new(BasicFace::new("Glynis Van", "glynisvan")),
        Box::new(BasicFace::new("Glynis Cat", "glyniscat")),
        Box::new(BasicFace::new("Glynis Butterfly", "glynisbutterfly")),
        Box::new(BasicFace::new("Glynis Neil", "glynisneil")),
        Box::new(BasicFace::new("Glynis Flower", "glynisflower")),
        Box::new(BasicFace::new("Bun World", "bunworld")),
        Box::new(BasicFace::new("This Hurts My Eyes", "thishurtsmyeyes")),
        Box::new(BasicFace::new("Glynis Rabbit", "glynisrabbit")),
        Box::new(BasicFace::new("Glynis Wolf", "glyniswolf")),
        Box::new(BasicFace::new("Bubble of Time", "bubbleoftime")),
        Box::new(BasicFace::new("Old Clock", "oldclock")),
        Box::new(BasicFace::new("Tiramisu & Ted", "tiramisuted")),
        Box::new(BasicFace::new("Bubbles", "bubbles")),
    ]
}
