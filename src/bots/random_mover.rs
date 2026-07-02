use crate::{
    game::{Game, board::Position},
    heuristics::Heuristic,
};

pub fn random_mover(game: &Game, _: Heuristic) -> Position {
    let legal_moves = game.get_legal_moves(None, true);
    assert!(!legal_moves.is_empty()); // TODO (should always be true once draws are implemented)
    legal_moves[0]
}
