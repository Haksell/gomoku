use crate::{
    bots::Bot,
    constants::BOARD_SIZE,
    heuristics::Heuristic,
    rules::{check_winner, handle_captures},
    turn::Turn,
};
use std::collections::HashSet;

pub type Board = [[Turn; BOARD_SIZE]; BOARD_SIZE];
pub type Position = (usize, usize); // TODO: !usize 

#[derive(Clone)]
pub struct Model {
    pub board: Board,
    pub current_player: Turn,
    pub human: Turn,
    pub winner: Turn,
    pub black_captures: usize,
    pub white_captures: usize,
    pub moves: Vec<Position>,
    pub forced_moves: HashSet<Position>,
    pub hover: Option<Position>,
    pub ai_pending_frames: u8,
    pub ai_thinking_time: Option<u128>,
    pub bot: Bot,
    pub heuristic: Heuristic,
}

impl Model {
    pub fn new(bot: Bot, heuristic: Heuristic) -> Self {
        Self {
            board: [[Turn::None; BOARD_SIZE]; BOARD_SIZE],
            current_player: Turn::Black,
            human: Turn::Black,
            winner: Turn::None,
            black_captures: 0,
            white_captures: 0,
            moves: Vec::new(),
            forced_moves: HashSet::new(),
            hover: None,
            ai_pending_frames: 0,
            ai_thinking_time: None,
            bot,
            heuristic,
        }
    }

    /// Assumes the move is valid.
    pub fn do_move(&mut self, x: usize, y: usize) {
        self.board[y][x] = self.current_player;
        handle_captures(self, x, y);
        let (is_winner, forced_moves) = check_winner(self, x, y);
        if is_winner {
            self.winner = self.current_player;
            // self.forced_moves.clear(); ???
            // self.current_player = Turn::None; ???
        } else {
            self.forced_moves = forced_moves;
            self.current_player = self.current_player.opponent();
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
    //         self.current_player = self.current_player.opponent();
    //     }
    //     self.forced_moves.clear();
    //     self.moves.pop();
    // }

    /// Assumes the sequence of moves is valid.
    pub fn from_moves(bot: Bot, heuristic: Heuristic, moves: &[Position]) -> Self {
        let mut model = Self::new(bot, heuristic);
        for &(x, y) in moves {
            model.do_move(x, y);
        }
        model
    }
}
