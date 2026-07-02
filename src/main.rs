mod bots;
mod constants;
mod coordinates;
mod game;
mod heuristics;
mod model;
mod player;
mod rules;
mod textures;
mod view;

use crate::{
    constants::WINDOW_SIZE,
    coordinates::mouse_to_board,
    game::Game,
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
    #[arg(short, long, default_value_t = 1)] // TODO: clap validate > 0
    num_games: usize,
    // TODO: arg for number of threads in algs and simulation
}

fn main() {
    let args = Args::parse();
    match args.num_games {
        0 => panic!("Can't play 0 games."),
        1 => nannou::app(app).update(update).view(view).run(),
        n => {
            assert!(
                args.black_player.is_bot() || args.white_player.is_bot(),
                "`num_games` is reserved for bot vs bot matches."
            );
            let mut black_wins = 0;
            let mut white_wins = 0;
            for game in 1..=n {
                let mut model = Game::new(args.black_player, args.white_player);
                model.play_game();
                match model.winner {
                    Some(PlayerColor::Black) => black_wins += 1,
                    Some(PlayerColor::White) => white_wins += 1,
                    None => unimplemented!("draws are broken"),
                }
                let black_percentage = 100. * black_wins as f64 / game as f64;
                let white_percentage = 100. - black_percentage;
                println!("Black won {black_wins} games ({black_percentage:.1}%)");
                println!("White won {white_wins} games ({white_percentage:.1}%)");
                println!();
            }
        }
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

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left
        && model.game.winner.is_none()
        && model.game.current_player().is_human()
        && let Some((x, y)) = mouse_to_board(app, model)
    {
        model.hover = None;
        model.game.do_move(x, y);
    }
}

fn key_pressed(_: &App, model: &mut Model, key: Key) {
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
