use crate::{model::Model, player::Player};

pub fn capturophile(model: &Model) -> i64 {
    match model.human {
        Player::Black => model.white_captures as i64 - model.black_captures as i64,
        Player::White => model.black_captures as i64 - model.white_captures as i64,
        Player::None => std::unreachable!(),
    }
}
