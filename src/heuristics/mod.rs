mod capturophile;
mod zero;

pub use self::{capturophile::capturophile, zero::zero};

use crate::model::Model;
use std::{collections::HashMap, sync::LazyLock};

pub type Heuristic = fn(&Model) -> i64;

pub const HEURISTIC_MAP: LazyLock<HashMap<&'static str, Heuristic>> = LazyLock::new(|| {
    HashMap::from([
        ("zero", zero as Heuristic),
        ("capturophile", capturophile as Heuristic),
    ])
});
