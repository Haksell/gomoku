use crate::{
    bots::{Bot, alpha_beta_pruning::alpha_beta_pruning, parse_bot, random_mover::random_mover},
    heuristics::{Heuristic, new::new, old::old, parse_heuristic, zero::zero},
};
use itertools::Itertools as _;
use std::{ops::Not, ptr::fn_addr_eq};

#[derive(Debug, Clone, Copy)]
pub enum Player {
    Human,
    Bot { bot: Bot, heuristic: Heuristic },
}

impl Player {
    pub const RANDOM: Self = Self::Bot { bot: random_mover, heuristic: zero };
    pub const OLD: Self = Self::Bot { bot: alpha_beta_pruning, heuristic: old }; // TODO: abp_old
    pub const NEW: Self = Self::Bot { bot: alpha_beta_pruning, heuristic: new }; // TODO: abp_new

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
                Self::Bot { bot: l_bot, heuristic: l_heuristic },
                Self::Bot { bot: r_bot, heuristic: r_heuristic },
            ) => fn_addr_eq(*l_bot, *r_bot) && fn_addr_eq(*l_heuristic, *r_heuristic),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[expect(clippy::fallible_impl_from)]
impl From<&str> for Player {
    fn from(v: &str) -> Self {
        match v {
            "human" => Self::Human,
            "old" => Self::OLD,
            "new" => Self::NEW,
            "random" => Self::RANDOM,
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
