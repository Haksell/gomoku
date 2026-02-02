mod check_winner;
mod creates_double_three;
mod handle_captures;

pub use self::{
    check_winner::check_winner, creates_double_three::creates_double_three,
    handle_captures::handle_captures,
};
use crate::{constants::BOARD_SIZE, model::Board, turn::Turn};

// TODO: all same prototype (no Model)

fn is_same_player(board: &Board, player: Turn, x: isize, y: isize) -> bool {
    x >= 0
        && y >= 0
        && x < BOARD_SIZE as isize
        && y < BOARD_SIZE as isize
        && board[y as usize][x as usize] == player
}

fn is_capture(board: &Board, player: Turn, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    let (x, y) = (x as isize, y as isize);
    is_same_player(board, player, x + 3 * dx, y + 3 * dy)
        && is_same_player(board, player.opponent(), x + 2 * dx, y + 2 * dy)
        && is_same_player(board, player.opponent(), x + dx, y + dy)
}
