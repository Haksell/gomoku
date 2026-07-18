use crate::{
    game::{
        Game, UpdateSign,
        board::{DIRECTIONS8, Position, is_capture},
    },
    player::PlayerColor,
};

impl Game {
    pub fn handle_captures(&mut self, (x, y): Position) {
        for (dx, dy) in DIRECTIONS8 {
            if !is_capture(&self.board, self.current_color, (x, y), (dx, dy)) {
                continue;
            }

            let (x, y) = (x as isize, y as isize);
            let captured1 = ((x + dx) as usize, (y + dy) as usize);
            let captured2 = ((x + 2 * dx) as usize, (y + 2 * dy) as usize);

            self.board.set(captured1, None);
            self.board.set(captured2, None);
            self.update_close_moves(captured1, UpdateSign::Negative);
            self.update_close_moves(captured2, UpdateSign::Negative);

            match self.current_color {
                PlayerColor::Black => self.black_captures += 1,
                PlayerColor::White => self.white_captures += 1,
            }

            self.captures.push((self.ply, captured1, captured2));
        }
    }
}
