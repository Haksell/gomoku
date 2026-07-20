pub mod capturophile;
pub mod coeffistic;
pub mod manual;
pub mod zero;

use crate::{
    game::Game,
    heuristics::coeffistic::{Coeffs, INITIAL_COEFFS, OLD_COEFFS},
};

/// A [`Heuristic`] returns a positive value if black has a good position,
/// and a negative value otherwise.
#[derive(Debug, Clone)]
pub struct Heuristic {
    pub fun: fn(&Game, Option<&Coeffs>) -> i64,
    pub coeffs: Option<Coeffs>,
}

impl Heuristic {
    pub const ZERO: Self = Self { fun: zero::zero, coeffs: None };
    pub const CAPTUROPHILE: Self = Self { fun: capturophile::capturophile, coeffs: None };
    pub const MANUAL: Self = Self { fun: manual::manual, coeffs: None };

    pub fn new() -> Self {
        Self { fun: coeffistic::coeffistic, coeffs: Some(INITIAL_COEFFS.clone()) }
    }

    pub fn old() -> Self {
        Self { fun: coeffistic::coeffistic, coeffs: Some(OLD_COEFFS.clone()) }
    }
}

pub fn parse_heuristic(s: &str) -> Result<Heuristic, String> {
    match s {
        "zero" => Ok(Heuristic::ZERO),
        "capturophile" => Ok(Heuristic::CAPTUROPHILE),
        "manual" => Ok(Heuristic::MANUAL),
        "coeffistic" => Ok(Heuristic::new()),
        _ => Err(format!("Invalid heuristic: `{s}`")),
    }
}
