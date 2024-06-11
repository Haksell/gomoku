use crate::{model::Model, turn::Turn};

pub fn capturophile(model: &Model) -> i64 {
    match model.human {
        Turn::Black => model.white_captures as i64 - model.black_captures as i64,
        Turn::White => model.black_captures as i64 - model.white_captures as i64,
        Turn::None => std::unreachable!(),
    }
}
