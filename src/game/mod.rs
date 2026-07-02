pub mod board;
pub mod check_finished;
pub mod creates_double_three;
pub mod handle_captures;

use crate::{Player, player::PlayerColor};
use board::{BOARD_SIZE, Board, Position};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Game {
    pub state: GameState,
    pub board: Board,
    pub current_color: PlayerColor,
    pub black_captures: usize,
    pub white_captures: usize,
    pub forced_moves: HashSet<Position>,
    pub black_player: Player,
    pub white_player: Player,
    // TODO: outside this struct
    pub plies: usize,
    // pub moves: Vec<Position>,
}

impl Game {
    pub fn new(black_player: Player, white_player: Player) -> Self {
        Self {
            state: GameState::Playing,
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            current_color: PlayerColor::Black,
            black_captures: 0,
            white_captures: 0,
            forced_moves: HashSet::new(),
            black_player,
            white_player,
            plies: 0,
        }
    }

    /// Assumes the move is valid.
    pub fn do_move(&mut self, x: usize, y: usize) {
        self.plies += 1;

        self.board[y][x] = Some(self.current_color);
        self.handle_captures(x, y);

        let (is_winner, forced_moves) = self.check_winner(x, y);
        if is_winner {
            self.state = GameState::Won(self.current_color);
        } else if self.check_draw() {
            self.state = GameState::Draw;
        } else {
            self.forced_moves = forced_moves;
        }

        self.current_color = !self.current_color;
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
        while self.state.is_playing() {
            let Player::Bot { bot, heuristic } = self.current_player() else { unreachable!() };
            let (x, y) = bot(self, *heuristic);
            self.do_move(x, y);
        }
    }
}

#[derive(Clone, Copy)]
pub enum GameState {
    Playing,
    Draw,
    Won(PlayerColor),
}

impl GameState {
    pub const fn is_playing(self) -> bool {
        matches!(self, Self::Playing)
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
