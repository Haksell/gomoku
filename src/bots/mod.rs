pub mod abp_new;
pub mod abp_old;
pub mod minimax;
pub mod random_mover;

use crate::{
    game::{Game, board::Position},
    heuristics::Heuristic,
};

pub type Bot = fn(&Game, Heuristic) -> Position;

pub fn parse_bot(s: &str) -> Result<Bot, String> {
    match s {
        "new" => Ok(abp_new::abp_new),
        "old" => Ok(abp_old::abp_old),
        "minimax" => Ok(minimax::minimax),
        "random_mover" => Ok(random_mover::random_mover),
        _ => Err(format!("Invalid bot: `{s}`")),
    }
}
