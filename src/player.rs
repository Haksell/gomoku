use crate::{
    algorithms::{Algorithm, ALGORITHM_MAP},
    heuristics::{Heuristic, HEURISTIC_MAP},
};

#[derive(Debug, Clone)]
pub enum Player {
    Human,
    Bot(&'static Algorithm, &'static Heuristic),
}

impl Player {
    pub fn from_arg(s: &str) -> Self {
        let s = s.to_lowercase();
        if s == "human" {
            return Player::Human;
        }
        let parts: Vec<&str> = s.split(':').collect();
        assert!(parts.len() == 2);
        let algorithm = ALGORITHM_MAP.get(parts[0]).unwrap();
        let heuristic = HEURISTIC_MAP.get(parts[1]).unwrap();
        Player::Bot(algorithm, heuristic)
    }
}
