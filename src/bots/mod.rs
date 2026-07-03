pub mod alpha_beta_pruning;
pub mod minimax;
pub mod random_mover;

use crate::{
    game::{Game, board::Position},
    heuristics::Heuristic,
};

pub type Bot = fn(&Game, Heuristic) -> Position;

pub fn parse_bot(s: &str) -> Result<Bot, String> {
    match s {
        "abp" | "alpha_beta_pruning" => Ok(alpha_beta_pruning::alpha_beta_pruning),
        "minimax" => Ok(minimax::minimax),
        "random_mover" => Ok(random_mover::random_mover),
        _ => Err(format!("Invalid bot: `{s}`")),
    }
}
