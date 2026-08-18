//! The handful of drawing helpers that macroquad doesn't already give us.

use macroquad::prelude::*;

/// Puts `(0, 0)` at the centre of the window
pub fn centred_camera() -> Camera2D {
    Camera2D {
        zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
        ..Default::default()
    }
}

pub fn polar(angle: f32, radius: f32) -> Vec2 {
    let (sin, cos) = angle.sin_cos();
    vec2(sin * radius, -cos * radius)
}

pub fn draw_line_round(from: Vec2, to: Vec2, thickness: f32, color: Color) {
    draw_line(from.x, from.y, to.x, to.y, thickness, color);
    draw_circle(from.x, from.y, thickness / 2.0, color);
    draw_circle(to.x, to.y, thickness / 2.0, color);
}

pub fn draw_texture_centred(texture: &Texture2D, centre: Vec2, size: Vec2, rotation: f32) {
    draw_texture_ex(
        texture,
        centre.x - size.x / 2.0,
        centre.y - size.y / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(size),
            rotation,
            ..Default::default()
        },
    );
}

pub fn draw_text_centred(text: &str, centre: Vec2, font_size: u16, color: Color) {
    let dims = measure_text(text, None, font_size, 1.0);
    draw_text(
        text,
        centre.x - dims.width / 2.0,
        centre.y + dims.offset_y - dims.height / 2.0,
        font_size as f32,
        color,
    );
}
