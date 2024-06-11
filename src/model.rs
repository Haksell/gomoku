use crate::constants::BOARD_SIZE;
use crate::player::Player;
use crate::rules::{check_winner, handle_captures};
use crate::turn::Turn;
use std::collections::HashSet;

pub type Board = [[Turn; BOARD_SIZE]; BOARD_SIZE];

#[derive(Clone)]
pub struct Model {
    pub board: Board,
    pub black_player: Player,
    pub white_player: Player,
    pub current_turn: Turn,
    pub winner: Turn,
    pub black_captures: usize,
    pub white_captures: usize,
    pub moves: Vec<(usize, usize)>,
    pub forced_moves: HashSet<(usize, usize)>,
    pub hover: Option<(usize, usize)>,
}

impl Model {
    pub fn start() -> Self {
        let mut args = std::env::args();
        let black_arg = args.nth(1);
        let white_arg = args.next();
        assert!(black_arg.is_some() && white_arg.is_some() && args.next().is_none());
        Self {
            board: [[Turn::None; BOARD_SIZE]; BOARD_SIZE],
            black_player: Player::from_arg(&black_arg.unwrap()),
            white_player: Player::from_arg(&white_arg.unwrap()),
            current_turn: Turn::Black,
            winner: Turn::None,
            black_captures: 0,
            white_captures: 0,
            moves: Vec::new(),
            forced_moves: HashSet::new(),
            hover: None,
        }
    }

    /// Assumes the move is valid
    pub fn do_move(&mut self, x: usize, y: usize) {
        self.board[y][x] = self.current_turn;
        handle_captures(self, x, y);
        let (is_winner, forced_moves) = check_winner(self, x, y);
        if is_winner {
            self.winner = self.current_turn;
            // self.forced_moves.clear(); ???
            // self.current_turn = Turn::None; ???
        } else {
            self.forced_moves = forced_moves;
            self.current_turn = self.current_turn.opponent();
        }
        self.moves.push((x, y));
    }

    /// Assumes the move is valid
    /// TODO: use for backspace
    /// TODO: undo captures
    // pub fn undo_move(&self, x: usize, y: usize) {
    //     self.board[y][x] = Turn::None;
    //     if self.winner != Turn::None {
    //         self.winner = Turn::None;
    //     } else {
    //         self.current_turn = self.current_turn.opponent();
    //     }
    //     self.forced_moves.clear();
    //     self.moves.pop();
    // }

    /// Assumes the sequence of moves is valid
    pub fn from_moves(moves: &[(usize, usize)]) -> Self {
        let mut model = Self::start();
        for &(x, y) in moves {
            model.do_move(x, y);
        }
        model
    }
}
