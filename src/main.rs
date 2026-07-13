use std::collections::HashMap;

use crate::{
    clock_context::{ClockContext, update_context},
    clock_face::ClockFace,
};
use nannou::prelude::*;

mod clock_context;
mod clock_face;
mod clock_hand;
mod faces;
mod textures;

#[derive(Debug, Default)]
struct Model {
    ctx: ClockContext,
    faces: Vec<Box<dyn ClockFace>>,
    current: usize,
}

fn model(app: &App) -> Model {
    app.new_window()
        .fullscreen_with(Some(Fullscreen::Borderless(None)))
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    println!("Loading textures...");
    let mut textures = HashMap::new();
    let root = app.assets_path().unwrap();
    textures::load_textures(app, &root, &root, &mut textures);

    Model {
        ctx: ClockContext {
            textures,
            ..Default::default()
        },
        faces: faces::all(),
        ..Default::default()
    }
}

fn update(app: &App, model: &mut Model, _u: Update) {
    update_context(app, &mut model.ctx); // time, angles, radius
    app.main_window()
        .set_title(model.faces[model.current].name());
    for face in &mut model.faces {
        face.update(app, &model.ctx);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.faces[model.current].view(app, &model.ctx, &draw);
    draw.to_frame(app, &frame).unwrap(); // faces can't forget to flush
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Right => model.current = (model.current + 1) % model.faces.len(),
        Key::Left => model.current = (model.current + model.faces.len() - 1) % model.faces.len(),
        Key::Escape => app.quit(),
        _ => {}
    }
}

fn main() {
    nannou::app(model).update(update).run()
}
