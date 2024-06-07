use crate::constants::BOARD_SIZE;
use crate::player::Player;
use crate::rules::{check_winner, handle_captures};
use std::collections::HashSet;

pub type Board = [[Player; BOARD_SIZE]; BOARD_SIZE];

pub struct Model {
    pub board: Board,
    pub current_player: Player,
    pub winner: Player,
    pub black_captures: usize,
    pub white_captures: usize,
    pub moves: Vec<(usize, usize)>,
    pub is_forced_move: bool,
    pub possible_moves: HashSet<(usize, usize)>,
    pub hover: Option<(usize, usize)>,
}

impl Model {
    pub fn start() -> Self {
        Self {
            board: [[Player::None; BOARD_SIZE]; BOARD_SIZE],
            current_player: Player::Black,
            winner: Player::None,
            black_captures: 0,
            white_captures: 0,
            moves: Vec::new(),
            is_forced_move: false,
            possible_moves: HashSet::new(),
            hover: None,
        }
    }

    /// Assumes the move is valid
    pub fn do_move(&mut self, x: usize, y: usize) {
        self.board[y][x] = self.current_player;
        handle_captures(self, x, y);
        let (is_winner, forced_moves) = check_winner(self, x, y);
        if is_winner {
            self.winner = self.current_player;
            // self.current_player = Player::None; ???
            println!("{:?} won.", self.winner);
        } else {
            if !forced_moves.is_empty() {
                self.is_forced_move = true;
                self.possible_moves = forced_moves;
            } else {
                self.is_forced_move = false;
                self.possible_moves.clear();
            }
            self.current_player = self.current_player.opponent();
        }
        self.moves.push((x, y));
    }

    /// Assumes the sequence of moves is valid
    pub fn from_moves(moves: &[(usize, usize)]) -> Self {
        let mut model = Self::start();
        for &(x, y) in moves {
            model.do_move(x, y);
        }
        model
    }
}
