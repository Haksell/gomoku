use crate::{game::Game, heuristics::Coeffs};

#[must_use]
pub const fn zero(_: &Game, _: Option<&Coeffs>) -> i64 {
    0
}
