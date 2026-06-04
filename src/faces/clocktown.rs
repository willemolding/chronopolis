use crate::Model;
use nannou::prelude::*;

pub fn render(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.texture(&model.textures["clocktown-border"])
        .xy(app.window_rect().xy())
        .wh(app.window_rect().wh())
        .rotate(-model.sec_hand.current_angle());

    draw.texture(&model.textures["clocktown-center"])
        .xy(app.window_rect().xy())
        .wh(app.window_rect().wh())
        .rotate(-model.min_hand.current_angle());

    draw.texture(&model.textures["clocktown-inner-dial-sun"])
        .xy(app.window_rect().xy())
        .wh(app.window_rect().wh())
        .rotate(-model.min_hand.current_angle());

    draw.to_frame(app, &frame).unwrap();
}
