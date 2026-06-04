use crate::Model;
use nannou::prelude::*;

pub fn render(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // Clock face
    draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(model.radius)
        .color(rgb(0.15, 0.15, 0.2))
        .stroke(WHITE)
        .stroke_weight(2.0);

    // Hour tick marks
    for i in 0..12 {
        let angle = i as f32 / 12.0 * TAU;
        let (sin, cos) = angle.sin_cos();
        let inner = model.radius * 0.88;
        let outer = model.radius * 0.97;
        draw.line()
            .start(pt2(sin * inner, cos * inner))
            .end(pt2(sin * outer, cos * outer))
            .weight(2.5)
            .color(WHITE);
    }

    // Minute tick marks
    for i in 0..60 {
        if i % 5 == 0 {
            continue;
        } // skip hour positions
        let angle = i as f32 / 60.0 * TAU;
        let (sin, cos) = angle.sin_cos();
        let inner = model.radius * 0.93;
        let outer = model.radius * 0.97;
        draw.line()
            .start(pt2(sin * inner, cos * inner))
            .end(pt2(sin * outer, cos * outer))
            .weight(1.0)
            .color(GRAY);
    }

    // Hour hand
    let (s, c) = model.hour_angle.sin_cos();
    draw.line()
        .start(pt2(0.0, 0.0))
        .end(pt2(s * model.radius * 0.55, c * model.radius * 0.55))
        .weight(6.0)
        .caps_round()
        .color(WHITE);

    // Minute hand
    let (s, c) = model.min_angle.sin_cos();
    draw.line()
        .start(pt2(0.0, 0.0))
        .end(pt2(s * model.radius * 0.78, c * model.radius * 0.78))
        .weight(4.0)
        .caps_round()
        .color(WHITE);

    // Second hand
    let (s, c) = model.sec_angle.sin_cos();
    draw.line()
        .start(pt2(-s * model.radius * 0.15, -c * model.radius * 0.15)) // tail
        .end(pt2(s * model.radius * 0.88, c * model.radius * 0.88))
        .weight(1.5)
        .color(RED);

    // Centre dot
    draw.ellipse().x_y(0.0, 0.0).radius(4.0).color(RED);

    draw.to_frame(app, &frame).unwrap();
}
