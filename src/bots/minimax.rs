use super::{get_close_moves, Bot};
use crate::{constants::BOARD_SIZE, model::Model};

const MAX_DISTANCE: usize = 2;

pub struct BotMinimax {}

impl Bot for BotMinimax {
    fn get_move(model: &Model) -> (usize, usize) {
        if model.moves.is_empty() {
            return (BOARD_SIZE / 2, BOARD_SIZE / 2);
        }
        let close_moves = get_close_moves(model, MAX_DISTANCE);
        assert!(!close_moves.is_empty()); // TODO: check
        close_moves[0]
    }
}
