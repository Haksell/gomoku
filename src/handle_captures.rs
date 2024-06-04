use crate::{
    constants::{BOARD_SIZE, DIRECTIONS},
    Board, Model, Player,
};

// TODO: in src/rules/mod.rs and remove from check_winner
fn is_same_player(board: &Board, player: Player, x: isize, y: isize) -> bool {
    x >= 0
        && y >= 0
        && x < BOARD_SIZE as isize
        && y < BOARD_SIZE as isize
        && board[y as usize][x as usize] == player
}

fn is_capture(board: &mut Board, player: Player, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    let (x, y) = (x as isize, y as isize);
    let (x1, y1) = (x + dx, y + dy);
    let (x2, y2) = (x + 2 * dx, y + 2 * dy);
    let (x3, y3) = (x + 3 * dx, y + 3 * dy);

    let is_capture = is_same_player(board, player, x3, y3)
        && is_same_player(board, player.opponent(), x2, y2)
        && is_same_player(board, player.opponent(), x1, y1);
    if is_capture {
        board[y1 as usize][x1 as usize] = Player::None;
        board[y2 as usize][x2 as usize] = Player::None;
    }
    is_capture
}

pub fn handle_captures(model: &mut Model, x: usize, y: usize) {
    let player = model.current_player;
    let total_captures = DIRECTIONS
        .into_iter()
        .map(|(dx, dy)| {
            is_capture(&mut model.board, player, x, y, dx, dy) as usize
                + is_capture(&mut model.board, player, x, y, -dx, -dy) as usize
        })
        .sum();
    player.increment_captures(model, total_captures);
}
