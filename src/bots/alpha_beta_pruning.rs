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

pub fn alpha_beta_pruning(game: &Game, heuristic: Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let mut best_move = (usize::MAX, usize::MAX);
    alpha_beta_pruning_helper(
        &mut game.clone(),
        heuristic,
        game.current_color,
        0,
        i64::MIN,
        i64::MAX,
        &mut best_move,
    );
    best_move
}

fn alpha_beta_pruning_helper(
    game: &mut Game,
    heuristic: Heuristic,
    maximizing_player: PlayerColor,
    depth: usize,
    mut min_h: i64,
    mut max_h: i64,
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
        let h = alpha_beta_pruning_helper(
            game,
            heuristic,
            maximizing_player,
            depth + 1,
            min_h,
            max_h,
            best_move,
        );
        game.undo_last_move();

        if is_maximizing_player {
            best_h = max(best_h, h);
            if depth == 0 && h == best_h {
                *best_move = (x, y);
            }
            if h > max_h {
                break;
            }
            min_h = max(min_h, h);
        } else {
            best_h = min(best_h, h);
            if h < min_h {
                break;
            }
            max_h = min(max_h, h);
        }
    }

    best_h
}
