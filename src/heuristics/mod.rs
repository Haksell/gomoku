mod capturophile;
mod zero;

pub use self::{capturophile::capturophile, zero::zero};

use crate::model::Model;
use clap::ValueEnum;
use std::{collections::HashMap, sync::LazyLock};

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum HeuristicArg {
    Capturophile,
    Zero,
}

pub type Heuristic = fn(&Model) -> i64;

pub static HEURISTIC_MAP: LazyLock<HashMap<&'static str, Heuristic>> = LazyLock::new(|| {
    HashMap::from([("zero", zero as Heuristic), ("capturophile", capturophile as Heuristic)])
});
