// TODO: aled (seems broken)

use crate::{
    game::{
        Game,
        board::{BOARD_CENTER, Position},
        state::GameState,
    },
    heuristics::Heuristic,
    player::PlayerColor,
};

const MAX_DEPTH: usize = 3;

pub fn minimax(game: &Game, heuristic: Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let mut game = game.clone();
    game.get_legal_moves(Some(2), true)
        .into_iter()
        .max_by_key(|&(x, y)| {
            game.do_move(x, y);
            let current_color = game.current_color;
            let h = minimax_helper(&mut game, current_color, heuristic, 1);
            game.undo_last_move();
            h
        })
        .unwrap()
}

fn minimax_helper(
    game: &mut Game,
    maximizing_player: PlayerColor,
    heuristic: Heuristic,
    depth: usize,
) -> i64 {
    match game.state {
        GameState::Playing(_) => {}
        GameState::Draw => return 0,
        GameState::Won(winner, _) => {
            return if winner == maximizing_player {
                i64::MAX - depth as i64
            } else {
                i64::MIN + depth as i64
            };
        }
    }

    if depth == MAX_DEPTH {
        return match maximizing_player {
            PlayerColor::Black => heuristic(game),
            PlayerColor::White => -heuristic(game),
        };
    }

    // TODO: NOT Some(2)
    let close_moves = game.get_legal_moves(Some(2), false);
    debug_assert!(!close_moves.is_empty());

    let is_maximizing_player = depth & 1 == 0;
    let mut best_h = if is_maximizing_player { i64::MIN } else { i64::MAX };

    for (x, y) in close_moves {
        game.do_move(x, y);
        let h = minimax_helper(game, maximizing_player, heuristic, depth + 1);
        game.undo_last_move();
        best_h = if is_maximizing_player { best_h.max(h) } else { best_h.min(h) };
    }

    best_h
}
