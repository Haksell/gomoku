use crate::{
    game::board::{BOARD_SIZE, HALF_BOARD_SIZE, Position},
    gui::{CELL_SIZE, Model},
};
use nannou::App;

// TODO: physical_to_board
pub fn mouse_to_board(app: &App, model: &Model) -> Option<Position> {
    fn split_float(z: f32) -> (isize, f32) {
        let pos = z / CELL_SIZE;
        let pos_round = pos.round();
        (pos_round as isize + HALF_BOARD_SIZE as isize, (pos - pos_round).abs())
    }

    let mouse_pos = app.mouse.position();
    let (x, xd) = split_float(mouse_pos.x);
    let (y, yd) = split_float(mouse_pos.y);
    let intersection_distance = f32::hypot(xd, yd);
    if x < 0 || y < 0 || intersection_distance > 0.4 {
        return None;
    }
    let (x, y) = (x as usize, y as usize);
    if x >= BOARD_SIZE
        || y >= BOARD_SIZE
        || model.game.board[y][x].is_some()
        || model.game.creates_double_three(x, y)
        || (!model.game.forced_moves.is_empty() && !model.game.forced_moves.contains(&(x, y)))
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
