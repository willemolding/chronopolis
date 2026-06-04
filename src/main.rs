use std::collections::HashMap;

use chrono::{DateTime, Local, Timelike};
use nannou::{
    ease::{bounce, cubic},
    prelude::*,
};

mod faces;

const WINDOW_SIZE: u32 = 512;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .run()
}

#[derive(Debug, Default)]
struct Model {
    /// The system local datetime. This is the canonical time and should be used if you want to display the correct time
    time: DateTime<Local>,
    /// Helper for analog clock faces, the angle of the second hand in radians, where 12 o'clock is 0 and angles increase clockwise
    sec_angle: f32,
    /// Helper for analog clock faces, the angle of the minute hand in radians, where 12 o'clock is 0 and angles increase clockwise
    min_angle: f32,
    /// Helper for analog clock faces, the angle of the hour hand in radians, where 12 o'clock is 0 and angles increase clockwise
    hour_angle: f32,
    /// Radius of the circular clock face canvas. Anything outside this radius from the center will be clipped
    radius: f32,
    /// All textures in the asset directory, keyed by filename stem (e.g. "clocktown" for "clocktown.png")
    textures: HashMap<String, wgpu::Texture>,

    sec_hand: ClockHand,
    min_hand: ClockHand,
    hour_hand: ClockHand,
    last_sec: u32,
    last_min: u32,
    last_hour: u32,
}

fn model(app: &App) -> Model {
    // Load all textures in the assets folder
    println!("Loading textures...");
    let mut textures = HashMap::new();
    for entry in std::fs::read_dir(&app.assets_path().unwrap()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if matches!(ext.to_str(), Some("png") | Some("jpg") | Some("jpeg")) {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        println!("Loading: {}", stem);
                        let texture = wgpu::Texture::from_path(app, &path).unwrap();
                        textures.insert(stem.to_string(), texture);
                    }
                }
            }
        }
    }

    Model {
        textures,
        ..Default::default()
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.time = Local::now();

    let now = chrono::Local::now();
    let current_min = now.minute();
    let current_hour = now.hour();
    let current_sec = now.second();

    let now = &model.time;
    let sec = now.second() as f32 + now.nanosecond() as f32 / 1e9;
    let min = now.minute() as f32 + sec / 60.0;
    let hour = (now.hour() % 12) as f32 + min / 60.0;

    // Angles: 12 o'clock is up (cos), clockwise means +sin
    model.sec_angle = sec / 60.0 * TAU;
    model.min_angle = min / 60.0 * TAU;
    model.hour_angle = hour / 12.0 * TAU;

    if current_sec != model.last_sec {
        model.last_sec = current_sec;
        model.sec_hand.tick((sec / 60.0) * TAU);
    }

    if current_min != model.last_min {
        model.last_min = current_min;
        model.min_hand.tick((min / 60.0) * TAU);
    }

    if current_hour != model.last_hour {
        model.last_hour = current_hour;
        model
            .hour_hand
            .tick(((current_hour % 12) as f32 / 12.0) * TAU);
    }

    model.sec_hand.update(0.025);
    model.min_hand.update(0.025);
    model.hour_hand.update(0.025);

    model.radius = (WINDOW_SIZE as f32) * 0.4;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // border
    draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(model.radius)
        .no_fill()
        .stroke(WHITE)
        .stroke_weight(1.0);

    // Can add logic here for selecting different clock faces
    faces::clocktown::render(app, model, frame);
}

#[derive(Default, Debug)]
pub struct ClockHand {
    pub from: f32,
    pub target: f32,
    pub t: f32,
}

impl ClockHand {
    pub fn new(angle: f32) -> Self {
        Self {
            from: angle,
            target: angle,
            t: 1.0,
        }
    }

    pub fn tick(&mut self, new_target: f32) {
        let current = self.current_angle();
        let delta = shortest_angle_delta(current, new_target);
        self.from = current;
        self.target = current + delta; // target expressed relative to current, no wrap
        self.t = 0.0;
    }

    pub fn update(&mut self, speed: f32) {
        if self.t < 1.0 {
            self.t = (self.t + speed).min(1.0);
        }
    }

    pub fn current_angle(&self) -> f32 {
        bounce::ease_out(
            self.t as f64,
            self.from as f64,
            (self.target - self.from) as f64,
            1.0,
        ) as f32
    }
}

fn shortest_angle_delta(from: f32, to: f32) -> f32 {
    let delta = (to - from).rem_euclid(TAU);
    if delta > PI { delta - TAU } else { delta }
}
