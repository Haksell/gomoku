use nannou::color::Srgb;

pub const BOARD_SIZE: usize = 19;
pub const HALF_BOARD_SIZE: usize = BOARD_SIZE >> 1;
pub const WINDOW_SIZE: usize = 800;
pub const WINDOW_MARGIN: f32 = 35.0;
pub const CELL_SIZE: f32 = (WINDOW_SIZE as f32 - 2.0 * WINDOW_MARGIN) / (BOARD_SIZE - 1) as f32;
pub const DOT_SPACING: usize = 6;
pub const REQUIRED_CAPTURES: usize = 5;

pub const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (1, 1), (0, 1), (1, -1)];

pub const COLOR_BACKGROUND: Srgb<u8> = Srgb {
    red: 237,
    green: 208,
    blue: 128,
    standard: core::marker::PhantomData,
};

#[test]
fn test_board_size() {
    assert!(BOARD_SIZE & 1 == 1);
    assert!(BOARD_SIZE >= 3);
}

#[test]
fn test_dot_spacing() {
    assert!(DOT_SPACING > 0);
    assert!(DOT_SPACING < HALF_BOARD_SIZE);
}
