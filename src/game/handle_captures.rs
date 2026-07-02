use crate::game::{
    Game,
    board::{DIRECTIONS8, is_capture},
};

impl Game {
    pub fn handle_captures(&mut self, x: usize, y: usize) {
        let total_captures = DIRECTIONS8
            .iter()
            .filter(|&&(dx, dy)| {
                let is_capture = is_capture(&self.board, self.current_color, x, y, dx, dy);
                if is_capture {
                    let (x, y) = (x as isize, y as isize);
                    self.board[(y + dy) as usize][(x + dx) as usize] = None;
                    self.board[(y + 2 * dy) as usize][(x + 2 * dx) as usize] = None;
                }
                is_capture
            })
            .count();
        self.current_color.increment_captures(self, total_captures);
    }
}
