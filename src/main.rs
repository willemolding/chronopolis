use chrono::{DateTime, Local, Timelike};
use nannou::prelude::*;

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
}

fn model(_app: &App) -> Model {
    Default::default()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.time = Local::now();

    let now = &model.time;
    let sec = now.second() as f32 + now.nanosecond() as f32 / 1e9;
    let min = now.minute() as f32 + sec / 60.0;
    let hour = (now.hour() % 12) as f32 + min / 60.0;

    // Angles: 12 o'clock is up (cos), clockwise means +sin
    model.sec_angle = sec / 60.0 * TAU;
    model.min_angle = min / 60.0 * TAU;
    model.hour_angle = hour / 12.0 * TAU;

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
    faces::analog::render(app, model, frame);
}
