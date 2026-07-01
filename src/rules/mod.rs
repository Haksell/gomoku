mod check_winner;
mod creates_double_three;
mod handle_captures;

pub use self::{
    check_winner::check_winner, creates_double_three::creates_double_three,
    handle_captures::handle_captures,
};
use crate::{constants::BOARD_SIZE, model::Board, player::PlayerColor};

pub const DIRECTIONS4: [(isize, isize); 4] = [(0, 1), (1, 1), (1, 0), (1, -1)];
pub const DIRECTIONS8: [(isize, isize); 8] =
    [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

// TODO: all same prototype (no Model)

fn is_same_player(board: &Board, player: Option<PlayerColor>, x: isize, y: isize) -> bool {
    x >= 0
        && y >= 0
        && x < BOARD_SIZE as isize
        && y < BOARD_SIZE as isize
        && board[y as usize][x as usize] == player
}

fn is_capture(
    board: &Board,
    player: PlayerColor,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> bool {
    let (x, y) = (x as isize, y as isize);
    is_same_player(board, Some(player), x + 3 * dx, y + 3 * dy)
        && is_same_player(board, Some(!player), x + 2 * dx, y + 2 * dy)
        && is_same_player(board, Some(!player), x + dx, y + dy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions() {
        assert!(DIRECTIONS8[..4] == DIRECTIONS4);
        assert!(
            DIRECTIONS8[4..]
                .iter()
                .zip(DIRECTIONS4)
                .all(|(&(x1, y1), (x2, y2))| x1 == -x2 && y1 == -y2)
        );
    }
}
