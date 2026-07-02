pub mod alpha_beta_pruning;
pub mod minimax;
pub mod random_mover;

use crate::{
    game::{
        Game,
        board::{BOARD_SIZE, Position},
    },
    heuristics::Heuristic,
};
use nannou::rand::{seq::SliceRandom as _, thread_rng};

pub type Bot = fn(&Game, Heuristic) -> Position;

pub fn parse_bot(s: &str) -> Result<Bot, String> {
    match s {
        "alpha_beta_pruning" => Ok(alpha_beta_pruning::alpha_beta_pruning),
        "minimax" => Ok(minimax::minimax),
        "random_mover" => Ok(random_mover::random_mover),
        _ => Err(format!("Invalid bot: `{s}`")),
    }
}

fn get_legal_moves(game: &Game, shuffle: bool) -> Vec<Position> {
    if !game.forced_moves.is_empty() {
        return game.forced_moves.clone().into_iter().collect();
    }
    let mut legal_moves = Vec::new();
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if game.board[y][x].is_none() && !game.creates_double_three(x, y) {
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

// TODO: precompute
// TODO: manhattan?
fn get_close_moves(game: &Game, max_dist: usize, shuffle: bool) -> Vec<Position> {
    get_legal_moves(game, shuffle)

    // let neighborhood = |z: usize| {
    //     (z as isize - max_dist as isize).max(0) as usize..=(z + max_dist).min(BOARD_SIZE - 1)
    // };

    // let mut is_close = [[true; BOARD_SIZE]; BOARD_SIZE];
    // FIXME: incorrect because some moves have been captured
    // for &(x, y) in &game.moves {
    //     for ny in neighborhood(y) {
    //         for nx in neighborhood(x) {
    //             is_close[ny][nx] = true;
    //         }
    //     }
    // }

    // get_legal_moves(game, shuffle).into_iter().filter(|&(x, y)| is_close[y][x]).collect()
}
