mod bots;
mod constants;
mod coordinates;
mod heuristics;
mod model;
mod player;
mod rules;
mod textures;
mod view;

use bots::alpha_beta_pruning;
use constants::WINDOW_SIZE;
use coordinates::mouse_to_board;
use heuristics::capturophile;
use model::Model;
use nannou::{prelude::*, winit::window::CursorIcon};
use player::Player;
use textures::init_textures;
use view::view;

fn main() {
    println!("{:?}", std::env::args());
    nannou::app(app).update(update).view(view).run();
}

fn app(app: &App) -> Model {
    app.new_window()
        .title("ligomoku.org")
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .resizable(false)
        .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    init_textures(app);
    Model::start()
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left
        && model.winner == Player::None
        && model.human == model.current_player
    {
        if let Some((x, y)) = mouse_to_board(app, model) {
            model.hover = None;
            model.do_move(x, y);
            if model.winner == Player::None {
                let (x, y) = alpha_beta_pruning(model, capturophile);
                model.do_move(x, y);
            }
            if model.winner != Player::None {
                println!("{:?} won.", model.winner);
            }
        }
    }
}

fn key_pressed(_: &App, model: &mut Model, key: Key) {
    if key == Key::Back && !model.moves.is_empty() {
        // TODO: use model.undo_move
        *model = Model::from_moves(&model.moves[0..model.moves.len() - 1]);
    }
    if key == Key::Home {
        *model = Model::start();
    }
}

fn update(app: &App, model: &mut Model, _: Update) {
    if model.winner != Player::None || model.current_player != model.human {
        return;
    }
    // TODO: fix bug where hover remains on edge of board when mouse leaves fast
    model.hover = mouse_to_board(app, model);
    app.main_window().set_cursor_icon(if model.hover.is_some() {
        CursorIcon::Hand
    } else {
        CursorIcon::Default
    });
}
