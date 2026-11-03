use crate::clock_context::update_context;
use crate::draw::{centred_camera, draw_circle_mask};
use crate::prelude::*;
use crate::textures::TextureStreamer;
use macroquad::rand::{ChooseRandom, RandGenerator};

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
    last_switch: f64,
}

const AUTO_SWITCH_SECS: f64 = 30.0;

/// Something unique to this board, so every clock shows the faces in its own
/// order. The Pi's serial number first; `machine-id` covers other Linux hosts.
fn device_id() -> Option<String> {
    let serial = std::fs::read_to_string("/sys/firmware/devicetree/base/serial-number")
        .ok()
        .or_else(|| {
            let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").ok()?;
            let line = cpuinfo.lines().find(|l| l.starts_with("Serial"))?;
            Some(line.split(':').nth(1)?.to_owned())
        })
        .or_else(|| std::fs::read_to_string("/etc/machine-id").ok())?;
    let serial = serial.trim_matches(|c: char| c.is_whitespace() || c == '\0');
    (!serial.is_empty()).then(|| serial.to_owned())
}

/// FNV-1a, rather than std's hasher, so a given board keeps the same order
/// across rebuilds.
fn seed_from(id: &str) -> u64 {
    id.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ byte as u64).wrapping_mul(0x0100_0000_01b3)
    })
}

fn shuffled_faces() -> Vec<Box<dyn ClockFace>> {
    let id = device_id();
    println!("Face order seeded from device id {id:?}");
    // A generator of its own, so faces using macroquad's global one can't
    // disturb the order.
    let rng = RandGenerator::new();
    rng.srand(id.as_deref().map_or(0, seed_from));
    let mut faces = faces::all();
    faces.shuffle_with_state(&rng);
    faces
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
        faces: shuffled_faces(),
        current: 0,
        last_switch: get_time(),
    };
    for face in &mut model.faces {
        face.resolve_textures(&model.ctx.textures);
    }

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
        // Faces are free to draw past the edge; the mask keeps them round.
        draw_circle_mask(model.ctx.canvas().x / 2.0);
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
    let now = get_time();
    if is_key_pressed(KeyCode::Right) || now - model.last_switch >= AUTO_SWITCH_SECS {
        model.current = (model.current + 1) % count;
    }
    if is_key_pressed(KeyCode::Left) {
        model.current = (model.current + count - 1) % count;
    }
    if model.current != previous {
        model.last_switch = now;
        println!("Showing: {}", model.faces[model.current].name());
    }
}
