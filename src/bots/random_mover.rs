use super::get_legal_moves;
use crate::{
    heuristics::Heuristic,
    state::{Position, State},
};

pub fn random_mover(state: &State, _: Heuristic) -> Position {
    let legal_moves = get_legal_moves(state, true);
    assert!(!legal_moves.is_empty()); // TODO (should always be true once draws are implemented)
    legal_moves[0]
}
