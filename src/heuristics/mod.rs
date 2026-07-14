pub mod capturophile;
pub mod coeff_heuristic;
pub mod new;
pub mod old;
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
        "capturophile" => Ok(Heuristic { fun: capturophile::capturophile, coeffs: None }),
        "new" => Ok(Heuristic { fun: new::new, coeffs: None }),
        "old" => Ok(Heuristic { fun: old::old, coeffs: None }),
        "zero" => Ok(Heuristic { fun: zero::zero, coeffs: None }),
        _ => Err(format!("Invalid heuristic: `{s}`")),
    }
}
