use crate::{constants::BOARD_SIZE, model::Model, player::Player, rules::creates_double_three};
use std::{thread::sleep, time};

fn get_legal_moves(model: &Model) -> Vec<(usize, usize)> {
    if !model.forced_moves.is_empty() {
        return model.forced_moves.clone().into_iter().collect();
    }
    // TODO: refactor with draw_invalid_moves
    let mut legal_moves = Vec::new();
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if model.board[y][x] == Player::None
                && !creates_double_three(&model.board, model.current_player, x, y)
            {
                legal_moves.push((x, y));
            }
        }
    }
    legal_moves
}

pub fn get_bot_move(model: &Model) -> (usize, usize) {
    // sleep(time::Duration::from_millis(1000));
    let legal_moves = get_legal_moves(model);
    assert!(!legal_moves.is_empty()); // TODO
    legal_moves[0]
}
