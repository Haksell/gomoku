use crate::{
    game::{
        Game, GameState,
        board::{BOARD_CENTER, Position},
    },
    heuristics::Heuristic,
    player::PlayerColor,
};
use std::cmp::{max, min};

// TODO: different max_dist and number of best moves to check depending on depth
const MAX_DEPTH: usize = 3;

pub fn alpha_beta_pruning(game: &Game, heuristic: Heuristic) -> Position {
    if game.plies == 0 {
        return BOARD_CENTER;
    }

    let mut best_move = (usize::MAX, usize::MAX);
    alpha_beta_pruning_helper(
        game,
        game.current_color,
        heuristic,
        0,
        i64::MIN,
        i64::MAX,
        &mut best_move,
    );
    best_move
}

fn alpha_beta_pruning_helper(
    game: &Game,
    maximizing_player: PlayerColor,
    heuristic: Heuristic,
    depth: usize,
    mut min_score: i64,
    mut max_score: i64,
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
        return heuristic(game, maximizing_player) - heuristic(game, !maximizing_player);
    }

    // TODO: sort by depth 1 heuristic
    let close_moves = game.get_legal_moves(Some(2), depth == 0);
    debug_assert!(!close_moves.is_empty());

    let is_maximizing_player = game.current_color == maximizing_player;
    let mut best_score = if is_maximizing_player { i64::MIN } else { i64::MAX };

    for (x, y) in close_moves {
        // TODO: do_move then undo_move
        let mut model = game.clone();
        model.do_move(x, y);
        let score = alpha_beta_pruning_helper(
            &model,
            maximizing_player,
            heuristic,
            depth + 1,
            min_score,
            max_score,
            best_move,
        );
        if is_maximizing_player {
            best_score = max(best_score, score);
            if depth == 0 && score == best_score {
                *best_move = (x, y);
            }
            if score > max_score {
                break;
            }
            min_score = max(min_score, score);
        } else {
            best_score = min(best_score, score);
            if score < min_score {
                break;
            }
            max_score = min(max_score, score);
        }
    }

    best_score
}
