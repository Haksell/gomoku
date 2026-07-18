use crate::player::PlayerColor;
use nannou::math::num_traits::Euclid as _;
use std::fmt::{Display, Write};

pub type Board = [[Option<PlayerColor>; BOARD_SIZE]; BOARD_SIZE];
pub type Position = (usize, usize);
pub type Direction = (isize, isize);

pub const BOARD_SIZE: usize = 19;
pub const HALF_BOARD_SIZE: usize = BOARD_SIZE / 2;
pub const BOARD_CENTER: Position = (HALF_BOARD_SIZE, HALF_BOARD_SIZE);

pub const DIRECTIONS4: [Direction; 4] = [(0, 1), (1, 1), (1, 0), (1, -1)];
pub const DIRECTIONS8: [Direction; 8] =
    [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

pub const MANHATTAN_TO_CENTER: [[u64; BOARD_SIZE]; BOARD_SIZE] = {
    let mut out = [[0; BOARD_SIZE]; BOARD_SIZE];
    let mut y = 0;
    while y < BOARD_SIZE {
        let dy = usize::abs_diff(y, HALF_BOARD_SIZE);
        let mut x = 0;
        while x < BOARD_SIZE {
            let dx = usize::abs_diff(x, HALF_BOARD_SIZE);
            out[y][x] = (dx + dy) as u64;
            x += 1;
        }
        y += 1;
    }
    out
};

pub struct BitBoard {
    board: [u64; 12],
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "╔{}╗", "═".repeat(3 * BOARD_SIZE + 1))?;
        for y in (0..BOARD_SIZE).rev() {
            f.write_str("║ ")?;
            for x in 0..BOARD_SIZE {
                let s = match self.get((x, y)) {
                    Some(PlayerColor::Black) => "⚫",
                    Some(PlayerColor::White) => "⚪",
                    None => "  ",
                };
                f.write_str(s)?;
                f.write_char(' ')?;
            }
            f.write_str("║\n")?;
        }
        write!(f, "╚{}╝", "═".repeat(3 * BOARD_SIZE + 1))?;
        Ok(())
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        Self { board: [0x5555_5555_5555_5555; 12] }
    }
}

impl BitBoard {
    pub fn get(&self, (x, y): Position) -> Option<PlayerColor> {
        let cell_idx = 2 * (BOARD_SIZE * y + x);
        let (arr_idx, shift) = cell_idx.div_rem_euclid(&64);
        let cell = (self.board[arr_idx] >> shift) & 0b11;
        match cell {
            0b01 => None,
            0b10 => Some(PlayerColor::Black),
            0b11 => Some(PlayerColor::White),
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, (x, y): Position, player_color: Option<PlayerColor>) {
        let val = match player_color {
            None => 0b01,
            Some(PlayerColor::Black) => 0b10,
            Some(PlayerColor::White) => 0b11,
        };

        let cell_idx = 2 * (BOARD_SIZE * y + x);
        let (arr_idx, shift) = cell_idx.div_rem_euclid(&64);
        let mask = !(0b11 << shift);
        self.board[arr_idx] = (self.board[arr_idx] & mask) | (val << shift);
    }
}

#[expect(unused)]
pub fn print_board(board: &Board) {
    for row in board {
        for player_color in row {
            print!(
                "{}",
                match player_color {
                    Some(PlayerColor::Black) => 'B',
                    Some(PlayerColor::White) => 'W',
                    None => '.',
                }
            );
        }
        println!();
    }
}

const fn bubble_sort<const N: usize>(arr: &mut [Position; N]) {
    let mut i = 0;
    while i < N {
        let mut j = 1;
        while j < N {
            let (prev_x, prev_y) = arr[j - 1];
            let (curr_x, curr_y) = arr[j];
            let prev_dist = MANHATTAN_TO_CENTER[prev_y][prev_x];
            let curr_dist = MANHATTAN_TO_CENTER[curr_y][curr_x];
            if prev_dist > curr_dist {
                let left = arr[j - 1];
                let right = arr[j];
                arr[j - 1] = right;
                arr[j] = left;
            }
            j += 1;
        }
        i += 1;
    }
}

pub const SPIRALLING_POSITIONS: [Position; BOARD_SIZE * BOARD_SIZE] = {
    let mut out = [(0, 0); BOARD_SIZE * BOARD_SIZE];
    let mut y = 0;
    while y < BOARD_SIZE {
        let mut x = 0;
        while x < BOARD_SIZE {
            out[y * BOARD_SIZE + x] = (x, y);
            x += 1;
        }
        y += 1;
    }
    bubble_sort(&mut out);
    out
};

pub fn is_same_color(board: &Board, player: Option<PlayerColor>, (x, y): (isize, isize)) -> bool {
    x >= 0
        && y >= 0
        && x < BOARD_SIZE as isize
        && y < BOARD_SIZE as isize
        && board[y as usize][x as usize] == player
}

pub fn is_capture(
    board: &Board,
    player_color: PlayerColor,
    (x, y): Position,
    (dx, dy): Direction,
) -> bool {
    let (x, y) = (x as isize, y as isize);
    is_same_color(board, Some(player_color), (x + 3 * dx, y + 3 * dy))
        && is_same_color(board, Some(!player_color), (x + 2 * dx, y + 2 * dy))
        && is_same_color(board, Some(!player_color), (x + dx, y + dy))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions() {
        assert!(DIRECTIONS8[..4] == DIRECTIONS4);
        assert!(
            DIRECTIONS8[4..]
                .iter()
                .zip(DIRECTIONS4)
                .all(|(&(x1, y1), (x2, y2))| x1 == -x2 && y1 == -y2)
        );
    }
}
