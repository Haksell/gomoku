use crate::constants::{BOARD_SIZE, CELL_SIZE, HALF_BOARD_SIZE};
use crate::model::Model;
use crate::rules::creates_double_three;
use crate::turn::Turn;
use nannou::App;

pub fn mouse_to_board(app: &App, model: &Model) -> Option<(usize, usize)> {
    fn split_float(z: f32) -> (isize, f32) {
        let pos = z / CELL_SIZE;
        let pos_round = pos.round();
        (
            pos_round as isize + HALF_BOARD_SIZE as isize,
            (pos - pos_round).abs(),
        )
    }

    let mouse_pos = app.mouse.position();
    let (x, xd) = split_float(mouse_pos.x);
    let (y, yd) = split_float(mouse_pos.y);
    let intersection_distance = (xd * xd + yd * yd).sqrt();
    if x < 0 || y < 0 || intersection_distance > 0.4 {
        return None;
    }
    let (x, y) = (x as usize, y as usize);
    if x >= BOARD_SIZE
        || y >= BOARD_SIZE
        || model.board[y][x] != Turn::None
        || creates_double_three(&model.board, model.current_turn, x, y)
        || (!model.forced_moves.is_empty() && !model.forced_moves.contains(&(x, y)))
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
