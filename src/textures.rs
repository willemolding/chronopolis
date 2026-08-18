use std::collections::HashMap;
use std::path::{Path, PathBuf};

use macroquad::texture::{FilterMode, Texture2D, load_texture};

pub fn assets_path() -> Option<PathBuf> {
    if let Ok(cwd) = std::env::current_dir() {
        let candidate = cwd.join("assets");
        if candidate.is_dir() {
            return Some(candidate);
        }
    }

    let exe = std::env::current_exe().ok()?;
    let mut dir = exe.parent();
    while let Some(d) = dir {
        let candidate = d.join("assets");
        if candidate.is_dir() {
            return Some(candidate);
        }
        dir = d.parent();
    }
    None
}

fn is_image(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| matches!(e.to_ascii_lowercase().as_str(), "png" | "jpg" | "jpeg"))
}

pub async fn load_textures(root: &Path) -> HashMap<String, Texture2D> {
    let mut textures = HashMap::new();
    let mut dirs = vec![root.to_path_buf()];

    while let Some(dir) = dirs.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(err) => {
                eprintln!("Could not read {}: {err}", dir.display());
                continue;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
                continue;
            }
            if !is_image(&path) {
                continue;
            }

            let key = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .with_extension("")
                .to_string_lossy()
                .replace('\\', "/");

            let Some(path_str) = path.to_str() else {
                eprintln!("Skipping non-UTF-8 path: {}", path.display());
                continue;
            };

            println!("Loading: {key}");
            match load_texture(path_str).await {
                Ok(texture) => {
                    // macroquad defaults to nearest filtering, which aliases
                    // badly on rotated hand artwork.
                    texture.set_filter(FilterMode::Linear);
                    textures.insert(key, texture);
                }
                Err(err) => eprintln!("Failed to load {}: {err}", path.display()),
            }
        }
    }

    textures
}
