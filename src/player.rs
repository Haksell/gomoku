use crate::Model;
use nannou::color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    None,
    Black,
    White,
}

impl Player {
    pub fn color(&self) -> color::Srgb<u8> {
        match self {
            Player::None => panic!("{self:?} doesn't have a color"),
            Player::Black => color::BLACK,
            Player::White => color::WHITE,
        }
    }

    pub fn opponent(&self) -> Self {
        match self {
            Player::None => panic!("{self:?} doesn't have an opponent"),
            Player::Black => Player::White,
            Player::White => Player::Black,
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
