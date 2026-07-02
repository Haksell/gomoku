pub mod board;
pub mod check_winner;
pub mod creates_double_three;
pub mod handle_captures;

use crate::{Player, player::PlayerColor};
use board::{BOARD_SIZE, Board, Position};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub current_color: PlayerColor,
    // TODO: is_game_over instead of Option because of redundancy with current_color
    pub winner: Option<PlayerColor>,
    pub black_captures: usize,
    pub white_captures: usize,
    // TODO: outside this struct
    pub moves: Vec<Position>,
    pub forced_moves: HashSet<Position>,
    pub black_player: Player,
    pub white_player: Player,
}

impl Game {
    pub fn new(black_player: Player, white_player: Player) -> Self {
        Self {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            current_color: PlayerColor::Black,
            winner: None,
            black_captures: 0,
            white_captures: 0,
            moves: Vec::new(),
            forced_moves: HashSet::new(),
            black_player,
            white_player,
        }
    }

    /// Assumes the move is valid.
    pub fn do_move(&mut self, x: usize, y: usize) {
        self.board[y][x] = Some(self.current_color);
        self.handle_captures(x, y);
        let (is_winner, forced_moves) = self.check_winner(x, y);
        if is_winner {
            self.winner = Some(self.current_color);
            // self.forced_moves.clear(); ???
        } else {
            self.forced_moves = forced_moves;
            self.current_color = !self.current_color;
        }
        self.moves.push((x, y));
    }

    // Assumes the move is valid
    // TODO: use for backspace
    // TODO: undo captures
    // pub fn undo_move(&self, x: usize, y: usize) {
    //     self.board[y][x] = Turn::None;
    //     if self.winner != Turn::None {
    //         self.winner = Turn::None;
    //     } else {
    //         self.current_player = !self.current_player;
    //     }
    //     self.forced_moves.clear();
    //     self.moves.pop();
    // }

    // /// Assumes the sequence of moves is valid.
    // pub fn from_moves(black_player: Player, white_player: Player, moves: &[Position]) -> Self {
    //     let mut model = Self::new(black_player, white_player);
    //     for &(x, y) in moves {
    //         model.do_move(x, y);
    //     }
    //     model
    // }

    pub const fn current_player(&self) -> &Player {
        match self.current_color {
            PlayerColor::Black => &self.black_player,
            PlayerColor::White => &self.white_player,
        }
    }

    pub fn play_game(&mut self) {
        assert!(self.black_player.is_bot());
        assert!(self.white_player.is_bot());

        // TODO: handle draws properly
        while self.winner.is_none() {
            let Player::Bot { bot, heuristic } = self.current_player() else { unreachable!() };
            let (x, y) = bot(self, *heuristic);
            self.do_move(x, y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_size() {
        assert!(BOARD_SIZE % 2 == 1);
        assert!(BOARD_SIZE >= 3);
    }
}
