use crate::{game::Game, heuristics::Coeffs};

pub const fn zero(_: &Game, _: Option<&Coeffs>) -> i64 {
    0
}
