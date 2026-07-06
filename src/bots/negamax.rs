use crate::{
    bots::{MAX_DEPTH, leaf_value},
    game::{
        Game,
        board::{BOARD_CENTER, Position},
    },
    heuristics::Heuristic,
};
use std::cmp::max;

pub fn negamax(game: &Game, heuristic: Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let mut game = game.clone();
    game.get_legal_moves(Some(2), true)
        .into_iter()
        .max_by_key(|&(x, y)| {
            game.do_move(x, y);
            let h = -negamax_helper(&mut game, heuristic, 1);
            game.undo_last_move();
            h
        })
        .unwrap()
}

fn negamax_helper(game: &mut Game, heuristic: Heuristic, depth: usize) -> i64 {
    if let Some(leaf_value) = leaf_value(game, heuristic, depth, MAX_DEPTH) {
        return leaf_value;
    }

    game.get_legal_moves(Some(2), false).into_iter().fold(i64::MIN, |best_h, (x, y)| {
        game.do_move(x, y);
        let h = -negamax_helper(game, heuristic, depth + 1);
        game.undo_last_move();
        max(best_h, h)
    })
}
