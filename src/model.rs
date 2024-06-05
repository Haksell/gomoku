use crate::constants::BOARD_SIZE;
use crate::player::Player;
use crate::rules::{check_winner, handle_captures};

pub type Board = [[Player; BOARD_SIZE]; BOARD_SIZE];

pub struct Model {
    pub board: Board,
    pub current_player: Player,
    pub winner: Player,
    pub black_captures: usize,
    pub white_captures: usize,
    pub moves: Vec<(usize, usize)>,
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
        }
    }

    /// Assumes the sequence of moves is valid
    /// TODO: avoid code repetition with mouse_pressed
    pub fn from_moves(moves: &[(usize, usize)]) -> Self {
        let mut model = Self::start();
        for &(x, y) in moves {
            model.board[y][x] = model.current_player;
            handle_captures(&mut model, x, y);
            if check_winner(&model, x, y) {
                model.winner = model.current_player;
                break;
            }
            model.current_player = model.current_player.opponent();
            model.moves.push((x, y));
        }
        model
    }
}
