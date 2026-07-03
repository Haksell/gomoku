use crate::{
    game::{
        Game,
        board::{DIRECTIONS8, MANHATTAN_TO_CENTER, is_capture},
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
                    let captured_x1 = (x + dx) as usize;
                    let captured_y1 = (y + dy) as usize;
                    let captured_x2 = (x + 2 * dx) as usize;
                    let captured_y2 = (y + 2 * dy) as usize;
                    self.board[captured_y1][captured_x1] = None;
                    self.board[captured_y2][captured_x2] = None;
                    let dist1 = MANHATTAN_TO_CENTER[captured_y1][captured_x1];
                    let dist2 = MANHATTAN_TO_CENTER[captured_y2][captured_x2];
                    match self.current_color {
                        PlayerColor::Black => {
                            self.white_dist_to_center -= dist1 + dist2;
                        }
                        PlayerColor::White => {
                            self.black_dist_to_center -= dist1 + dist2;
                        }
                    }
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
