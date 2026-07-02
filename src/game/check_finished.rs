use crate::{
    game::{
        Game, Position,
        board::{BOARD_SIZE, DIRECTIONS4, DIRECTIONS8, is_same_color},
    },
    player::PlayerColor,
};
use std::collections::HashSet;

const STONES_IN_A_ROW: usize = 5;
const REQUIRED_CAPTURES: usize = 5;

impl Game {
    pub fn check_winner(&self, x: usize, y: usize) -> (bool, HashSet<Position>) {
        let captures = match self.current_color {
            PlayerColor::Black => self.black_captures,
            PlayerColor::White => self.white_captures,
        };
        if captures >= REQUIRED_CAPTURES {
            return (true, HashSet::new());
        }

        let mut potential_winners = Vec::new();
        for &(dx, dy) in &DIRECTIONS4 {
            let longest_row_in_dir = self.get_longest_row_in_dir(x, y, dx, dy);
            if longest_row_in_dir.len() >= STONES_IN_A_ROW {
                potential_winners.push(longest_row_in_dir);
            }
        }

        let mut breakable_positions = HashSet::new();
        for potential_winner in &mut potential_winners {
            let break_possibilities = self.get_break_possibilities(potential_winner);
            if break_possibilities.is_empty() {
                return (true, breakable_positions);
            }
            if breakable_positions.is_empty() {
                breakable_positions = break_possibilities;
            } else {
                breakable_positions.retain(|item| break_possibilities.contains(item));
            }
            if breakable_positions.is_empty() {
                return (true, breakable_positions);
            }
        }

        (false, breakable_positions)
    }

    pub const fn check_draw(&self) -> bool {
        self.plies == BOARD_SIZE * BOARD_SIZE + 2 * (self.black_captures + self.white_captures)
    }

    fn get_break_possibilities(&self, potential_winner: &mut [Position]) -> HashSet<Position> {
        let mut break_possibilities: HashSet<Position> = HashSet::new();
        // TODO: sort before
        potential_winner.sort_unstable();
        let overflow = potential_winner.len() - STONES_IN_A_ROW;
        for &(x, y) in &potential_winner[overflow..STONES_IN_A_ROW] {
            break_possibilities.extend(self.find_breakable(x as isize, y as isize));
        }
        break_possibilities
    }

    fn find_breakable(&self, new_x: isize, new_y: isize) -> HashSet<Position> {
        let mut breaking_positions: HashSet<Position> = HashSet::new();
        for (dx, dy) in &DIRECTIONS8 {
            if is_same_color(&self.board, None, new_x - dx, new_y - dy)
                && is_same_color(&self.board, Some(self.current_color), new_x + dx, new_y + dy)
                && is_same_color(
                    &self.board,
                    Some(!self.current_color),
                    new_x + 2 * dx,
                    new_y + 2 * dy,
                )
            {
                breaking_positions.insert(((new_x - dx) as usize, (new_y - dy) as usize));
            }
            if is_same_color(&self.board, Some(!self.current_color), new_x - dx, new_y - dy)
                && is_same_color(&self.board, Some(self.current_color), new_x + dx, new_y + dy)
                && is_same_color(&self.board, None, new_x + 2 * dx, new_y + 2 * dy)
            {
                breaking_positions.insert(((new_x + 2 * dx) as usize, (new_y + 2 * dy) as usize));
            }
        }
        breaking_positions
    }

    fn get_longest_row_in_dir(&self, x: usize, y: usize, dx: isize, dy: isize) -> Vec<Position> {
        let mut row: Vec<Position> = vec![(x, y)];

        let mut advance = |reverse: bool| {
            for mut step in 1..STONES_IN_A_ROW as isize {
                if reverse {
                    step = -step;
                }
                let new_x = x as isize + step * dx;
                let new_y = y as isize + step * dy;
                if is_same_color(&self.board, Some(self.current_color), new_x, new_y) {
                    row.push((new_x as usize, new_y as usize));
                } else {
                    return;
                }
            }
        };

        advance(false);
        advance(true);
        row
    }
}
