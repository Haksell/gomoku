use nannou::{App, wgpu::Texture};
use std::{
    path::Path,
    sync::{Mutex, OnceLock},
};

pub static TEXTURE_BACKGROUND: OnceLock<Mutex<Texture>> = OnceLock::new();
pub static TEXTURE_BLACK: OnceLock<Mutex<Texture>> = OnceLock::new();
pub static TEXTURE_WHITE: OnceLock<Mutex<Texture>> = OnceLock::new();

const TEXTURES_DIRECTORY: &str = "assets";

fn handle_error<T, E: core::fmt::Debug>(result: Result<T, E>) -> T {
    result.unwrap_or_else(|err| {
        panic!("Failed to load texture: {err:?}");
    })
}

fn init_texture(app: &App, texture_background: &OnceLock<Mutex<Texture>>, path: &str) {
    let texture_path = Path::new(TEXTURES_DIRECTORY).join(path);
    let texture = handle_error(Texture::from_path(app, texture_path));
    handle_error(texture_background.set(Mutex::new(texture)));
}

pub fn init_textures(app: &App) {
    init_texture(app, &TEXTURE_BACKGROUND, "forum.png");
    init_texture(app, &TEXTURE_BLACK, "black.png");
    init_texture(app, &TEXTURE_WHITE, "white.png");
}
