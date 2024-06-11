mod alpha_beta_pruning;
mod minimax;
mod random_mover;

pub use self::{
    alpha_beta_pruning::alpha_beta_pruning, minimax::minimax, random_mover::random_mover,
};
use crate::{
    constants::BOARD_SIZE, heuristics::Heuristic, model::Model, rules::creates_double_three,
    turn::Turn,
};
use lazy_static::lazy_static;
use rand::{seq::SliceRandom as _, thread_rng};
use std::collections::HashMap;

pub type Algorithm = fn(&Model, Heuristic) -> (usize, usize);

lazy_static! {
    pub static ref ALGORITHM_MAP: HashMap<&'static str, Algorithm> = {
        let mut map: HashMap<&'static str, Algorithm> = HashMap::new();
        map.insert("alpha_beta_pruning", alpha_beta_pruning);
        map.insert("minimax", minimax);
        map.insert("random_mover", random_mover);
        map
    };
}

fn get_legal_moves(model: &Model, shuffle: bool) -> Vec<(usize, usize)> {
    if !model.forced_moves.is_empty() {
        return model.forced_moves.clone().into_iter().collect();
    }
    let mut legal_moves = Vec::new();
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if model.board[y][x] == Turn::None
                && !creates_double_three(&model.board, model.current_turn, x, y)
            {
                legal_moves.push((x, y));
            }
        }
    }
    if shuffle {
        let mut rng = thread_rng();
        legal_moves.shuffle(&mut rng);
    }
    legal_moves
}

/// TODO: precompute
fn get_close_moves(model: &Model, max_dist: usize, shuffle: bool) -> Vec<(usize, usize)> {
    let neighborhood = |z: usize| {
        (z as isize - max_dist as isize).max(0) as usize..=(z + max_dist).min(BOARD_SIZE - 1)
    };

    let mut is_close = [[false; BOARD_SIZE]; BOARD_SIZE];
    for &(x, y) in &model.moves {
        for ny in neighborhood(y) {
            for nx in neighborhood(x) {
                is_close[ny][nx] = true;
            }
        }
    }
    get_legal_moves(model, shuffle)
        .into_iter()
        .filter(|&(x, y)| is_close[y][x])
        .collect()
}
