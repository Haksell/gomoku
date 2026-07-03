use crate::{
    game::{Game, board::BOARD_SIZE},
    player::PlayerColor,
};

pub fn new(game: &Game) -> i64 {
    let mut black_closed = [[0; 3]; 10];
    let mut white_closed = [[0; 3]; 10];

    // Lines
    for y in 0..BOARD_SIZE {
        fill_combos(game.board[y], &mut black_closed, &mut white_closed);
    }

    // Columns
    for x in 0..BOARD_SIZE {
        fill_combos(
            (0..BOARD_SIZE).map(|y| game.board[y][x]),
            &mut black_closed,
            &mut white_closed,
        );
    }

    // Upward diagonals
    for x in 0..BOARD_SIZE {
        fill_combos(
            (0..BOARD_SIZE - x).map(|y| game.board[y][x + y]),
            &mut black_closed,
            &mut white_closed,
        );
    }
    for y in 1..BOARD_SIZE {
        fill_combos(
            (0..BOARD_SIZE - y).map(|x| game.board[y + x][x]),
            &mut black_closed,
            &mut white_closed,
        );
    }

    // Downward diagonals
    for x in 0..BOARD_SIZE {
        fill_combos(
            (0..BOARD_SIZE - x).map(|y| game.board[BOARD_SIZE - y - 1][x + y]),
            &mut black_closed,
            &mut white_closed,
        );
    }
    for y in 1..BOARD_SIZE {
        fill_combos(
            (0..BOARD_SIZE - y).map(|x| game.board[BOARD_SIZE - y - x - 1][x]),
            &mut black_closed,
            &mut white_closed,
        );
    }

    let mut score = 0i64;

    score += game.black_captures.pow(3) as i64 - game.white_captures.pow(3) as i64;

    for length in 2..=9 {
        for openness in 1..=2 {
            score += (length as i64).pow(3)
                * (match openness {
                    1 => 1,
                    2 => 3,
                    _ => unreachable!(),
                })
                * (black_closed[length][openness] - white_closed[length][openness]);
        }
    }

    score
}

fn fill_combos(
    line: impl IntoIterator<Item = Option<PlayerColor>>,
    black_combos: &mut [[i64; 3]; 10],
    white_combos: &mut [[i64; 3]; 10],
) {
    let mut is_open_before = false;
    let mut cur_color = None;
    let mut cur_length = 0;

    for player_color in line {
        if player_color == cur_color {
            cur_length += 1;
        } else {
            match cur_color {
                None => {}
                Some(PlayerColor::Black) => {
                    let openness = is_open_before as usize + player_color.is_none() as usize;
                    black_combos[cur_length][openness] += 1;
                }
                Some(PlayerColor::White) => {
                    let openness = is_open_before as usize + player_color.is_none() as usize;
                    white_combos[cur_length][openness] += 1;
                }
            }
            is_open_before = cur_color.is_none();
            cur_color = player_color;
            cur_length = 1;
        }
    }

    match cur_color {
        None => {}
        Some(PlayerColor::Black) => {
            let openness = is_open_before as usize;
            black_combos[cur_length][openness] += 1;
        }
        Some(PlayerColor::White) => {
            let openness = is_open_before as usize;
            white_combos[cur_length][openness] += 1;
        }
    }
}
