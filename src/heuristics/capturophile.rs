use crate::{model::Model, player::PlayerColor};

pub const fn capturophile(model: &Model) -> i64 {
    match model.current_color {
        PlayerColor::Black => model.black_captures as i64 - model.white_captures as i64,
        PlayerColor::White => model.white_captures as i64 - model.black_captures as i64,
    }
}
