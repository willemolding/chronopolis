use std::collections::HashMap;

use nannou::{App, wgpu};

/// Recursively load every image under `dir`, keying each by its path relative
/// to `root` (without extension, `/`-separated).
pub fn load_textures(
    app: &App,
    root: &std::path::Path,
    dir: &std::path::Path,
    textures: &mut HashMap<String, wgpu::Texture>,
) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            load_textures(app, root, &path, textures);
        } else if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| matches!(e, "png" | "jpg" | "jpeg"))
        {
            let key = path
                .strip_prefix(root)
                .unwrap()
                .with_extension("")
                .to_string_lossy()
                .replace('\\', "/");
            println!("Loading: {}", key);
            let texture = wgpu::Texture::from_path(app, &path).unwrap();
            textures.insert(key, texture);
        }
    }
}
