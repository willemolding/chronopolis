use crate::clock_context::update_context;
use crate::draw::centred_camera;
use crate::prelude::*;
use crate::textures::TextureStreamer;

mod clock_context;
mod clock_face;
mod clock_hand;
mod draw;
mod faces;
mod prelude;
mod textures;

#[derive(Debug)]
struct Model {
    ctx: ClockContext,
    faces: Vec<Box<dyn ClockFace>>,
    current: usize,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Chronopolis".to_owned(),
        fullscreen: true,
        high_dpi: false,
        sample_count: 1,
        window_width: 1024,
        window_height: 1024,
        platform: miniquad::conf::Platform {
            linux_x11_gl: miniquad::conf::LinuxX11Gl::EGLOnly,
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let textures = match textures::assets_path() {
        Some(root) => TextureStreamer::new(&root),
        None => {
            eprintln!("No `assets` directory found — faces with artwork will be blank.");
            TextureStreamer::default()
        }
    };

    let mut model = Model {
        ctx: ClockContext {
            textures,
            ..Default::default()
        },
        faces: faces::all(),
        current: 0,
    };

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        handle_input(&mut model);

        update_context(&mut model.ctx);

        // Only the showing face and the one after it are kept in memory, so the
        // next switch is instant.
        let next = (model.current + 1) % model.faces.len();
        let wanted: Vec<&str> = model.faces[model.current]
            .textures()
            .into_iter()
            .chain(model.faces[next].textures())
            .collect();
        model.ctx.textures.update(&wanted);

        // Every face updates, not just the visible one, so switching to a face
        // doesn't make its hands jump from wherever they were left.
        for face in &mut model.faces {
            face.update(&model.ctx);
        }

        clear_background(BLACK);
        set_camera(&centred_camera());
        model.faces[model.current].view(&model.ctx);
        set_default_camera();

        next_frame().await
    }
}

fn handle_input(model: &mut Model) {
    let count = model.faces.len();
    if count == 0 {
        return;
    }

    let previous = model.current;
    if is_key_pressed(KeyCode::Right) {
        model.current = (model.current + 1) % count;
    }
    if is_key_pressed(KeyCode::Left) {
        model.current = (model.current + count - 1) % count;
    }
    if model.current != previous {
        println!("Showing: {}", model.faces[model.current].name());
    }
}
