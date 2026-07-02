mod coordinates;
mod events;
mod textures;
mod view;

use crate::{
    Args,
    constants::WINDOW_SIZE,
    game::{Game, Position},
    player::Player,
};
use clap::Parser as _;
use coordinates::mouse_to_board;
use events::{key_pressed, mouse_pressed};
use nannou::{App, event::Update, winit::window::CursorIcon};
use std::time::Instant;
use textures::init_textures;
use view::view;

pub fn run() {
    nannou::app(app).update(update).view(view).run();
}

#[derive(Clone)]
struct Model {
    game: Game,
    hover: Option<Position>,
    ai_thinking_time: Option<u128>,
}

impl Model {
    fn new(black_player: Player, white_player: Player) -> Self {
        Self { game: Game::new(black_player, white_player), hover: None, ai_thinking_time: None }
    }
}

fn app(app: &App) -> Model {
    // TODO: parse args only once
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

fn update(app: &App, model: &mut Model, _: Update) {
    model.hover = None;

    if model.game.winner.is_none()
        && let Player::Bot { bot, heuristic } = model.game.current_player()
    {
        let start = Instant::now();
        // let bot_thread = std::thread::spawn(|| bot(model., *heuristic));
        let (x, y) = bot(&model.game, *heuristic);
        model.ai_thinking_time = Some(start.elapsed().as_millis());
        println!("AI move computed in {:?} ms", model.ai_thinking_time.unwrap()); // TODO: show in UI and delete this println (MANDATORY!)
        model.game.do_move(x, y);
    }

    if model.game.winner.is_none() && model.game.current_player().is_human() {
        model.hover = mouse_to_board(app, model);
    }

    app.main_window().set_cursor_icon(if model.hover.is_some() {
        CursorIcon::Hand
    } else {
        CursorIcon::Default
    });
}
