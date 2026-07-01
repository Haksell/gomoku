mod capturophile;
mod zero;

use crate::model::Model;
use clap::ValueEnum;

pub type Heuristic = fn(&Model) -> i64;

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum HeuristicArg {
    Capturophile,
    Zero,
}

impl HeuristicArg {
    pub fn func(&self) -> Heuristic {
        match self {
            Self::Capturophile => capturophile::capturophile,
            Self::Zero => zero::zero,
        }
    }
}
