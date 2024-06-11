use super::is_capture;
use crate::{constants::DIRECTIONS8, turn::Turn, Model};

pub fn handle_captures(model: &mut Model, x: usize, y: usize) {
    let player = model.current_player;
    let total_captures = DIRECTIONS8
        .iter()
        .filter(|&&(dx, dy)| {
            let is_capture = is_capture(&model.board, player, x, y, dx, dy);
            if is_capture {
                let (x, y) = (x as isize, y as isize);
                model.board[(y + dy) as usize][(x + dx) as usize] = Turn::None;
                model.board[(y + 2 * dy) as usize][(x + 2 * dx) as usize] = Turn::None;
            }
            is_capture
        })
        .count();
    player.increment_captures(model, total_captures);
}
