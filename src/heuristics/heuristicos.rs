use crate::game::Game;

pub const fn heuristicos(game: &Game) -> i64 {
    let mut score = 0;

    score += game.black_captures as i64 - game.white_captures as i64;

    score
}
