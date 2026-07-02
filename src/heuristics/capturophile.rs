use crate::{player::PlayerColor, model::Game};

pub const fn capturophile(game: &Game) -> i64 {
    match game.current_color {
        PlayerColor::Black => game.black_captures as i64 - game.white_captures as i64,
        PlayerColor::White => game.white_captures as i64 - game.black_captures as i64,
    }
}
