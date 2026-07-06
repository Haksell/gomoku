use crate::{
    game::{
        Game,
        board::{BOARD_SIZE, DIRECTIONS4, DIRECTIONS8, Direction, Position, is_same_color},
    },
    player::PlayerColor,
};
use std::collections::HashSet;

pub const STONES_IN_A_ROW: usize = 5;
pub const REQUIRED_CAPTURES: usize = 5;

// TODO: no HashSet (always small so array or vec)
pub type ForcedMoves = HashSet<Position>;
pub type WinningAlignments = Vec<Vec<(usize, usize)>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    Playing(ForcedMoves),
    Draw,
    Won(PlayerColor, WinningWay),
}

impl GameState {
    pub fn init() -> Self {
        Self::Playing(ForcedMoves::new())
    }

    pub const fn is_playing(&self) -> bool {
        matches!(self, Self::Playing(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WinningWay {
    pub win_by_captures: bool,
    pub winning_alignments: WinningAlignments,
}

impl Game {
    pub fn update_state(&self, pos: Position) -> GameState {
        let captures = match self.current_color {
            PlayerColor::Black => self.black_captures,
            PlayerColor::White => self.white_captures,
        };

        let mut alignments = Vec::new();
        for &dir in &DIRECTIONS4 {
            let longest_row_in_dir = self.get_longest_row_in_dir(pos, dir);
            if longest_row_in_dir.len() >= STONES_IN_A_ROW {
                alignments.push(longest_row_in_dir);
            }
        }

        if captures >= REQUIRED_CAPTURES {
            return GameState::Won(
                self.current_color,
                WinningWay { win_by_captures: true, winning_alignments: alignments },
            );
        }

        let mut forced_moves = HashSet::new();
        for alignment in &mut alignments {
            let break_possibilities = self.get_break_possibilities(alignment);
            if forced_moves.is_empty() {
                forced_moves = break_possibilities;
            } else {
                forced_moves.retain(|item| break_possibilities.contains(item));
            }
            if forced_moves.is_empty() {
                return GameState::Won(
                    self.current_color,
                    WinningWay { win_by_captures: false, winning_alignments: alignments },
                );
            }
        }

        if self.ply == BOARD_SIZE * BOARD_SIZE + 2 * (self.black_captures + self.white_captures) {
            return GameState::Draw;
        }

        GameState::Playing(forced_moves)
    }

    fn get_break_possibilities(&self, potential_winner: &mut [Position]) -> HashSet<Position> {
        let mut break_possibilities = HashSet::new();
        // TODO: sort before
        potential_winner.sort_unstable();
        let overflow = potential_winner.len() - STONES_IN_A_ROW;
        for &(x, y) in &potential_winner[overflow..STONES_IN_A_ROW] {
            break_possibilities.extend(self.find_breakable((x as isize, y as isize)));
        }
        break_possibilities
    }

    fn find_breakable(&self, (new_x, new_y): (isize, isize)) -> HashSet<Position> {
        let mut breaking_positions = HashSet::new();
        for (dx, dy) in &DIRECTIONS8 {
            if is_same_color(&self.board, None, (new_x - dx, new_y - dy))
                && is_same_color(&self.board, Some(self.current_color), (new_x + dx, new_y + dy))
                && is_same_color(
                    &self.board,
                    Some(!self.current_color),
                    (new_x + 2 * dx, new_y + 2 * dy),
                )
            {
                breaking_positions.insert(((new_x - dx) as usize, (new_y - dy) as usize));
            }
            if is_same_color(&self.board, Some(!self.current_color), (new_x - dx, new_y - dy))
                && is_same_color(&self.board, Some(self.current_color), (new_x + dx, new_y + dy))
                && is_same_color(&self.board, None, (new_x + 2 * dx, new_y + 2 * dy))
            {
                breaking_positions.insert(((new_x + 2 * dx) as usize, (new_y + 2 * dy) as usize));
            }
        }
        breaking_positions
    }

    fn get_longest_row_in_dir(&self, (x, y): Position, (dx, dy): Direction) -> Vec<Position> {
        let mut row: Vec<Position> = vec![(x, y)];

        let mut advance = |reverse: bool| {
            for mut step in 1..STONES_IN_A_ROW as isize {
                if reverse {
                    step = -step;
                }
                let new_x = x as isize + step * dx;
                let new_y = y as isize + step * dy;
                if is_same_color(&self.board, Some(self.current_color), (new_x, new_y)) {
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
