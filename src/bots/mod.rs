pub mod alpha_beta_pruning;
pub mod idabp;
pub mod minimax;
pub mod negamax;
pub mod random_mover;

use crate::{
    game::{Game, board::Position, state::GameState},
    heuristics::Heuristic,
    player::PlayerColor,
};
use std::time::Duration;

pub type Bot = fn(&Game, &Heuristic) -> Position;

pub fn parse_bot(s: &str) -> Result<Bot, String> {
    match s {
        "random" | "random_mover" => Ok(random_mover::random_mover),
        "minimax" => Ok(minimax::minimax),
        "negamax" => Ok(negamax::negamax),
        "abp" | "alpha_beta_pruning" => Ok(alpha_beta_pruning::alpha_beta_pruning),
        "idabp" => Ok(idabp::idabp),
        _ => Err(format!("Invalid bot: `{s}`")),
    }
}

// TODO: different max_dist and number of best moves to check depending on depth
pub const MAX_DEPTH: usize = 10; // TODO: flag with default value of 10
pub const TIME_LIMIT: Duration = Duration::from_millis(128); // TODO: flag with default value of 500ms

/// Maximizes for the current player, not necessarily black.
fn leaf_value(game: &Game, heuristic: &Heuristic, depth: usize, max_depth: usize) -> Option<i64> {
    let leaf_value = match game.state {
        GameState::Playing(_) => {
            (depth == max_depth).then(|| (heuristic.fun)(game, heuristic.coeffs.as_ref()))
        }
        GameState::Draw => Some(0),
        GameState::Won(PlayerColor::Black, _) => Some(i64::MAX - depth as i64),
        GameState::Won(PlayerColor::White, _) => Some(depth as i64 - i64::MAX),
    };

    leaf_value.map(|leaf_value| match game.current_color {
        PlayerColor::Black => leaf_value,
        PlayerColor::White => -leaf_value,
    })
}
