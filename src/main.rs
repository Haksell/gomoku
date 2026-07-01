mod bots;
mod constants;
mod coordinates;
mod heuristics;
mod player;
mod rules;
mod state;
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
use nannou::{
    App,
    event::{Key, MouseButton, Update},
    winit::window::CursorIcon,
};
use state::State;
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
                let mut state = State::new(args.black_player, args.white_player);
                state.play_game();
                match state.winner {
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

fn app(app: &App) -> State {
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
    State::new(args.black_player, args.white_player)
}

fn mouse_pressed(app: &App, state: &mut State, button: MouseButton) {
    if button == MouseButton::Left
        && state.winner.is_none()
        && state.current_player().is_human()
        && let Some((x, y)) = mouse_to_board(app, state)
    {
        state.hover = None;
        state.do_move(x, y);
    }
}

fn key_pressed(_: &App, state: &mut State, key: Key) {
    // FIXME
    // if key == Key::Back && !state.moves.is_empty() {
    //     // TODO: use state.undo_move
    //     *state = State::from_moves(
    //         state.black_player,
    //         state.white_player,
    //         &state.moves[0..state.moves.len() - 1],
    //     );
    // }
    if key == Key::Home {
        *state = State::new(state.black_player, state.white_player);
    }
}

fn update(app: &App, state: &mut State, _: Update) {
    state.hover = None;

    if state.winner.is_none()
        && let Player::Bot { bot, heuristic } = state.current_player()
    {
        let start = Instant::now();
        let (x, y) = bot(state, *heuristic);
        state.ai_thinking_time = Some(start.elapsed().as_millis());
        println!("AI move computed in {:?} ms", state.ai_thinking_time.unwrap()); // TODO: show in UI and delete this println (MANDATORY!)
        state.do_move(x, y);
    }

    if state.winner.is_none() && state.current_player().is_human() {
        state.hover = mouse_to_board(app, state);
    }

    app.main_window().set_cursor_icon(if state.hover.is_some() {
        CursorIcon::Hand
    } else {
        CursorIcon::Default
    });
}
