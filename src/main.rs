use crate::clock_context::update_context;
use crate::draw::centred_camera;
use crate::prelude::*;

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
        high_dpi: true,
        sample_count: 4,
        window_width: 1024,
        window_height: 1024,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("Loading textures...");
    let textures = match textures::assets_path() {
        Some(root) => textures::load_textures(&root).await,
        None => {
            eprintln!("No `assets` directory found — faces with artwork will be blank.");
            Default::default()
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
