pub mod capturophile;
pub mod duelistic;
pub mod manual;
pub mod zero;

use crate::game::Game;

/// A [`Heuristic`] returns a positive value if black has a good position,
/// and a negative value otherwise.
#[derive(Debug, Clone, Copy)]
pub struct Heuristic {
    pub fun: fn(&Game, Option<&Coeffs>) -> i64,
    pub coeffs: Option<Coeffs>,
}

pub type Coeffs = [i64; 729 + 9];

pub fn parse_heuristic(s: &str) -> Result<Heuristic, String> {
    match s {
        "zero" => Ok(Heuristic { fun: zero::zero, coeffs: None }),
        "capturophile" => Ok(Heuristic { fun: capturophile::capturophile, coeffs: None }),
        "manual" => Ok(Heuristic { fun: manual::manual, coeffs: None }),
        "duelistic" => Ok(Heuristic {
            fun: duelistic::duelistic,
            coeffs: Some(include!("../../weights/current.rs")),
        }),
        _ => Err(format!("Invalid heuristic: `{s}`")),
    }
}
