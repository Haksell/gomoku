use crate::{
    game::{
        board::{BOARD_SIZE, Position},
        state::REQUIRED_CAPTURES,
    },
    player::PlayerColor,
};

const RANDOM_VALUES: usize = (BOARD_SIZE * BOARD_SIZE + REQUIRED_CAPTURES + 8) * 2;

const ZOBRIST: [u64; RANDOM_VALUES] = {
    let mut zobrist = [0; RANDOM_VALUES];
    let mut x: u64 = 1_003_205_231_972_679_264;
    let mut i = 0;
    while i < RANDOM_VALUES {
        zobrist[i] = x;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        i += 1;
    }
    zobrist[2 * BOARD_SIZE * BOARD_SIZE] = 0;
    zobrist[2 * BOARD_SIZE * BOARD_SIZE + 1] = 0;
    zobrist
};

pub const fn zobrist_move(zobrist: u64, color: PlayerColor, (x, y): Position) -> u64 {
    let i = 2 * (y * BOARD_SIZE + x) + matches!(color, PlayerColor::White) as usize;
    zobrist ^ ZOBRIST[i]
}

pub const fn zobrist_capture(zobrist: u64, color: PlayerColor, old_captures: usize) -> u64 {
    let i =
        2 * (BOARD_SIZE * BOARD_SIZE + old_captures) + matches!(color, PlayerColor::White) as usize;
    zobrist ^ ZOBRIST[i] ^ ZOBRIST[i + 2]
}
