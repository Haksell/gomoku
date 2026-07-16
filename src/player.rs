use crate::{
    bots::{Bot, idabp::idabp, parse_bot, random_mover::random_mover},
    heuristics::{Heuristic, parse_heuristic},
};
use itertools::Itertools as _;
use std::{ops::Not, ptr::fn_addr_eq};

#[derive(Debug, Clone)]
pub enum Player {
    Human,
    Bot { bot: Bot, heuristic: Heuristic },
}

impl Player {
    pub const RANDOM: Self = Self::Bot { bot: random_mover, heuristic: Heuristic::ZERO };
    pub const MANUAL: Self = Self::Bot { bot: idabp, heuristic: Heuristic::MANUAL };

    fn coeffistic() -> Self {
        Self::Bot { bot: idabp, heuristic: Heuristic::coeffistic() }
    }

    pub const fn is_human(&self) -> bool {
        matches!(self, Self::Human)
    }

    pub const fn is_bot(&self) -> bool {
        matches!(self, Self::Bot { .. })
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Bot {
                    bot: l_bot,
                    heuristic: Heuristic { fun: l_heuristic, coeffs: l_coeffs },
                },
                Self::Bot {
                    bot: r_bot,
                    heuristic: Heuristic { fun: r_heuristic, coeffs: r_coeffs },
                },
            ) => {
                fn_addr_eq(*l_bot, *r_bot)
                    && fn_addr_eq(*l_heuristic, *r_heuristic)
                    && l_coeffs == r_coeffs
            }
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[expect(clippy::fallible_impl_from)]
impl From<&str> for Player {
    fn from(v: &str) -> Self {
        match v {
            "human" => Self::Human,
            "random" => Self::RANDOM,
            "manual" => Self::MANUAL,
            "coeffistic" => Self::coeffistic(),
            _ => {
                let words = v.split(':').collect_vec();
                let [bot_arg, heuristic_arg] = *words else { panic!("Invalid arg: {v}") };
                let bot = parse_bot(bot_arg).unwrap();
                let heuristic = parse_heuristic(heuristic_arg).unwrap();
                Self::Bot { bot, heuristic }
            }
        }
    }
}

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
