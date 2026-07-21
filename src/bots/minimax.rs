use crate::{
    game::{
        Game,
        board::{BOARD_CENTER, Position},
        state::GameState,
    },
    heuristics::Heuristic,
    player::PlayerColor,
};
use std::cmp::{max, min};

const MAX_DEPTH: usize = 4;

pub fn minimax(game: &Game, heuristic: &Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let maximizing_player = game.current_color;
    let mut game = game.clone();
    game.get_legal_moves(Some(2))
        .into_iter()
        .max_by_key(|&pos| {
            game.do_move(pos);
            let h = minimax_helper(&mut game, heuristic, maximizing_player, 1);
            game.undo_last_move();
            h
        })
        .unwrap()
}

fn minimax_helper(
    game: &mut Game,
    heuristic: &Heuristic,
    maximizing_player: PlayerColor,
    depth: usize,
) -> i64 {
    match game.state {
        GameState::Playing(_) => {
            if depth == MAX_DEPTH {
                return match maximizing_player {
                    PlayerColor::Black => (heuristic.fun)(game, heuristic.coeffs.as_ref()),
                    PlayerColor::White => -(heuristic.fun)(game, heuristic.coeffs.as_ref()),
                };
            }
        }
        GameState::Draw => return 0,
        GameState::Won(winner, _) => {
            return if winner == maximizing_player {
                i64::MAX - depth as i64
            } else {
                i64::MIN + depth as i64
            };
        }
    }

    let close_moves = game.get_legal_moves(Some(2));
    debug_assert!(!close_moves.is_empty());

    let is_maximizing_player = game.current_color == maximizing_player;
    let initial_h = if is_maximizing_player { i64::MIN } else { i64::MAX };

    close_moves.into_iter().fold(initial_h, |best_h, pos| {
        game.do_move(pos);
        let h = minimax_helper(game, heuristic, maximizing_player, depth + 1);
        game.undo_last_move();
        if is_maximizing_player { max(best_h, h) } else { min(best_h, h) }
    })
}
