use super::get_close_moves;
use crate::{constants::BOARD_CENTER, heuristics::Heuristic, model::Model};

const MAX_DISTANCE: usize = 1;
const MAX_DEPTH: usize = 4;

pub fn minimax(model: &Model, heuristic: Heuristic) -> (usize, usize) {
    if model.moves.is_empty() {
        return BOARD_CENTER;
    }
    let close_moves = get_close_moves(model, MAX_DISTANCE, true);
    assert!(!close_moves.is_empty()); // TODO: check
    let mut best_score = i64::MIN;
    let mut best_move = close_moves[0];
    for (x, y) in close_moves {
        let mut model = model.clone();
        model.do_move(x, y);
        let score = _minimax(&model, heuristic, 1);
        if score > best_score {
            best_score = score;
            best_move = (x, y);
        }
        // model.undo_move(x, y);
    }
    best_move
}

fn _minimax(model: &Model, heuristic: Heuristic, depth: usize) -> i64 {
    if model.winner == model.human {
        return i64::MIN;
    }
    if model.winner == model.human.opponent() {
        return i64::MAX;
    }
    if depth == MAX_DEPTH {
        return heuristic(model);
    }
    let close_moves = get_close_moves(model, 1, false);
    if close_moves.is_empty() {
        return 0;
    }
    let is_maximizing_player = depth & 1 == 0;
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
