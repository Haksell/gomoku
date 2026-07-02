pub mod capturophile;
pub mod zero;

use crate::state::Game;

pub type Heuristic = fn(&Game) -> i64;

pub fn parse_heuristic(s: &str) -> Result<Heuristic, String> {
    match s {
        "capturophile" => Ok(capturophile::capturophile),
        "zero" => Ok(zero::zero),
        _ => Err(format!("Invalid heuristic: `{s}`")),
    }
}
