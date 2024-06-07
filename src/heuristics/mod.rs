mod capturophile;
mod zero;

use std::collections::HashMap;

pub use self::{capturophile::capturophile, zero::zero};
use crate::model::Model;
use lazy_static::lazy_static;

pub type Heuristic = fn(&Model) -> i64;

lazy_static! {
    pub static ref HEURISTIC_MAP: HashMap<&'static str, Heuristic> = {
        let mut map: HashMap<&'static str, Heuristic> = HashMap::new();
        map.insert("zero", zero);
        map.insert("capturophile", capturophile);
        map
    };
}
