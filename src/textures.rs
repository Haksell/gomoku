use nannou::{wgpu::Texture, App};
use once_cell::sync::OnceCell;
use std::{path::Path, sync::Mutex};

pub static TEXTURE_BACKGROUND: OnceCell<Mutex<Texture>> = OnceCell::new();
pub static TEXTURE_BLACK: OnceCell<Mutex<Texture>> = OnceCell::new();
pub static TEXTURE_WHITE: OnceCell<Mutex<Texture>> = OnceCell::new();

const TEXTURES_DIRECTORY: &'static str = "assets";

fn handle_error<T, E: std::fmt::Debug>(result: Result<T, E>) -> T {
    result.unwrap_or_else(|err| {
        eprintln!("Failed to load texture: {err:?}");
        std::process::exit(1);
    })
}

fn init_texture(app: &App, texture_background: &OnceCell<Mutex<Texture>>, path: &str) {
    let texture_path = Path::new(TEXTURES_DIRECTORY).join(path);
    let texture = handle_error(Texture::from_path(app, texture_path));
    handle_error(texture_background.set(Mutex::new(texture)));
}

pub fn init_textures(app: &App) {
    init_texture(app, &TEXTURE_BACKGROUND, "forum.png");
    init_texture(app, &TEXTURE_BLACK, "black.png");
    init_texture(app, &TEXTURE_WHITE, "white.png");
}
