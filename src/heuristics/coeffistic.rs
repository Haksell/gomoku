use std::{
    fs::File,
    io::{self, Write as _},
    sync::LazyLock,
};

use crate::{
    game::{
        Game,
        board::{Board, Position},
        lines::{COLUMNS, DOWNWARD_DIAGONALS, ROWS, UPWARD_DIAGONALS},
    },
    player::PlayerColor,
};

pub const COEFFS_FILE: &str = "./coeffs/current.rs";
pub static INITIAL_COEFFS: LazyLock<Coeffs> = LazyLock::new(|| include!("../../coeffs/current.rs"));
pub const STENCIL_SIZE: usize = 7;
pub const N_STENCIL_COEFFS: usize = 3usize.pow(STENCIL_SIZE as u32);
pub const N_COEFFS: usize = N_STENCIL_COEFFS + 9;
pub type Coeffs = Box<[i64]>;

pub fn coeffistic(game: &Game, coeffs: Option<&Coeffs>) -> i64 {
    let mut h = 0;
    let mut black_capture_threats = 0;
    let mut white_capture_threats = 0;
    let coeffs = coeffs.unwrap();

    for lines in [ROWS, COLUMNS] {
        for line in &lines {
            h += evaluate_patterns(
                &game.board,
                line,
                coeffs,
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
                coeffs,
                &mut black_capture_threats,
                &mut white_capture_threats,
            );
        }
    }

    h += capture_heuristic(coeffs, game.black_captures as i64, black_capture_threats);
    h -= capture_heuristic(coeffs, game.white_captures as i64, white_capture_threats);

    h
}

const fn capture_heuristic(coeffs: &Coeffs, c: i64, t: i64) -> i64 {
    let i = N_STENCIL_COEFFS;
    let h_captures = coeffs[i] * c * c * c + coeffs[i + 1] * c * c + coeffs[i + 2] * c;
    let h_threats = coeffs[i + 3] * t * t * t + coeffs[i + 4] * t * t + coeffs[i + 5] * t;
    let cross_terms = c * t * (coeffs[i + 6] + coeffs[i + 7] * c + coeffs[i + 8] * t);
    h_captures + h_threats + cross_terms
}

fn stencil_index(mut stencil: i64) -> usize {
    let mut index = 0;
    for _ in 0..STENCIL_SIZE {
        index *= 3;
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

        if i >= STENCIL_SIZE - 1 {
            h += coeffs[stencil_index(stencil)];
        }
    }

    h
}

pub fn write_coeffs(coeffs: &[i64]) -> io::Result<()> {
    // TODO: compute correct size from STENCIL_SIZE
    let mut buf = io::BufWriter::with_capacity(1 << 17, Vec::new());
    writeln!(buf, "vec![")?;

    for i in 0..N_STENCIL_COEFFS {
        let c = coeffs[i];
        // TODO: check correct direction (might be symmetric)
        let pat: String =
            (0..STENCIL_SIZE).map(|j| ['.', 'b', 'w'][i / 3usize.pow(j as u32) % 3]).collect();
        let num = format!("{c},");
        writeln!(buf, "    {num:7}// {pat}")?;
    }

    for (i, poly_coeff) in
        ["ccc", "cc", "c", "ttt", "tt", "t", "ct", "cct", "ctt"].iter().enumerate()
    {
        let c = coeffs[N_STENCIL_COEFFS + i];
        let num = format!("{c},");
        writeln!(buf, "    {num:7}// {poly_coeff}")?;
    }

    writeln!(buf, "].into_boxed_slice()")?;

    let mut file = File::create(COEFFS_FILE)?;
    file.write_all(buf.buffer())
}
