use super::get_close_moves;
use crate::{
    constants::BOARD_CENTER,
    heuristics::Heuristic,
    player::PlayerColor,
    state::{Position, State},
};

const MAX_DEPTH: usize = 4;

pub fn minimax(state: &State, heuristic: Heuristic) -> Position {
    if state.moves.is_empty() {
        return BOARD_CENTER;
    }
    get_close_moves(state, 1, true)
        .into_iter()
        .max_by_key(|&(x, y)| {
            // TODO: undo_move instead of clone
            let mut state = state.clone();
            state.do_move(x, y);
            minimax_helper(&state, state.current_color, heuristic, 1)
        })
        .unwrap() // TODO: check get_close_moves never returns empty vector
}

fn minimax_helper(
    state: &State,
    current_player: PlayerColor,
    heuristic: Heuristic,
    depth: usize,
) -> i64 {
    if let Some(winner) = state.winner {
        return if winner == current_player { i64::MAX } else { i64::MIN };
    }
    if depth == MAX_DEPTH {
        return heuristic(state);
    }
    let close_moves = get_close_moves(state, 1, false);
    if close_moves.is_empty() {
        return 0;
    }
    let is_maximizing_player = depth & 1 == 0;
    let mut best_score = if is_maximizing_player { i64::MIN } else { i64::MAX };
    for (x, y) in close_moves {
        let mut state = state.clone();
        state.do_move(x, y);
        let score = minimax_helper(&state, current_player, heuristic, depth + 1);
        best_score =
            if is_maximizing_player { best_score.max(score) } else { best_score.min(score) };
        // state.undo_move(x, y);
    }
    best_score
}
