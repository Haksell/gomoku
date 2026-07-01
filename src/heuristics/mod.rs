mod capturophile;
mod zero;

use crate::model::Model;

pub type Heuristic = fn(&Model) -> i64;
pub const DEFAULT_HEURISTIC: Heuristic = capturophile::capturophile;

pub fn parse_heuristic(s: &str) -> Result<Heuristic, String> {
    match s {
        "capturophile" => Ok(capturophile::capturophile),
        "zero" => Ok(zero::zero),
        _ => Err(format!("Invalid heuristic: `{s}`")),
    }
}
