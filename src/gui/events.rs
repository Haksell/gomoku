use crate::gui::{Model, coordinates::mouse_to_board};
use nannou::{
    App,
    event::{Key, MouseButton},
};

pub fn mouse_released(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left
        && model.game.state.is_playing()
        && model.game.current_player().is_human()
        && let Some(pos) = mouse_to_board(app, model)
    {
        model.hover = None;
        model.game.do_move(pos);
    }
}

pub fn key_released(_: &App, model: &mut Model, key: Key) {
    // TODO: left, right with history, not undo
    if key == Key::Back && model.game.ply > 0 {
        model.game.undo_last_move();
    }

    // TODO: remove (flag or button in gui)
    if Key::Key1 <= key && key <= Key::Key9 {
        *model = Model::new(model.game.black_player, model.game.white_player);
        let n_moves = key as u32 - Key::Key1 as u32 + 1; // ghetto
        println!("{n_moves}");
        model.game.play_random_moves(n_moves, 5);
    }

    if key == Key::Home {
        *model = Model::new(model.game.black_player, model.game.white_player);
    }
}
