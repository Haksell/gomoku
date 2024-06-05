use crate::constants::{BOARD_SIZE, CELL_SIZE, HALF_BOARD_SIZE};
use crate::model::Model;
use crate::player::Player;
use crate::rules::creates_double_three;
use nannou::App;

pub fn mouse_to_board(app: &App, model: &Model) -> Option<(usize, usize)> {
    let mouse_pos = app.mouse.position();
    let x = (mouse_pos.x / CELL_SIZE).round() as isize + HALF_BOARD_SIZE as isize;
    let y = (mouse_pos.y / CELL_SIZE).round() as isize + HALF_BOARD_SIZE as isize;
    if x < 0 || y < 0 {
        return None;
    }
    let (x, y) = (x as usize, y as usize);
    if x >= BOARD_SIZE
        || y >= BOARD_SIZE
        || model.board[y][x] != Player::None
        || creates_double_three(&model.board, model.current_player, x, y)
    {
        return None;
    }
    Some((x, y))
}

pub fn board_to_physical(x: usize, y: usize) -> (f32, f32) {
    fn b2p1d(z: usize) -> f32 {
        (z as isize - HALF_BOARD_SIZE as isize) as f32 * CELL_SIZE
    }

    (b2p1d(x), b2p1d(y))
}
