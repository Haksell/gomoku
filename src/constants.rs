use nannou::color::Srgb;

pub const BOARD_SIZE: usize = 19;
pub const CELLS: usize = BOARD_SIZE - 1;
pub const HALF_BOARD_SIZE: usize = BOARD_SIZE >> 1;
pub const WINDOW_SIZE: usize = 800;
pub const WINDOW_MARGIN: usize = 30;
pub const CELL_SIZE: usize = (WINDOW_SIZE - 2 * WINDOW_MARGIN) / CELLS;

#[test]
fn board_size_is_odd() {
    assert!(BOARD_SIZE & 1 == 1);
}

pub const COLOR_BACKGROUND: Srgb<u8> = Srgb {
    red: 237,
    green: 208,
    blue: 128,
    standard: core::marker::PhantomData,
};
