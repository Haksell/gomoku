use super::get_legal_moves;
use crate::{
    heuristics::Heuristic,
    model::{Game, Position},
};

pub fn random_mover(game: &Game, _: Heuristic) -> Position {
    let legal_moves = get_legal_moves(game, true);
    assert!(!legal_moves.is_empty()); // TODO (should always be true once draws are implemented)
    legal_moves[0]
}
