use crate::{
    Player,
    constants::BOARD_SIZE,
    player::PlayerColor,
    rules::{check_winner, handle_captures},
};
use std::collections::HashSet;

pub type Board = [[Option<PlayerColor>; BOARD_SIZE]; BOARD_SIZE];
pub type Position = (usize, usize); // TODO: !usize 

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub current_color: PlayerColor,
    // TODO: is_game_over instead of Option because of redundancy with current_color
    pub winner: Option<PlayerColor>,
    pub black_captures: usize,
    pub white_captures: usize,
    pub moves: Vec<Position>,
    pub forced_moves: HashSet<Position>,
    pub black_player: Player,
    pub white_player: Player,
}

#[derive(Clone)]
pub struct Model {
    pub game: Game,
    pub hover: Option<Position>,
    pub ai_thinking_time: Option<u128>,
}

impl Model {
    pub fn new(black_player: Player, white_player: Player) -> Self {
        Self { game: Game::new(black_player, white_player), hover: None, ai_thinking_time: None }
    }
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
        handle_captures(self, x, y);
        let (is_winner, forced_moves) = check_winner(self, x, y);
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
