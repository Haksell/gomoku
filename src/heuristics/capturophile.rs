use crate::{player::PlayerColor, state::State};

pub const fn capturophile(state: &State) -> i64 {
    match state.current_color {
        PlayerColor::Black => state.black_captures as i64 - state.white_captures as i64,
        PlayerColor::White => state.white_captures as i64 - state.black_captures as i64,
    }
}
