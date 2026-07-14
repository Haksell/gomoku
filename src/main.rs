mod arena;
mod bots;
mod game;
mod genetrain;
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
    num_games: usize, // TODO: Option like num_threads
    #[arg(short('t'), long)]
    num_threads: Option<usize>,
    genetrain: Option<bool>,
}

fn main() {
    let args = Args::parse();
    if args.genetrain.is_some_and(|genetrain| genetrain) {
        genetrain::run();
        return;
    }
    match args.num_games {
        0 => panic!("Can't play 0 games."),
        1 => gui::run(),
        n if n.is_multiple_of(4) => arena::run(&args),
        _ => panic!("TODO: good error message pls"), // TODO: warn instead, but run arena anyway
    }
}
