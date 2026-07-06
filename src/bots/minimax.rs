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

    let mut best_move = (usize::MAX, usize::MAX);
    minimax_helper(&mut game.clone(), heuristic, game.current_color, 0, &mut best_move);
    best_move
}

fn minimax_helper(
    game: &mut Game,
    heuristic: Heuristic,
    maximizing_player: PlayerColor,
    depth: usize,
    best_move: &mut Position,
) -> i64 {
    match game.state {
        GameState::Playing => {}
        GameState::Draw => return 0,
        GameState::Won(winner) => {
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

    let close_moves = game.get_legal_moves(Some(2), depth == 0);
    debug_assert!(!close_moves.is_empty());

    let is_maximizing_player = game.current_color == maximizing_player;
    let mut best_h = if is_maximizing_player { i64::MIN } else { i64::MAX };

    for (x, y) in close_moves {
        game.do_move(x, y);
        let h = minimax_helper(game, heuristic, maximizing_player, depth + 1, best_move);
        game.undo_last_move();

        if is_maximizing_player {
            best_h = max(best_h, h);
            if depth == 0 && h == best_h {
                *best_move = (x, y);
            }
        } else {
            best_h = min(best_h, h);
        }
    }

    best_h
}
