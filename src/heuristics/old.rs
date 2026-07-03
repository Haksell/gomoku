use crate::{
    game::{Game, board::BOARD_SIZE},
    player::PlayerColor,
};
use itertools::chain;

pub fn old(game: &Game) -> i64 {
    let mut black_combos = [0; 10];
    let mut white_combos = [0; 10];

    // Lines
    for y in 0..BOARD_SIZE {
        fill_combos(game.board[y], &mut black_combos, &mut white_combos);
    }

    // Columns
    for x in 0..BOARD_SIZE {
        fill_combos(
            (0..BOARD_SIZE).map(|y| game.board[y][x]),
            &mut black_combos,
            &mut white_combos,
        );
    }

    // Upward diagonals
    for x in 0..BOARD_SIZE {
        fill_combos(
            (0..BOARD_SIZE - x).map(|y| game.board[y][x + y]),
            &mut black_combos,
            &mut white_combos,
        );
    }
    for y in 1..BOARD_SIZE {
        fill_combos(
            (0..BOARD_SIZE - y).map(|x| game.board[y + x][x]),
            &mut black_combos,
            &mut white_combos,
        );
    }

    // Downward diagonals
    for x in 1..BOARD_SIZE {
        fill_combos(
            (0..BOARD_SIZE - x).map(|y| game.board[BOARD_SIZE - y - 1][x + y]),
            &mut black_combos,
            &mut white_combos,
        );
    }
    for y in 1..BOARD_SIZE {
        fill_combos(
            (0..BOARD_SIZE - y).map(|x| game.board[BOARD_SIZE - y - x - 1][x]),
            &mut black_combos,
            &mut white_combos,
        );
    }

    let mut score = 0i64;
    score += game.black_captures.pow(3) as i64 - game.white_captures.pow(3) as i64;
    for length in 2..=9 {
        score += (length as i64).pow(3) * (black_combos[length] - white_combos[length]);
    }

    score
}

fn fill_combos(
    line: impl IntoIterator<Item = Option<PlayerColor>>,
    black_combos: &mut [i64; 10],
    white_combos: &mut [i64; 10],
) {
    let mut cur_color = None;
    let mut cur_length = 0;
    for player_color in chain(line, std::iter::once(None)) {
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
