use nannou::color::Srgb;

pub const SQUARES: usize = 19;
pub const HALF_SQUARES: usize = SQUARES >> 1;
pub const WINDOW_SIZE: usize = 800;
pub const BOARD_MARGIN: f32 = 60.0;
pub const BOARD_SIZE: f32 = WINDOW_SIZE as f32 - 2.0 * BOARD_MARGIN;
pub const CELL_SIZE: f32 = BOARD_SIZE / SQUARES as f32;

#[test]
fn board_is_odd() {
    assert!(SQUARES & 1 == 1);
}

pub const COLOR_BACKGROUND: Srgb<u8> = Srgb {
    red: 237,
    green: 208,
    blue: 128,
    standard: ::core::marker::PhantomData,
};
