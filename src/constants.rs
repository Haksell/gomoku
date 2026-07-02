use crate::game::Position;

pub const WINDOW_SIZE: u32 = 750;
pub const WINDOW_MARGIN: f32 = WINDOW_SIZE as f32 * 0.055;
pub const CELL_SIZE: f32 = (WINDOW_SIZE as f32 - 2.0 * WINDOW_MARGIN) / (BOARD_SIZE - 1) as f32;

pub const BOARD_SIZE: usize = 19;
pub const HALF_BOARD_SIZE: usize = BOARD_SIZE / 2;
pub const BOARD_CENTER: Position = (HALF_BOARD_SIZE, HALF_BOARD_SIZE);
pub const DOT_SPACING: usize = 6;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_size() {
        assert!(BOARD_SIZE % 2 == 1);
        assert!(BOARD_SIZE >= 3);
    }

    #[test]
    fn dot_spacing() {
        assert!(DOT_SPACING > 0);
        assert!(DOT_SPACING < HALF_BOARD_SIZE);
    }
}
