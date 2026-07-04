use crate::{
    game::{
        Game,
        board::{Board, Position},
        lines::{COLUMNS, DOWNWARD_DIAGONALS, ROWS, UPWARD_DIAGONALS},
    },
    player::PlayerColor,
};

pub fn old(game: &Game) -> i64 {
    let mut black_combos = [[0; 3]; 10];
    let mut white_combos = [[0; 3]; 10];

    for lines in [ROWS, COLUMNS] {
        for line in &lines {
            fill_combos(&game.board, line, &mut black_combos, &mut white_combos);
        }
    }

    for lines in [UPWARD_DIAGONALS, DOWNWARD_DIAGONALS] {
        for line in lines {
            fill_combos(&game.board, line, &mut black_combos, &mut white_combos);
        }
    }

    let mut h = 0;

    // TODO: find better factor
    h += (game.white_dist_to_center as i64 - game.black_dist_to_center as i64) / 8;
    h += (game.black_captures.pow(3) as i64 - game.white_captures.pow(3) as i64) * 3;

    for length in 2..=9 {
        for openness in 1..=2 {
            h += (length as i64).pow(4)
                * (match openness {
                    1 => 1,
                    2 => 3,
                    _ => unreachable!(),
                })
                * (black_combos[length][openness] - white_combos[length][openness]);
        }
    }

    h
}

fn fill_combos(
    board: &Board,
    line: &[Position],
    black_combos: &mut [[i64; 3]; 10],
    white_combos: &mut [[i64; 3]; 10],
) {
    let mut is_open_before = false;
    let mut cur_color = None;
    let mut cur_length = 0;

    for &(x, y) in line {
        let player_color = board[y][x];
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
