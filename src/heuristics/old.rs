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
    let mut black_open_xx_x = 0;
    let mut white_open_xx_x = 0;
    let mut black_xx_xx = 0;
    let mut white_xx_xx = 0;
    let mut black_x_x_x = 0;
    let mut white_x_x_x = 0;
    let mut black_xxx_x = 0;
    let mut white_xxx_x = 0;
    let mut black_x_x = 0;
    let mut white_x_x = 0;
    let mut black_capture_threats = 0;
    let mut white_capture_threats = 0;
    let mut black_locked_4 = 0;
    let mut white_locked_4 = 0;

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
                &mut black_x_x,
                &mut white_x_x,
                &mut black_capture_threats,
                &mut white_capture_threats,
                &mut black_locked_4,
                &mut white_locked_4,
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
                &mut black_x_x,
                &mut white_x_x,
                &mut black_capture_threats,
                &mut white_capture_threats,
                &mut black_locked_4,
                &mut white_locked_4,
            );
        }
    }

    let mut h = 0;

    // TODO: find better factor
    h += (game.white_dist_to_center as i64 - game.black_dist_to_center as i64) / 8;

    #[expect(clippy::items_after_statements)]
    const fn capture_heuristic(captures: i64, threats: i64) -> i64 {
        match (captures, threats) {
            (0, 0) => 0,
            (0, 1) => 200,
            (0, 2) => 400,
            (0, 3) => 600,
            (0, 4) => 800,
            (0, _) => 200 * threats,
            (1, 0) => 16,
            (1, 1) => 216,
            (1, 2) => 416,
            (1, 3) => 616,
            (1, 4) => 816,
            (1, _) => 16 + 200 * threats,
            (2, 0) => 128,
            (2, 1) => 328,
            (2, 2) => 528,
            (2, 3) => 728,
            (2, 4) => 928,
            (2, _) => 128 + 200 * threats,
            (3, 0) => 432,
            (3, 1) => 632,
            (3, 2) => 832,
            (3, 3) => 1032,
            (3, 4) => 1232,
            (3, _) => 432 + 200 * threats,
            (4, 0) => 1024,
            (4, 1) => 1224,
            (4, 2) => 1424,
            (4, 3) => 1624,
            (4, 4) => 1824,
            (4, _) => 1024 + 200 * threats,
            _ => unreachable!(),
        }
    }

    h += capture_heuristic(game.black_captures as i64, black_capture_threats);
    h -= capture_heuristic(game.white_captures as i64, white_capture_threats);

    // h += (black_combos[2][0] - white_combos[2][0]) * 0;
    // h += (black_combos[3][0] - white_combos[3][0]) * 0;
    // h += (black_combos[4][0] - white_combos[4][0]) * 0;
    h += (black_combos[5][0] - white_combos[5][0]) * 32768;
    h += (black_combos[6][0] - white_combos[6][0]) * 32768;
    h += (black_combos[7][0] - white_combos[7][0]) * 32768;
    h += (black_combos[8][0] - white_combos[8][0]) * 32768;
    h += (black_combos[9][0] - white_combos[9][0]) * 32768;

    h += (black_combos[2][1] - white_combos[2][1]) * 32;
    h += (black_combos[3][1] - white_combos[3][1]) * 243;
    h += (black_combos[4][1] - white_combos[4][1]) * 1024;
    h += (black_combos[5][1] - white_combos[5][1]) * 6250;
    h += (black_combos[6][1] - white_combos[6][1]) * 15552;
    h += (black_combos[7][1] - white_combos[7][1]) * 33614;
    h += (black_combos[8][1] - white_combos[8][1]) * 65536;
    h += (black_combos[9][1] - white_combos[9][1]) * 99999;

    h += (black_combos[2][2] - white_combos[2][2]) * 96;
    h += (black_combos[3][2] - white_combos[3][2]) * 896;
    h += (black_combos[4][2] - white_combos[4][2]) * 3072;
    h += (black_combos[5][2] - white_combos[5][2]) * 9375;
    h += (black_combos[6][2] - white_combos[6][2]) * 23328;
    h += (black_combos[7][2] - white_combos[7][2]) * 50421;
    h += (black_combos[8][2] - white_combos[8][2]) * 98304;
    h += (black_combos[9][2] - white_combos[9][2]) * 99999;

    h += (black_open_xx_x - white_open_xx_x) * 1152;

    h += (black_xx_xx - white_xx_xx) * 512;
    h += (black_x_x_x - white_x_x_x) * 128;
    h += (black_xxx_x - white_xxx_x) * 64;

    h += (black_x_x - white_x_x) * 36;

    h += (white_locked_4 - black_locked_4) * 384;

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

#[expect(clippy::too_many_arguments)]
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
    black_x_x: &mut i64,
    white_x_x: &mut i64,
    black_capture_threats: &mut i64,
    white_capture_threats: &mut i64,
    black_locked_4: &mut i64,
    white_locked_4: &mut i64,
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
            0b_11_10_10_10_10_11 => *black_locked_4 += 1,
            0b_10_11_11_11_11_10 => *white_locked_4 += 1,
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

        match stencil & 255 {
            0b_11_10_10_01 | 0b_01_10_10_11 => *white_capture_threats += 1,
            0b_10_11_11_01 | 0b_01_11_11_10 => *black_capture_threats += 1,
            _ => {}
        }

        match stencil & 63 {
            0b_10_01_10 => *black_x_x += 1,
            0b_11_01_11 => *white_x_x += 1,
            _ => {}
        }
    }
}
