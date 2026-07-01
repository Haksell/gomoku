mod bots;
mod constants;
mod coordinates;
mod heuristics;
mod model;
mod rules;
mod textures;
mod turn;
mod view;

use crate::{
    bots::BotArg, constants::WINDOW_SIZE, coordinates::mouse_to_board, heuristics::HeuristicArg,
    textures::init_textures, turn::Turn, view::view,
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
    #[arg(short, long, value_enum, default_value_t = BotArg::AlphaBetaPruning)]
    bot: BotArg,
    #[arg(short, long, value_enum, default_value_t = HeuristicArg::Capturophile)]
    heuristic: HeuristicArg,
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
    Model::new(args.bot.func(), args.heuristic.func())
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left
        && model.winner == Turn::None
        && model.human == model.current_player
        && let Some((x, y)) = mouse_to_board(app, model)
    {
        model.hover = None;
        model.do_move(x, y);
        if model.winner == Turn::None {
            model.ai_pending_frames = 2;
        }
    }
}

fn key_pressed(_: &App, model: &mut Model, key: Key) {
    if key == Key::Back && !model.moves.is_empty() {
        // TODO: use model.undo_move
        *model =
            Model::from_moves(model.bot, model.heuristic, &model.moves[0..model.moves.len() - 1]);
    }
    if key == Key::Home {
        *model = Model::new(model.bot, model.heuristic);
    }
}

fn update(app: &App, model: &mut Model, _: Update) {
    model.hover = None;

    let should_allow_hover = if model.winner != Turn::None {
        false
    } else if model.ai_pending_frames > 0 {
        model.ai_pending_frames -= 1;

        if model.ai_pending_frames == 0 && model.current_player != model.human {
            let start = Instant::now();
            let (x, y) = (model.bot)(model, model.heuristic);
            model.ai_thinking_time = Some(start.elapsed().as_millis());
            println!("AI move computed in {:?} ms", model.ai_thinking_time.unwrap()); // TODO: show in UI and delete this println (MANDATORY!)
            model.do_move(x, y);
        }
        false
    } else {
        model.current_player == model.human
    };

    if should_allow_hover {
        model.hover = mouse_to_board(app, model);
    }

    app.main_window().set_cursor_icon(if model.hover.is_some() {
        CursorIcon::Hand
    } else {
        CursorIcon::Default
    });
}
