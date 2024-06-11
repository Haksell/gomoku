use crate::{
    textures::{TEXTURE_BLACK, TEXTURE_WHITE},
    Model,
};
use nannou::wgpu::Texture;
use std::sync::MutexGuard;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Turn {
    None,
    Black,
    White,
}

impl Turn {
    pub fn opponent(&self) -> Self {
        match self {
            Turn::None => panic!("{self:?} doesn't have an opponent"),
            Turn::Black => Turn::White,
            Turn::White => Turn::Black,
        }
    }

    pub fn texture(&self) -> MutexGuard<'_, Texture> {
        match self {
            Turn::None => panic!("{self:?} doesn't have a texture"),
            Turn::Black => TEXTURE_BLACK.get().unwrap().lock().unwrap(),
            Turn::White => TEXTURE_WHITE.get().unwrap().lock().unwrap(),
        }
    }

    pub fn captures(&self, model: &Model) -> usize {
        match self {
            Turn::None => panic!("{self:?} doesn't have captures"),
            Turn::Black => model.black_captures,
            Turn::White => model.white_captures,
        }
    }

    pub fn increment_captures(&self, model: &mut Model, captures: usize) {
        match self {
            Turn::None => panic!("{self:?} doesn't have captures"),
            Turn::Black => model.black_captures += captures,
            Turn::White => model.white_captures += captures,
        };
    }
}
