mod constants;
mod coordinates;
mod model;
mod player;
mod rules;
mod textures;
mod view;

use constants::WINDOW_SIZE;
use coordinates::mouse_to_board;
use model::Model;
use nannou::prelude::*;
use player::Player;
use textures::init_textures;
use view::view;

fn main() {
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
    if button == MouseButton::Left && model.winner == Player::None {
        if let Some((x, y)) = mouse_to_board(app, model) {
            model.do_move(x, y);
        }
    }
}

fn key_pressed(_: &App, model: &mut Model, key: Key) {
    if key == Key::Back && !model.moves.is_empty() {
        *model = Model::from_moves(&model.moves[0..model.moves.len() - 1]);
    }
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    let mouse_pos = app.mouse.position();
    println!("{:?}", mouse_pos);
}
