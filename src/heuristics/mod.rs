pub mod capturophile;
pub mod zero;

use crate::state::State;

pub type Heuristic = fn(&State) -> i64;

pub fn parse_heuristic(s: &str) -> Result<Heuristic, String> {
    match s {
        "capturophile" => Ok(capturophile::capturophile),
        "zero" => Ok(zero::zero),
        _ => Err(format!("Invalid heuristic: `{s}`")),
    }
}
