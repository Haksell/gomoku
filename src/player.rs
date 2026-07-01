use crate::{
    Model,
    bots::{Bot, DEFAULT_BOT, parse_bot},
    heuristics::{DEFAULT_HEURISTIC, Heuristic, parse_heuristic},
    textures::{TEXTURE_BLACK, TEXTURE_WHITE},
};
use itertools::Itertools as _;
use nannou::wgpu::Texture;
use std::ops::Not;

#[derive(Debug, Clone, Copy)]
pub enum Player {
    Human,
    Bot { bot: Bot, heuristic: Heuristic },
}

impl Player {
    pub const fn is_human(&self) -> bool {
        matches!(self, Self::Human)
    }
}

#[expect(clippy::fallible_impl_from)]
impl From<&str> for Player {
    fn from(v: &str) -> Self {
        match v {
            "human" => return Self::Human,
            "bot" => return Self::Bot { bot: DEFAULT_BOT, heuristic: DEFAULT_HEURISTIC },
            _ => {}
        }

        let words = v.split(':').collect_vec();
        let [bot_arg, heuristic_arg] = *words else { panic!("Invalid arg: {v}") };
        let bot = parse_bot(bot_arg).unwrap();
        let heuristic = parse_heuristic(heuristic_arg).unwrap();
        Self::Bot { bot, heuristic }
    }
}

// very sussy baka
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlayerColor {
    Black,
    White,
}

impl Not for PlayerColor {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

impl PlayerColor {
    pub fn texture(&self) -> &Texture {
        match self {
            Self::Black => TEXTURE_BLACK.get().unwrap(),
            Self::White => TEXTURE_WHITE.get().unwrap(),
        }
    }

    pub const fn captures(self, model: &Model) -> usize {
        match self {
            Self::Black => model.black_captures,
            Self::White => model.white_captures,
        }
    }

    pub const fn increment_captures(self, model: &mut Model, captures: usize) {
        match self {
            Self::Black => model.black_captures += captures,
            Self::White => model.white_captures += captures,
        }
    }
}
