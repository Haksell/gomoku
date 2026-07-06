use crate::{
    game::{
        Game, GameState,
        board::{BOARD_CENTER, Position},
    },
    heuristics::Heuristic,
    player::PlayerColor,
};
use std::cmp::{max, min};

const MAX_DEPTH: usize = 3;

pub fn minimax(game: &Game, heuristic: Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let maximizing_player = game.current_color;
    let mut game = game.clone();
    game.get_legal_moves(Some(2), true)
        .into_iter()
        .max_by_key(|&(x, y)| {
            game.do_move(x, y);
            let h = minimax_helper(&mut game, heuristic, maximizing_player, 1);
            game.undo_last_move();
            h
        })
        .unwrap()
}

fn minimax_helper(
    game: &mut Game,
    heuristic: Heuristic,
    maximizing_player: PlayerColor,
    depth: usize,
) -> i64 {
    match game.state {
        GameState::Playing => {
            if depth == MAX_DEPTH {
                return match maximizing_player {
                    PlayerColor::Black => heuristic(game),
                    PlayerColor::White => -heuristic(game),
                };
            }
        }
        GameState::Draw => return 0,
        GameState::Won(winner) => {
            return if winner == maximizing_player {
                i64::MAX - depth as i64
            } else {
                i64::MIN + depth as i64
            };
        }
    }

    let close_moves = game.get_legal_moves(Some(2), false);
    debug_assert!(!close_moves.is_empty());

    let is_maximizing_player = game.current_color == maximizing_player;
    let initial_h = if is_maximizing_player { i64::MIN } else { i64::MAX };

    game.get_legal_moves(Some(2), false).into_iter().fold(initial_h, |best_h, (x, y)| {
        game.do_move(x, y);
        let h = minimax_helper(game, heuristic, maximizing_player, depth + 1);
        game.undo_last_move();
        if is_maximizing_player { max(best_h, h) } else { min(best_h, h) }
    })
}
