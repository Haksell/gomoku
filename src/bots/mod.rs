mod minimax;
mod random;

pub use self::{minimax::BotMinimax, random::BotRandom};
use crate::{constants::BOARD_SIZE, model::Model, player::Player, rules::creates_double_three};
use rand::{seq::SliceRandom as _, thread_rng};

pub trait Bot {
    fn get_move(model: &Model) -> (usize, usize);
}

fn get_legal_moves(model: &Model, shuffle: bool) -> Vec<(usize, usize)> {
    if !model.forced_moves.is_empty() {
        return model.forced_moves.clone().into_iter().collect();
    }
    let mut legal_moves = Vec::new();
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if model.board[y][x] == Player::None
                && !creates_double_three(&model.board, model.current_player, x, y)
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
    let mut is_close = [[false; BOARD_SIZE]; BOARD_SIZE];
    for &(x, y) in &model.moves {
        for dy in -(max_dist as isize)..=max_dist as isize {
            let ny = y as isize + dy;
            if ny < 0 || ny as usize >= BOARD_SIZE {
                continue;
            }
            let ny = ny as usize;
            for dx in -(max_dist as isize)..=max_dist as isize {
                let nx = x as isize + dx;
                if nx < 0 || nx as usize >= BOARD_SIZE {
                    continue;
                }
                let nx = nx as usize;
                is_close[ny][nx] = true;
            }
        }
    }
    get_legal_moves(model, shuffle)
        .into_iter()
        .filter(|&(x, y)| is_close[y][x])
        .collect()
}
