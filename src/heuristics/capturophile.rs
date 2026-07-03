use crate::game::Game;

pub const fn capturophile(game: &Game) -> i64 {
    game.black_captures as i64 - game.white_captures as i64
}
