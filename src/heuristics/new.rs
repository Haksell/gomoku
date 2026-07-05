use crate::{
    game::{
        Game,
        board::{Board, Position},
        lines::{COLUMNS, DOWNWARD_DIAGONALS, ROWS, UPWARD_DIAGONALS},
    },
    player::PlayerColor,
};

pub fn new(game: &Game) -> i64 {
    let mut black_combos = [[0; 3]; 10];
    let mut white_combos = [[0; 3]; 10];
    let mut black_open_xx_x = 0;
    let mut white_open_xx_x = 0;
    let mut black_xx_xx = 0;
    let mut white_xx_xx = 0;
    let mut black_x_x_x = 0;
    let mut white_x_x_x = 0;
    let mut black_xxx_x = 0;
    let mut white_xxx_x = 0;

    for lines in [ROWS, COLUMNS] {
        for line in &lines {
            fill_combos(&game.board, line, &mut black_combos, &mut white_combos);
            fill_patterns(
                &game.board,
                line,
                &mut black_open_xx_x,
                &mut white_open_xx_x,
                &mut black_xx_xx,
                &mut white_xx_xx,
                &mut black_x_x_x,
                &mut white_x_x_x,
                &mut black_xxx_x,
                &mut white_xxx_x,
            );
        }
    }

    for lines in [UPWARD_DIAGONALS, DOWNWARD_DIAGONALS] {
        for line in lines {
            fill_combos(&game.board, line, &mut black_combos, &mut white_combos);
            fill_patterns(
                &game.board,
                line,
                &mut black_open_xx_x,
                &mut white_open_xx_x,
                &mut black_xx_xx,
                &mut white_xx_xx,
                &mut black_x_x_x,
                &mut white_x_x_x,
                &mut black_xxx_x,
                &mut white_xxx_x,
            );
        }
    }

    let mut h = 0;

    // TODO: find better factor
    h += (game.white_dist_to_center as i64 - game.black_dist_to_center as i64) / 8;
    h += (game.black_captures.pow(3) as i64 - game.white_captures.pow(3) as i64) * 3;

    // TODO: semi-open with wall shouldn't count
    h += (white_combos[2][1] - black_combos[2][1]) * 100;

    for length in 2..=9 {
        for openness in 1..=2 {
            h += (length as i64).pow(5)
                * (match openness {
                    1 => 1,
                    2 => 3,
                    _ => unreachable!(),
                })
                * (black_combos[length][openness] - white_combos[length][openness]);
        }
    }

    h += (black_open_xx_x - white_open_xx_x) * 1152;

    h += (black_xx_xx - white_xx_xx) * 512;
    /*
    0 => 50
    256 => 49.6
    512 => 50.4 51.9
    1024 => 50.1
    4096 => 35.9
    */

    h += (black_x_x_x - white_x_x_x) * 128;
    /*
    0 => 50
    64 => 50.5
    128 => 51.5 52.2
    192 => 50.3
    256 => 51.2
    */

    h += (black_xxx_x - white_xxx_x) * 64;

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

fn fill_patterns(
    board: &Board,
    line: &[Position],
    black_open_xx_x: &mut i64,
    white_open_xx_x: &mut i64,
    black_xx_xx: &mut i64,
    white_xx_xx: &mut i64,
    black_x_x_x: &mut i64,
    white_x_x_x: &mut i64,
    black_xxx_x: &mut i64,
    white_xxx_x: &mut i64,
) {
    let mut stencil = 0;

    for &(x, y) in line {
        let player_color = board[y][x];
        stencil <<= 2;
        stencil |= match player_color {
            None => 0b01,
            Some(PlayerColor::Black) => 0b10,
            Some(PlayerColor::White) => 0b11,
        };

        match stencil & 4095 {
            0b_01_10_10_01_10_01 | 0b_01_10_01_10_10_01 => *black_open_xx_x += 1,
            0b_01_11_11_01_11_01 | 0b_01_11_01_11_11_01 => *white_open_xx_x += 1,
            _ => {}
        }

        match stencil & 1023 {
            0b_10_10_01_10_10 => *black_xx_xx += 1,
            0b_11_11_01_11_11 => *white_xx_xx += 1,
            0b_10_01_10_01_10 => *black_x_x_x += 1,
            0b_11_01_11_01_11 => *white_x_x_x += 1,
            0b_10_10_10_01_10 | 0b_10_01_10_10_10 => *black_xxx_x += 1,
            0b_11_11_11_01_11 | 0b_11_01_11_11_11 => *white_xxx_x += 1,
            _ => {}
        }
    }
}
