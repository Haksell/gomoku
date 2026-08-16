use crate::{game::Game, heuristics::Coeffs};

#[must_use]
pub const fn capturophile(game: &Game, _: Option<&Coeffs>) -> i64 {
    game.black_captures as i64 - game.white_captures as i64
}
