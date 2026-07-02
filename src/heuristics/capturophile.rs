use crate::{game::Game, player::PlayerColor};

pub const fn capturophile(game: &Game, player_color: PlayerColor) -> i64 {
    match player_color {
        PlayerColor::Black => game.black_captures as i64,
        PlayerColor::White => game.white_captures as i64,
    }
}
