use super::{DIRECTIONS8, is_capture};
use crate::game::Game;

pub fn handle_captures(model: &mut Game, x: usize, y: usize) {
    let player = model.current_color;
    let total_captures = DIRECTIONS8
        .iter()
        .filter(|&&(dx, dy)| {
            let is_capture = is_capture(&model.board, player, x, y, dx, dy);
            if is_capture {
                let (x, y) = (x as isize, y as isize);
                model.board[(y + dy) as usize][(x + dx) as usize] = None;
                model.board[(y + 2 * dy) as usize][(x + 2 * dx) as usize] = None;
            }
            is_capture
        })
        .count();
    player.increment_captures(model, total_captures);
}
