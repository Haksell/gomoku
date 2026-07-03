pub mod board;
pub mod check_finished;
pub mod creates_double_three;
pub mod handle_captures;
pub mod lines;

use crate::{Player, player::PlayerColor};
use board::{BOARD_SIZE, Board, Position};
use nannou::rand::{seq::SliceRandom as _, thread_rng};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Game {
    pub state: GameState,
    pub board: Board,
    pub close_moves: [[bool; BOARD_SIZE]; BOARD_SIZE],
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
            close_moves: [[false; BOARD_SIZE]; BOARD_SIZE],
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
        self.update_close_moves(x, y);
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

    // TODO: dynamic ajustable en tout cas
    fn update_close_moves(&mut self, x: usize, y: usize) {
        const MANHATTAN2: [(isize, isize); 13] = [
            (0, 0),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 2),
            (2, 0),
            (0, -2),
            (-2, 0),
        ];

        for (dx, dy) in &MANHATTAN2 {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= BOARD_SIZE as isize || ny < 0 || ny >= BOARD_SIZE as isize {
                continue;
            }
            self.close_moves[ny as usize][nx as usize] = true;
        }
    }

    pub fn get_legal_moves(&self, max_dist: Option<usize>, shuffle: bool) -> Vec<Position> {
        // TODO: stop hardcoding 2
        debug_assert!(matches!(max_dist, None | Some(2)));
        if !self.forced_moves.is_empty() {
            return self.forced_moves.clone().into_iter().collect();
        }

        let mut legal_moves = Vec::new();
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if (max_dist.is_none() || self.close_moves[y][x])
                    && self.board[y][x].is_none()
                    && !self.creates_double_three(x, y)
                {
                    legal_moves.push((x, y));
                }
            }
        }

        if shuffle {
            let mut rng = thread_rng();
            legal_moves.shuffle(&mut rng);
        }

        legal_moves
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
