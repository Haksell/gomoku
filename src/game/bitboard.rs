use crate::{
    game::board::{BOARD_SIZE, Position},
    player::PlayerColor,
};

// empty: 00
// black: 10
// white: 11

const BITBOARD_SIZE: usize = (BOARD_SIZE * BOARD_SIZE * 2 + 6).div_ceil(64);
pub type BitBoard = [u64; BITBOARD_SIZE];

pub const fn bitboard_set(bitboard: &mut BitBoard, (x, y): Position, color: Option<PlayerColor>) {
    let i = (y * BOARD_SIZE + x) << 1;
    let word = i >> 6;
    let shift = i & 63;
    let mask = !(3 << shift);
    let color_value = match color {
        None => 0b00,
        Some(PlayerColor::Black) => 0b10,
        Some(PlayerColor::White) => 0b11,
    };
    bitboard[word] = (bitboard[word] & mask) | (color_value << shift);
}

pub const fn bitboard_set_captures(
    bitboard: &mut BitBoard,
    black_captures: usize,
    white_captures: usize,
) {
    // TODO: handle case where it overflows between two words
    const BLACK_CAPTURES_IDX: usize = (BOARD_SIZE * BOARD_SIZE * 2).rem_euclid(64);
    const WHITE_CAPTURES_IDX: usize = BLACK_CAPTURES_IDX + 3;
    const LAST_WORD_CELLS_MASK: u64 = (1 << BLACK_CAPTURES_IDX) - 1;
    const LAST_WORD_IDX: usize = BITBOARD_SIZE - 1;

    bitboard[LAST_WORD_IDX] = (bitboard[LAST_WORD_IDX] & LAST_WORD_CELLS_MASK)
        | (black_captures << BLACK_CAPTURES_IDX) as u64
        | (white_captures << WHITE_CAPTURES_IDX) as u64;
}
