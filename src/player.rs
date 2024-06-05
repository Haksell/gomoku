use crate::{
    textures::{TEXTURE_BLACK, TEXTURE_WHITE},
    Model,
};
use nannou::wgpu::Texture;
use std::sync::MutexGuard;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    None,
    Black,
    White,
}

impl Player {
    pub fn opponent(&self) -> Self {
        match self {
            Player::None => panic!("{self:?} doesn't have an opponent"),
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    pub fn texture(&self) -> MutexGuard<'_, Texture> {
        match self {
            Player::None => panic!("{self:?} doesn't have a texture"),
            Player::Black => TEXTURE_BLACK.get().unwrap().lock().unwrap(),
            Player::White => TEXTURE_WHITE.get().unwrap().lock().unwrap(),
        }
    }

    pub fn captures(&self, model: &Model) -> usize {
        match self {
            Player::None => panic!("{self:?} doesn't have captures"),
            Player::Black => model.black_captures,
            Player::White => model.white_captures,
        }
    }

    pub fn increment_captures(&self, model: &mut Model, captures: usize) {
        match self {
            Player::None => panic!("{self:?} doesn't have captures"),
            Player::Black => model.black_captures += captures,
            Player::White => model.white_captures += captures,
        };
    }
}
