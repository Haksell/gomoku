mod check_winner;
mod creates_double_three;
mod handle_captures;

use crate::{constants::BOARD_SIZE, model::Board, player::Player};
pub use check_winner::check_winner;
pub use creates_double_three::creates_double_three;
pub use handle_captures::handle_captures;

// TODO: all same prototype (no Model)

fn is_same_player(board: &Board, player: Player, x: isize, y: isize) -> bool {
    x >= 0
        && y >= 0
        && x < BOARD_SIZE as isize
        && y < BOARD_SIZE as isize
        && board[y as usize][x as usize] == player
}
