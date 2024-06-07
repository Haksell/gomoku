use super::get_legal_moves;
use crate::{heuristics::Heuristic, model::Model};

pub fn random_mover(model: &Model, _: Heuristic) -> (usize, usize) {
    let legal_moves = get_legal_moves(model, true);
    assert!(!legal_moves.is_empty()); // TODO (should always be true once draws are implemented)
    legal_moves[0]
}
