use crate::clock_face::ClockFace;
use basic::{BasicFace, Layer::*};

pub mod analog;
pub mod basic;
pub mod clocktown;

pub fn all() -> Vec<Box<dyn ClockFace>> {
    vec![
        Box::new(clocktown::ClocktownFace::new()),
        Box::new(BasicFace::new("Scrimbles", "scrimbles")),
        // Community submissions. Layers are listed bottom first.
        Box::new(BasicFace::new("Purple Crystal", "purplecrystal").layers([Bg, Secs, Mins, Hours])),
        Box::new(
            BasicFace::new("Burnoff Time", "burnofftime")
                .layers([Mins, Bg, Secs, Hours])
                .sweeping(),
        ),
        Box::new(
            BasicFace::new("Mushroom Forest", "mushroomforest")
                .layers([Bg, Mins, Secs, Hours])
                .sweeping(),
        ),
        Box::new(BasicFace::new("Groovy Gaze", "groovygaze")),
        Box::new(BasicFace::new("Karl's Space Disco", "karlsspacedisco")),
        Box::new(BasicFace::new("Who is Mark Dofo?", "whoismarkdofo")),
        Box::new(BasicFace::new("Anama", "anama")),
        Box::new(BasicFace::new(
            "House of a Thousand Memories",
            "houseofathousandmemories",
        )),
        Box::new(
            BasicFace::new("Kelp Forest", "kelpforest")
                .layers([Mins, Bg, Hours, Secs])
                .sweeping(),
        ),
        Box::new(
            BasicFace::new("Temple Time", "templetime")
                .layers([Bg, Mins, Secs, Hours])
                .sweeping(),
        ),
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
        Box::new(BasicFace::new("Questionable", "questionable")),
        Box::new(BasicFace::new("Bubbles 2", "bubbles2")),
        Box::new(BasicFace::new("Temple 2025", "temple2025")),
        Box::new(BasicFace::new("Cave Flower Power", "caveflowerpower")),
        Box::new(BasicFace::new("Cloud Bow", "cloudbow")),
        Box::new(BasicFace::new("Aurora Time!", "auroratime")),
        Box::new(BasicFace::new(
            "Little Flame Robin Time",
            "littleflamerobintime",
        )),
        Box::new(BasicFace::new("NICE Time", "nicetime")),
        Box::new(BasicFace::new("Time Fatigue", "timefatigue")),
        Box::new(BasicFace::new("Pocket(watch) Dimension", "pocketwatchdimension").sweeping()),
        Box::new(BasicFace::new("Aurora Australis", "auroraaustralis").sweeping()),
        Box::new(BasicFace::new("Mandala", "mandala")),
        Box::new(BasicFace::new("Glynis 1", "glynis1")),
        Box::new(BasicFace::new("Glynis 2", "glynis2")),
        Box::new(BasicFace::new("Glynis 3", "glynis3")),
        Box::new(BasicFace::new("Glynis 4", "glynis4")),
        Box::new(BasicFace::new("Glynis 5", "glynis5")),
        Box::new(BasicFace::new("Glynis 6", "glynis6")),
        Box::new(BasicFace::new("Glynis 7", "glynis7")),
        Box::new(BasicFace::new("Glynis 8", "glynis8")),
        Box::new(BasicFace::new("Glynis 9", "glynis9")),
        Box::new(BasicFace::new("Glynis 10", "glynis10")),
        Box::new(BasicFace::new("Glynis 11", "glynis11")),
        Box::new(BasicFace::new("Glynis 12", "glynis12")),
        Box::new(BasicFace::new("Glynis 13", "glynis13")),
        Box::new(BasicFace::new("Glynis 14", "glynis14")),
        Box::new(BasicFace::new("Glynis 15", "glynis15")),
        Box::new(BasicFace::new("Glynis 16", "glynis16")),
        Box::new(BasicFace::new("Glynis 17", "glynis17")),
        Box::new(BasicFace::new("Glynis 18", "glynis18")),
        Box::new(BasicFace::new("Glynis 19", "glynis19")),
        Box::new(BasicFace::new("Glynis 20", "glynis20")),
        Box::new(BasicFace::new("Glynis 21", "glynis21")),
        Box::new(BasicFace::new("Glynis 22", "glynis22")),
        Box::new(BasicFace::new("Glynis 23", "glynis23")),
        Box::new(BasicFace::new("Glynis 24", "glynis24")),
        Box::new(BasicFace::new("Glynis 25", "glynis25")),
        Box::new(BasicFace::new("Glynis 26", "glynis26")),
        Box::new(BasicFace::new("Glynis 27", "glynis27")),
        Box::new(BasicFace::new("Glynis 28", "glynis28")),
        Box::new(BasicFace::new("Glynis 29", "glynis29")),
    ]
}
