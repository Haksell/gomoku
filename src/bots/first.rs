use super::{get_legal_moves, Bot};
use crate::model::Model;

pub struct BotFirst {}

impl Bot for BotFirst {
    fn get_move(model: &Model) -> (usize, usize) {
        let legal_moves = get_legal_moves(model);
        assert!(!legal_moves.is_empty()); // TODO (should always be true once draws are implemented)
        legal_moves[0]
    }
}
