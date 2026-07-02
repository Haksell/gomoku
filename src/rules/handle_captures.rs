use super::{DIRECTIONS8, is_capture};
use crate::state::Game;

pub fn handle_captures(state: &mut Game, x: usize, y: usize) {
    let player = state.current_color;
    let total_captures = DIRECTIONS8
        .iter()
        .filter(|&&(dx, dy)| {
            let is_capture = is_capture(&state.board, player, x, y, dx, dy);
            if is_capture {
                let (x, y) = (x as isize, y as isize);
                state.board[(y + dy) as usize][(x + dx) as usize] = None;
                state.board[(y + 2 * dy) as usize][(x + 2 * dx) as usize] = None;
            }
            is_capture
        })
        .count();
    player.increment_captures(state, total_captures);
}
