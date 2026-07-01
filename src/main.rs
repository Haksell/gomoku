mod bots;
mod constants;
mod coordinates;
mod heuristics;
mod model;
mod player;
mod rules;
mod textures;
mod view;

use crate::{
    constants::WINDOW_SIZE,
    coordinates::mouse_to_board,
    player::{Player, PlayerColor},
    textures::init_textures,
    view::view,
};
use clap::Parser;
use model::Model;
use nannou::{
    App,
    event::{Key, MouseButton, Update},
    winit::window::CursorIcon,
};
use std::time::Instant;

#[derive(Debug, Parser)]
struct Args {
    black_player: Player,
    white_player: Player,
}

fn main() {
    nannou::app(app).update(update).view(view).run();
}

fn app(app: &App) -> Model {
    let args = Args::parse();
    app.new_window()
        .title("ligomoku.org")
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .resizable(false)
        .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    init_textures(app);
    Model::new(args.black_player, args.white_player)
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left
        && model.winner.is_none()
        && model.current_player().is_human()
        && let Some((x, y)) = mouse_to_board(app, model)
    {
        model.hover = None;
        model.do_move(x, y);
    }
}

fn key_pressed(_: &App, model: &mut Model, key: Key) {
    if key == Key::Back && !model.moves.is_empty() {
        // TODO: use model.undo_move
        *model = Model::from_moves(
            model.black_player,
            model.white_player,
            &model.moves[0..model.moves.len() - 1],
        );
    }
    if key == Key::Home {
        *model = Model::new(model.black_player, model.white_player);
    }
}

fn update(app: &App, model: &mut Model, _: Update) {
    model.hover = None;

    if model.winner.is_none()
        && let Player::Bot { bot, heuristic } = model.current_player()
    {
        let start = Instant::now();
        let (x, y) = bot(model, *heuristic);
        model.ai_thinking_time = Some(start.elapsed().as_millis());
        println!("AI move computed in {:?} ms", model.ai_thinking_time.unwrap()); // TODO: show in UI and delete this println (MANDATORY!)
        model.do_move(x, y);
    }

    if model.winner.is_none() && model.current_player().is_human() {
        model.hover = mouse_to_board(app, model);
    }

    app.main_window().set_cursor_icon(if model.hover.is_some() {
        CursorIcon::Hand
    } else {
        CursorIcon::Default
    });
}
