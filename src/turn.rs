use crate::{
    Model,
    textures::{TEXTURE_BLACK, TEXTURE_WHITE},
};
use nannou::wgpu::Texture;

// very sussy baka
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Turn {
    None,
    Black,
    White,
}

impl Turn {
    pub fn opponent(self) -> Self {
        match self {
            Self::None => panic!("{self:?} doesn't have an opponent"),
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    pub fn texture(&self) -> &Texture {
        match self {
            Self::None => panic!("{self:?} doesn't have a texture"),
            Self::Black => TEXTURE_BLACK.get().unwrap(),
            Self::White => TEXTURE_WHITE.get().unwrap(),
        }
    }

    pub fn captures(self, model: &Model) -> usize {
        match self {
            Self::None => panic!("{self:?} doesn't have captures"),
            Self::Black => model.black_captures,
            Self::White => model.white_captures,
        }
    }

    pub fn increment_captures(self, model: &mut Model, captures: usize) {
        match self {
            Self::None => panic!("{self:?} doesn't have captures"),
            Self::Black => model.black_captures += captures,
            Self::White => model.white_captures += captures,
        }
    }
}
