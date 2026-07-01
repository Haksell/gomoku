use super::get_close_moves;
use crate::{
    constants::BOARD_CENTER,
    heuristics::Heuristic,
    model::{Model, Position},
    player::PlayerColor,
};
use std::cmp::{max, min};

// TODO: struct with distance and number of moves
const DFS: &[Position] = &[(1, usize::MAX), (1, usize::MAX), (1, usize::MAX)];
const MAX_DEPTH: usize = DFS.len();

pub fn alpha_beta_pruning(model: &Model, heuristic: Heuristic) -> Position {
    if model.moves.is_empty() {
        return BOARD_CENTER;
    }
    let mut best_move = (usize::MAX, usize::MAX);
    alpha_beta_pruning_helper(
        model,
        model.current_color,
        heuristic,
        0,
        i64::MIN,
        i64::MAX,
        &mut best_move,
    );
    best_move
}

fn alpha_beta_pruning_helper(
    model: &Model,
    current_player: PlayerColor,
    heuristic: Heuristic,
    depth: usize,
    mut min_score: i64,
    mut max_score: i64,
    best_move: &mut Position,
) -> i64 {
    if let Some(winner) = model.winner {
        return if winner == current_player { i64::MAX } else { i64::MIN };
    }
    if depth == MAX_DEPTH {
        return heuristic(model);
    }

    // TODO: sort by depth 1 heuristic
    let close_moves = get_close_moves(model, DFS[depth].0, true);
    debug_assert!(!close_moves.is_empty());

    let is_maximizing_player = depth & 1 == 0;
    let mut best_score = if is_maximizing_player { i64::MIN } else { i64::MAX };

    // TODO: sort by depth 1 heuristic
    let a = &close_moves[0..(DFS[depth].1).min(close_moves.len())];
    for &(x, y) in a {
        // TODO: do_move then undo_move
        let mut model = model.clone();
        model.do_move(x, y);
        let score = alpha_beta_pruning_helper(
            &model,
            current_player,
            heuristic,
            depth + 1,
            min_score,
            max_score,
            best_move,
        );
        if is_maximizing_player {
            best_score = max(best_score, score);
            if depth == 0 && score == best_score {
                *best_move = (x, y);
            }
            if score > max_score {
                break;
            }
            min_score = max(min_score, score);
        } else {
            best_score = min(best_score, score);
            if score < min_score {
                break;
            }
            max_score = min(max_score, score);
        }
    }

    best_score
}
