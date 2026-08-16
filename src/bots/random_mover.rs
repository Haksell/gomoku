use crate::{
    game::{Game, board::Position},
    heuristics::Heuristic,
};
use rand::{rng, seq::IndexedRandom as _};

/// # Panics
///
/// Will panic if no legal moves can be played.
pub fn random_mover(game: &Game, _: &Heuristic) -> Position {
    let legal_moves = game.get_legal_moves(None);
    assert!(!legal_moves.is_empty()); // TODO: (should always be true once draws are implemented)
    *legal_moves.choose(&mut rng()).unwrap()
}
