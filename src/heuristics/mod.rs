pub mod capturophile;
pub mod genetrain;
pub mod new;
pub mod old;
pub mod zero;

use crate::game::Game;

/// A [`Heuristic`] returns a positive value if black has a good position,
/// and a negative value otherwise.
pub type Heuristic = fn(&Game) -> i64;

pub type Coefs = [i64; 729 + 9];

pub fn parse_heuristic(s: &str) -> Result<Heuristic, String> {
    match s {
        "capturophile" => Ok(capturophile::capturophile),
        "new" => Ok(new::new),
        "old" => Ok(old::old),
        "zero" => Ok(zero::zero),
        _ => Err(format!("Invalid heuristic: `{s}`")),
    }
}
