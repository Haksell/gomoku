pub mod alpha_beta_pruning;
pub mod idabp_new;
pub mod idabp_old;
pub mod minimax;
pub mod negamax;
pub mod random_mover;

use crate::{
    game::{Game, board::Position},
    heuristics::Heuristic,
};

pub type Bot = fn(&Game, Heuristic) -> Position;

pub fn parse_bot(s: &str) -> Result<Bot, String> {
    match s {
        "new" => Ok(idabp_new::idabp_new),
        "old" => Ok(idabp_old::idabp_old),
        "abp" | "alpha_beta_pruning" => Ok(alpha_beta_pruning::alpha_beta_pruning),
        "minimax" => Ok(minimax::minimax),
        "negamax" => Ok(negamax::negamax),
        "random_mover" | "random" => Ok(random_mover::random_mover),
        _ => Err(format!("Invalid bot: `{s}`")),
    }
}
