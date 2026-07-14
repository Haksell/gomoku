use crate::{
    game::{
        Game,
        board::{Board, Position},
        lines::{COLUMNS, DOWNWARD_DIAGONALS, ROWS, UPWARD_DIAGONALS},
    },
    heuristics::Coeffs,
    player::PlayerColor,
};

pub fn coeff_heuristic(game: &Game) -> i64 {
    let mut h = 0;
    let mut black_capture_threats = 0;
    let mut white_capture_threats = 0;

    for lines in [ROWS, COLUMNS] {
        for line in &lines {
            h += evaluate_patterns(
                &game.board,
                line,
                &game.coeffs.unwrap(),
                &mut black_capture_threats,
                &mut white_capture_threats,
            );
        }
    }

    for lines in [UPWARD_DIAGONALS, DOWNWARD_DIAGONALS] {
        for line in lines {
            h += evaluate_patterns(
                &game.board,
                line,
                &game.coeffs.unwrap(),
                &mut black_capture_threats,
                &mut white_capture_threats,
            );
        }
    }

    h +=
        capture_heuristic(&game.coeffs.unwrap(), game.black_captures as i64, black_capture_threats);
    h -=
        capture_heuristic(&game.coeffs.unwrap(), game.white_captures as i64, white_capture_threats);

    h
}

#[expect(clippy::identity_op)]
const fn capture_heuristic(coeffs: &Coeffs, c: i64, t: i64) -> i64 {
    // None of these constants are properly optimized,
    // because capture wins are much rarer than alignments.
    let h_captures = coeffs[729 + 0] * c * c * c + coeffs[729 + 1] * c * c + coeffs[729 + 2] * c;
    let h_threats = coeffs[729 + 3] * t * t * t + coeffs[729 + 4] * t * t + coeffs[729 + 5] * t;
    let cross_terms = c * t * (coeffs[729 + 6] + coeffs[729 + 7] * c + coeffs[729 + 8] * t);
    h_captures + h_threats + cross_terms
}

const fn stencil_index(mut stencil: i64) -> usize {
    let mut index = 0;
    while stencil > 0 {
        index += ((stencil & 0b11) - 1) as usize;
        stencil >>= 2;
    }
    index
}

fn evaluate_patterns(
    board: &Board,
    line: &[Position],
    coeffs: &Coeffs,
    black_capture_threats: &mut i64,
    white_capture_threats: &mut i64,
) -> i64 {
    let mut stencil = 0;
    let mut h = 0;

    for (i, &(x, y)) in line.iter().enumerate() {
        let player_color = board[y][x];
        stencil <<= 2;
        stencil |= match player_color {
            None => 0b01,
            Some(PlayerColor::Black) => 0b10,
            Some(PlayerColor::White) => 0b11,
        };

        match stencil & 255 {
            0b_11_10_10_01 | 0b_01_10_10_11 => *white_capture_threats += 1,
            0b_10_11_11_01 | 0b_01_11_11_10 => *black_capture_threats += 1,
            _ => {}
        }

        // TODO: 5 is STENCIL_LENGTH - 1
        if i >= 5 {
            h += coeffs[stencil_index(stencil & 0b_11_11_11_11_11_11)];
        }
    }

    h
}
