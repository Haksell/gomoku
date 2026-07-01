use super::get_close_moves;
use crate::{
    constants::BOARD_CENTER,
    heuristics::Heuristic,
    model::{Model, Position},
    player::PlayerColor,
};

const MAX_DEPTH: usize = 4;

pub fn minimax(model: &Model, heuristic: Heuristic) -> Position {
    if model.moves.is_empty() {
        return BOARD_CENTER;
    }
    get_close_moves(model, 1, true)
        .into_iter()
        .max_by_key(|&(x, y)| {
            // TODO: undo_move instead of clone
            let mut model = model.clone();
            model.do_move(x, y);
            minimax_helper(&model, model.current_color, heuristic, 1)
        })
        .unwrap() // TODO: check get_close_moves never returns empty vector
}

fn minimax_helper(
    model: &Model,
    current_player: PlayerColor,
    heuristic: Heuristic,
    depth: usize,
) -> i64 {
    if let Some(winner) = model.winner {
        return if winner == current_player { i64::MAX } else { i64::MIN };
    }
    if depth == MAX_DEPTH {
        return heuristic(model);
    }
    let close_moves = get_close_moves(model, 1, false);
    if close_moves.is_empty() {
        return 0;
    }
    let is_maximizing_player = depth & 1 == 0;
    let mut best_score = if is_maximizing_player { i64::MIN } else { i64::MAX };
    for (x, y) in close_moves {
        let mut model = model.clone();
        model.do_move(x, y);
        let score = minimax_helper(&model, current_player, heuristic, depth + 1);
        best_score =
            if is_maximizing_player { best_score.max(score) } else { best_score.min(score) };
        // model.undo_move(x, y);
    }
    best_score
}
