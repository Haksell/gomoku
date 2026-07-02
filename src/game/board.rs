use crate::player::PlayerColor;

pub type Board = [[Option<PlayerColor>; BOARD_SIZE]; BOARD_SIZE];
pub type Position = (usize, usize); // TODO: !usize

pub const BOARD_SIZE: usize = 19;
pub const HALF_BOARD_SIZE: usize = BOARD_SIZE / 2;
pub const BOARD_CENTER: Position = (HALF_BOARD_SIZE, HALF_BOARD_SIZE);

pub fn is_same_player(board: &Board, player: Option<PlayerColor>, x: isize, y: isize) -> bool {
    x >= 0
        && y >= 0
        && x < BOARD_SIZE as isize
        && y < BOARD_SIZE as isize
        && board[y as usize][x as usize] == player
}

pub fn is_capture(
    board: &Board,
    player_color: PlayerColor,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> bool {
    let (x, y) = (x as isize, y as isize);
    is_same_player(board, Some(player_color), x + 3 * dx, y + 3 * dy)
        && is_same_player(board, Some(!player_color), x + 2 * dx, y + 2 * dy)
        && is_same_player(board, Some(!player_color), x + dx, y + dy)
}
