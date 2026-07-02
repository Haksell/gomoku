use crate::{
    game::{
        Game,
        board::{DIRECTIONS8, is_capture},
    },
    player::PlayerColor,
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

        match self.current_color {
            PlayerColor::Black => self.black_captures += total_captures,
            PlayerColor::White => self.white_captures += total_captures,
        }
    }
}
