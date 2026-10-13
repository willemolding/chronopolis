use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

use macroquad::texture::{FilterMode, Image, Texture2D};

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

#[derive(Debug, Default)]
pub struct TextureStreamer {
    paths: HashMap<String, PathBuf>,
    resident: HashMap<String, Texture2D>,
    /// Missing or undecodable, so not retried every frame.
    failed: HashSet<String>,
    in_flight: Option<(String, Receiver<Result<Image, String>>)>,
}

impl TextureStreamer {
    /// Initialize a new texture streamer from a root path
    pub fn new(root: &Path) -> Self {
        let mut paths = HashMap::new();
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
                paths.insert(key, path);
            }
        }

        Self {
            paths,
            ..Default::default()
        }
    }

    pub fn get(&self, key: &str) -> Option<&Texture2D> {
        self.resident.get(key)
    }

    pub fn update(&mut self, wanted: &[&str]) {
        self.resident
            .retain(|key, _| wanted.contains(&key.as_str()));

        if let Some((key, rx)) = &self.in_flight {
            match rx.try_recv() {
                Err(TryRecvError::Empty) => return,
                Ok(Ok(image)) if wanted.contains(&key.as_str()) => {
                    let texture = Texture2D::from_image(&image);
                    texture.set_filter(FilterMode::Linear);
                    self.resident.insert(key.clone(), texture);
                }
                Ok(Ok(_)) => {}
                Ok(Err(err)) => {
                    eprintln!("Failed to load {key}: {err}");
                    self.failed.insert(key.clone());
                }
                Err(TryRecvError::Disconnected) => {
                    eprintln!("Loader for {key} died");
                    self.failed.insert(key.clone());
                }
            }
            self.in_flight = None;
        }

        let Some(&key) = wanted
            .iter()
            .find(|&&k| !self.resident.contains_key(k) && !self.failed.contains(k))
        else {
            return;
        };
        let Some(path) = self.paths.get(key).cloned() else {
            eprintln!("No artwork found for {key}");
            self.failed.insert(key.to_owned());
            return;
        };

        println!("Loading: {key}");
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let image = std::fs::read(&path)
                .map_err(|e| e.to_string())
                .and_then(|bytes| {
                    Image::from_file_with_format(&bytes, None).map_err(|e| e.to_string())
                });
            // The receiver is gone only if the streamer was dropped.
            let _ = tx.send(image);
        });
        self.in_flight = Some((key.to_owned(), rx));
    }
}
