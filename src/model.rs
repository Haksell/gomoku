use crate::{
    Player,
    game::{Game, Position},
};

#[derive(Clone)]
pub struct Model {
    pub game: Game,
    pub hover: Option<Position>,
    pub ai_thinking_time: Option<u128>,
}

impl Model {
    pub fn new(black_player: Player, white_player: Player) -> Self {
        Self { game: Game::new(black_player, white_player), hover: None, ai_thinking_time: None }
    }
}
