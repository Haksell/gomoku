mod zero;

pub use self::zero::zero;
use crate::model::Model;

pub type Heuristic = fn(&Model) -> i64;
// TODO: Vec<Heuristic>
