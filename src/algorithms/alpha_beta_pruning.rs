use super::get_close_moves;
use crate::{constants::BOARD_CENTER, heuristics::Heuristic, model::Model, turn::Turn};

// TODO: struct with distance and number of moves
const DFS: &[(usize, usize)] = &[
    (1, usize::MAX),
    (1, usize::MAX),
    (1, usize::MAX),
    (1, usize::MAX),
];
const MAX_DEPTH: usize = DFS.len();

pub fn alpha_beta_pruning(model: &Model, heuristic: Heuristic) -> (usize, usize) {
    if model.moves.is_empty() {
        return BOARD_CENTER;
    }
    get_close_moves(model, DFS[0].0, true)
        .into_iter()
        .max_by_key(|&(x, y)| {
            // TODO: undo_move instead of clone
            let mut model = model.clone();
            model.do_move(x, y);
            _alpha_beta_pruning(&model, heuristic, 1, i64::MIN, i64::MAX)
        })
        .unwrap() // TODO: check get_close_moves never returns empty vector
}

fn _alpha_beta_pruning(
    model: &Model,
    heuristic: Heuristic,
    depth: usize,
    mut min_score: i64,
    mut max_score: i64,
) -> i64 {
    let is_maximizing_player = depth & 1 == 0;
    if model.winner != Turn::None {
        return if is_maximizing_player {
            i64::MIN
        } else {
            i64::MAX
        };
    }
    if depth == MAX_DEPTH {
        return heuristic(model);
    }
    // TODO: sort by heuristic instead of shuffle
    let close_moves = get_close_moves(model, DFS[1].0, true);
    if close_moves.is_empty() {
        return 0;
    }
    let mut best_score = if is_maximizing_player {
        i64::MIN
    } else {
        i64::MAX
    };
    // TODO: sort by depth 1 heuristic
    for &(x, y) in &close_moves[0..(DFS[depth].1).min(close_moves.len())] {
        let mut model = (*model).clone();
        model.do_move(x, y);
        let score = _alpha_beta_pruning(&model, heuristic, depth + 1, min_score, max_score);
        if is_maximizing_player {
            best_score = best_score.max(score);
            if score > max_score {
                break;
            }
            min_score = min_score.max(score);
        } else {
            best_score = best_score.min(score);
            if score < min_score {
                break;
            }
            max_score = max_score.min(score);
        }
        // model.undo_move(x, y);
    }
    best_score
}
