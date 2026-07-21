use crate::{
    bots::leaf_value,
    game::{
        Game,
        board::{BOARD_CENTER, Position},
    },
    heuristics::Heuristic,
};
use std::cmp::max;

const MAX_DEPTH: u32 = 4;

pub fn negamax(game: &Game, heuristic: &Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let mut game = game.clone();
    game.get_legal_moves(Some(2))
        .into_iter()
        .max_by_key(|&pos| {
            game.do_move(pos);
            let h = -negamax_helper(&mut game, heuristic, 1);
            game.undo_last_move();
            h
        })
        .unwrap()
}

fn negamax_helper(game: &mut Game, heuristic: &Heuristic, depth: u32) -> i64 {
    if let Some(leaf_value) = leaf_value(game, heuristic, depth, MAX_DEPTH) {
        return leaf_value;
    }

    game.get_legal_moves(Some(2)).into_iter().fold(i64::MIN, |best_h, pos| {
        game.do_move(pos);
        let h = -negamax_helper(game, heuristic, depth + 1);
        game.undo_last_move();
        max(best_h, h)
    })
}
