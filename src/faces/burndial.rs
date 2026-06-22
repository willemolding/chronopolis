use crate::Model;
use nannou::prelude::*;

pub fn render(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let size = f32::min(app.window_rect().w(), app.window_rect().h());
    let wh = vec2(size, size);

    draw.texture(&model.textures["Clock_BurndialMins - Itchigotchi2"])
        .xy(app.window_rect().xy())
        .wh(wh)
        .rotate(-model.min_hand.current_angle());

    draw.texture(&model.textures["Clock_BurndialBG - Itchigotchi2"])
        .xy(app.window_rect().xy())
        .wh(wh);

    draw.texture(&model.textures["Clock_BurndialSecs - Itchigotchi2"])
        .xy(app.window_rect().xy())
        .wh(wh)
        .rotate(-model.sec_hand.current_angle());

    draw.texture(&model.textures["Clock_BurndialHrs - Itchigotchi2"])
        .xy(app.window_rect().xy())
        .wh(wh)
        .rotate(-model.hour_hand.current_angle());

    draw.to_frame(app, &frame).unwrap();
}
