//! The handful of drawing helpers that macroquad doesn't already give us.

use std::f32::consts::TAU;

use macroquad::prelude::*;

/// Puts `(0, 0)` at the centre of the window
pub fn centred_camera() -> Camera2D {
    Camera2D {
        zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
        ..Default::default()
    }
}

/// Blacks out everything outside a circle of `radius` about the origin, out to
/// the corners of the window, so nothing is projected beyond the round face.
/// Only the area outside the circle is drawn: the GPU is fill-rate bound, so a
/// full-screen mask texture would cost a whole extra layer per frame.
pub fn draw_circle_mask(radius: f32) {
    const SEGMENTS: u16 = 256;
    // A one pixel ramp from clear to black stands in for the antialiasing
    // that multisampling can't give us on this GPU.
    const FEATHER: f32 = 1.0;
    let corner = vec2(screen_width(), screen_height()).length() / 2.0 + 1.0;

    let mut vertices = Vec::with_capacity(3 * SEGMENTS as usize);
    let mut indices = Vec::with_capacity(12 * SEGMENTS as usize);
    for i in 0..SEGMENTS {
        let angle = i as f32 / SEGMENTS as f32 * TAU;
        for (r, color) in [(radius - FEATHER, BLANK), (radius, BLACK), (corner, BLACK)] {
            let p = polar(angle, r);
            vertices.push(Vertex::new(p.x, p.y, 0.0, 0.0, 0.0, color));
        }
        // Two quads per segment: the feather, then the solid band outside it.
        let (here, next) = (i * 3, (i + 1) % SEGMENTS * 3);
        for ring in 0..2 {
            let (a, b, c, d) = (here + ring, here + ring + 1, next + ring, next + ring + 1);
            indices.extend_from_slice(&[a, b, c, b, d, c]);
        }
    }

    draw_mesh(&Mesh {
        vertices,
        indices,
        texture: None,
    });
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
