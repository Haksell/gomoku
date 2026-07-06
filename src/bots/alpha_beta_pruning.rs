use crate::{
    bots::{MAX_DEPTH, leaf_value},
    game::{
        Game,
        board::{BOARD_CENTER, Position},
    },
    heuristics::Heuristic,
};
use std::cmp::max;

pub fn alpha_beta_pruning(game: &Game, heuristic: Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let mut best_move = (usize::MAX, usize::MAX);
    let mut game = game.clone();
    alpha_beta_pruning_helper(&mut game, heuristic, 0, -i64::MAX, i64::MAX, &mut best_move);
    best_move
}

fn alpha_beta_pruning_helper(
    game: &mut Game,
    heuristic: Heuristic,
    depth: usize,
    mut min_h: i64,
    max_h: i64,
    best_move: &mut Position,
) -> i64 {
    if let Some(leaf_value) = leaf_value(game, heuristic, depth, MAX_DEPTH) {
        return leaf_value;
    }

    let close_moves = game.get_legal_moves(Some(2), depth == 0);
    debug_assert!(!close_moves.is_empty());

    let mut best_h = i64::MIN;

    for (x, y) in close_moves {
        game.do_move(x, y);
        let h = -alpha_beta_pruning_helper(game, heuristic, depth + 1, -max_h, -min_h, best_move);
        game.undo_last_move();

        best_h = max(best_h, h);
        if depth == 0 && h == best_h {
            *best_move = (x, y);
        }
        min_h = max(min_h, best_h);
        if best_h > max_h {
            break;
        }
    }

    best_h
}
