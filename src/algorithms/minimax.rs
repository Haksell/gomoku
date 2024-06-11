use super::get_close_moves;
use crate::{constants::BOARD_CENTER, heuristics::Heuristic, model::Model, turn::Turn};

const MAX_DEPTH: usize = 4;

pub fn minimax(model: &Model, heuristic: Heuristic) -> (usize, usize) {
    if model.moves.is_empty() {
        return BOARD_CENTER;
    }
    let is_maximizing_player = model.current_turn == Turn::Black;
    get_close_moves(model, 1, true)
        .into_iter()
        .max_by_key(|&(x, y)| {
            // TODO: undo_move instead of clone
            let mut model = model.clone();
            model.do_move(x, y);
            if is_maximizing_player {
                _minimax(&model, heuristic, 1)
            } else {
                -_minimax(&model, heuristic, 1)
            }
        })
        .unwrap() // TODO: check get_close_moves never returns empty vector
}

fn _minimax(model: &Model, heuristic: Heuristic, depth: usize) -> i64 {
    let is_maximizing_player = model.current_turn == Turn::Black;
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
    let close_moves = get_close_moves(model, 1, false);
    if close_moves.is_empty() {
        return 0;
    }
    let mut best_score = if is_maximizing_player {
        i64::MIN
    } else {
        i64::MAX
    };
    for (x, y) in close_moves {
        let mut model = (*model).clone();
        model.do_move(x, y);
        let score = _minimax(&model, heuristic, depth + 1);
        best_score = if is_maximizing_player {
            best_score.max(score)
        } else {
            best_score.min(score)
        };
        // model.undo_move(x, y);
    }
    best_score
}
