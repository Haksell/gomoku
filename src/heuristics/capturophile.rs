use crate::model::Model;

pub fn capturophile(model: &Model) -> i64 {
    model.black_captures as i64 - model.white_captures as i64
}
