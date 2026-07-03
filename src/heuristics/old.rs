use crate::{
    game::{Game, board::BOARD_SIZE},
    player::PlayerColor,
};
use itertools::chain;

pub fn old(game: &Game) -> i64 {
    let mut black_combos = [0; 10];
    let mut white_combos = [0; 10];

    for y in 0..BOARD_SIZE {
        let mut cur_color = None;
        let mut cur_length = 0;
        for player_color in chain(game.board[y], std::iter::once(None)) {
            if player_color == cur_color {
                cur_length += 1;
            } else {
                match cur_color {
                    None => {}
                    Some(PlayerColor::Black) => black_combos[cur_length] += 1,
                    Some(PlayerColor::White) => white_combos[cur_length] += 1,
                }
                cur_color = player_color;
                cur_length = 1;
            }
        }
    }

    let mut score = 0i64;
    score += game.black_captures.pow(2) as i64 - game.white_captures.pow(2) as i64;
    for length in 2..=9 {
        score += (length as i64).pow(2) * (black_combos[length] - white_combos[length]);
    }

    score
}
