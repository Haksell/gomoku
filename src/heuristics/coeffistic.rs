use crate::{
    game::{
        Game,
        board::{Board, Position},
        lines::{COLUMNS, DOWNWARD_DIAGONALS, ROWS, UPWARD_DIAGONALS},
    },
    player::PlayerColor,
};
use std::{
    fs::File,
    io::{self, Write as _},
    sync::LazyLock,
};

pub const STENCIL_SIZE: usize = 6;

pub const COEFFS_FILE: &str = match STENCIL_SIZE {
    6 => "./coeffs/grid_search_6.rs",
    7 => "./coeffs/grid_search_7.rs",
    _ => unreachable!(),
};
pub static INITIAL_COEFFS: LazyLock<Coeffs> = LazyLock::new(|| match STENCIL_SIZE {
    // include! needs a literal, so we can't give it COEFFS_FILE
    6 => include!("../../coeffs/grid_search_6.rs"),
    7 => include!("../../coeffs/grid_search_7.rs"),
    _ => unreachable!(),
});

pub const N_STENCIL_COEFFS: usize = 3usize.pow(STENCIL_SIZE as u32);
pub const N_COEFFS: usize = N_STENCIL_COEFFS + 9;
pub type Coeffs = Box<[i64]>;

static STENCIL_INDEX_MAPPING: [usize; 1 << (2 * STENCIL_SIZE)] = {
    let mut res = [usize::MAX; 1 << (2 * STENCIL_SIZE)];
    let mut base4 = 0;
    while base4 < res.len() {
        let mut n = base4;
        let mut base3 = 0;
        let mut is_valid = true;
        while n > 0 {
            let bits = n & 0b11;
            if bits == 0 {
                is_valid = false;
                break;
            }
            base3 = 3 * base3 + bits - 1;
            n >>= 2;
        }
        if is_valid {
            res[base4] = base3;
        }
        base4 += 1;
    }
    res
};

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

fn evaluate_patterns(
    board: &Board,
    line: &[Position],
    coeffs: &Coeffs,
    black_capture_threats: &mut i64,
    white_capture_threats: &mut i64,
) -> i64 {
    const MASK: usize = (1 << (STENCIL_SIZE * 2 - 2)) - 1;

    let mut stencil = 0;
    let mut h = 0;

    for (i, &(x, y)) in line.iter().enumerate() {
        let player_color = board[y][x];
        let new_bits = match player_color {
            None => 0b01,
            Some(PlayerColor::Black) => 0b10,
            Some(PlayerColor::White) => 0b11,
        };
        stencil = ((stencil & MASK) << 2) | new_bits;

        match stencil & 255 {
            0b_11_10_10_01 | 0b_01_10_10_11 => *white_capture_threats += 1,
            0b_10_11_11_01 | 0b_01_11_11_10 => *black_capture_threats += 1,
            _ => {}
        }

        if i >= STENCIL_SIZE - 1 {
            h += coeffs[STENCIL_INDEX_MAPPING[stencil]];
        }
    }

    h
}

pub fn write_coeffs(coeffs: &[i64]) -> io::Result<()> {
    const CAPACITY: usize = match STENCIL_SIZE {
        6 => 1 << 15,
        7 => 1 << 17,
        _ => unreachable!(),
    };

    let mut buf = io::BufWriter::with_capacity(CAPACITY, Vec::new());
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
