mod arena;
mod bots;
mod game;
mod gui;
mod heuristics;
mod player;

use crate::player::Player;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    black_player: Player,
    white_player: Player,
    #[arg(short('g'), long, default_value_t = 1)] // TODO: clap validate > 0
    num_games: usize,
    #[arg(short('t'), long)]
    num_threads: Option<usize>,
}

fn main() {
    let args = Args::parse();
    match args.num_games {
        0 => panic!("Can't play 0 games."),
        1 => gui::run(),
        n if n.is_multiple_of(2) => arena::run(&args),
        _ => {
            panic!(
                "The arena requires an even amount of games to avoid the first mover advantage."
            );
        }
    }
}
