use crate::gui::{Model, coordinates::mouse_to_board};
use nannou::{
    App,
    event::{Key, MouseButton},
};

pub fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left
        && model.game.winner.is_none()
        && model.game.current_player().is_human()
        && let Some((x, y)) = mouse_to_board(app, model)
    {
        model.hover = None;
        model.game.do_move(x, y);
    }
}

pub fn key_pressed(_: &App, model: &mut Model, key: Key) {
    // FIXME
    // if key == Key::Back && !model.moves.is_empty() {
    //     // TODO: use model.undo_move
    //     *model = State::from_moves(
    //         model.black_player,
    //         model.white_player,
    //         &model.moves[0..model.moves.len() - 1],
    //     );
    // }
    if key == Key::Home {
        *model = Model::new(model.game.black_player, model.game.white_player);
    }
}
