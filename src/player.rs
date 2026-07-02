use crate::{
    bots::{Bot, alpha_beta_pruning::alpha_beta_pruning, parse_bot, random_mover::random_mover},
    game::Game,
    heuristics::{Heuristic, capturophile::capturophile, parse_heuristic, zero::zero},
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

    pub const fn is_bot(&self) -> bool {
        matches!(self, Self::Bot { .. })
    }
}

#[expect(clippy::fallible_impl_from)]
impl From<&str> for Player {
    fn from(v: &str) -> Self {
        match v {
            "human" => return Self::Human,
            "best" => return Self::Bot { bot: alpha_beta_pruning, heuristic: capturophile },
            "random" => return Self::Bot { bot: random_mover, heuristic: zero },
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

    pub const fn captures(self, game: &Game) -> usize {
        match self {
            Self::Black => game.black_captures,
            Self::White => game.white_captures,
        }
    }

    pub const fn increment_captures(self, game: &mut Game, captures: usize) {
        match self {
            Self::Black => game.black_captures += captures,
            Self::White => game.white_captures += captures,
        }
    }
}
