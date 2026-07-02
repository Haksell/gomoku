// TODO: aled (seems broken)

use crate::{
    game::{
        Game, GameState,
        board::{BOARD_CENTER, Position},
    },
    heuristics::Heuristic,
    player::PlayerColor,
};

const MAX_DEPTH: usize = 3;

pub fn minimax(game: &Game, heuristic: Heuristic) -> Position {
    if game.plies == 0 {
        return BOARD_CENTER;
    }

    game.get_legal_moves(Some(2), true)
        .into_iter()
        .max_by_key(|&(x, y)| {
            // TODO: undo_move instead of clone
            let mut game = game.clone();
            game.do_move(x, y);
            minimax_helper(&game, game.current_color, heuristic, 1)
        })
        .unwrap() // TODO: check get_close_moves never returns empty vector
}

fn minimax_helper(
    game: &Game,
    maximizing_player: PlayerColor,
    heuristic: Heuristic,
    depth: usize,
) -> i64 {
    match game.state {
        GameState::Playing => {}
        GameState::Draw => return 0,
        GameState::Won(winner) => {
            return if winner == maximizing_player { i64::MAX } else { i64::MIN };
        }
    }

    if depth == MAX_DEPTH {
        return heuristic(game, maximizing_player) - heuristic(game, !maximizing_player);
    }

    // TODO: NOT Some(2)
    let close_moves = game.get_legal_moves(Some(2), false);
    debug_assert!(!close_moves.is_empty());

    let is_maximizing_player = depth & 1 == 0;
    let mut best_score = if is_maximizing_player { i64::MIN } else { i64::MAX };
    for (x, y) in close_moves {
        let mut model = game.clone();
        model.do_move(x, y);
        let score = minimax_helper(&model, maximizing_player, heuristic, depth + 1);
        best_score =
            if is_maximizing_player { best_score.max(score) } else { best_score.min(score) };
        // model.undo_move(x, y);
    }

    best_score
}
