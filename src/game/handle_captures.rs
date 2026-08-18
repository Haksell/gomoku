use crate::{
    game::{
        Game, UpdateSign,
        bitboard::{bitboard_set, bitboard_set_captures},
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
            let captured_x1 = (x + dx) as usize;
            let captured_y1 = (y + dy) as usize;
            let captured1 = (captured_x1, captured_y1);
            let captured_x2 = (x + 2 * dx) as usize;
            let captured_y2 = (y + 2 * dy) as usize;
            let captured2 = (captured_x2, captured_y2);

            self.board[captured_y1][captured_x1] = None;
            self.board[captured_y2][captured_x2] = None;
            bitboard_set(&mut self.bitboard, captured1, None);
            bitboard_set(&mut self.bitboard, captured2, None);
            self.update_close_moves(captured1, UpdateSign::Negative);
            self.update_close_moves(captured2, UpdateSign::Negative);

            match self.current_color {
                PlayerColor::Black => self.black_captures += 1,
                PlayerColor::White => self.white_captures += 1,
            }
            bitboard_set_captures(&mut self.bitboard, self.black_captures, self.white_captures);

            self.captures.push((self.ply, captured1, captured2));
        }
    }
}
