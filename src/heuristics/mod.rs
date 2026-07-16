pub mod capturophile;
pub mod coeffistic;
pub mod manual;
pub mod zero;

use crate::{game::Game, heuristics::coeffistic::Coeffs};

/// A [`Heuristic`] returns a positive value if black has a good position,
/// and a negative value otherwise.
#[derive(Debug, Clone, Copy)]
pub struct Heuristic {
    pub fun: fn(&Game, Option<&Coeffs>) -> i64,
    pub coeffs: Option<Coeffs>,
}

pub fn parse_heuristic(s: &str) -> Result<Heuristic, String> {
    match s {
        "zero" => Ok(Heuristic { fun: zero::zero, coeffs: None }),
        "capturophile" => Ok(Heuristic { fun: capturophile::capturophile, coeffs: None }),
        "manual" => Ok(Heuristic { fun: manual::manual, coeffs: None }),
        "coeffistic" => Ok(Heuristic {
            fun: coeffistic::coeffistic,
            coeffs: Some(include!("../../coeffs/current.rs")),
        }),
        _ => Err(format!("Invalid heuristic: `{s}`")),
    }
}
