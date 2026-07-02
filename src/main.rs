mod arena;
mod bots;
mod constants;
mod game;
mod gui;
mod heuristics;
mod player;
mod rules;

use crate::player::{Player, PlayerColor};
use clap::Parser;

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
        1 => gui::run(),
        _ => arena::run(&args),
    }
}
