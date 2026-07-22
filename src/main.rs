mod arena;
mod bots;
mod game;
mod genetrain; // TODO: remove?
mod gradientrain;
mod gui;
mod heuristics;
mod player;

use crate::player::Player;
use clap::Parser;
use rayon::ThreadPoolBuilder;
use std::{thread::available_parallelism, time::Duration};

// TODO: flag with default value of 500ms
const TIME_LIMIT: Duration = Duration::from_millis(8);

#[derive(Debug, Parser)]
struct Args {
    black_player: Player,
    white_player: Player,
    #[arg(short('g'), long, default_value_t = 1)] // TODO: clap validate > 0
    num_games: usize,
    #[arg(short('t'), long, default_value_t = 1)] // TODO: clap validate > 0
    num_threads: usize,
    #[arg(long)]
    genetrain: bool,
    #[arg(long)]
    gradientrain: bool,
}

fn main() {
    let args = Args::parse();
    init_thread_pool(args.num_threads);

    // TODO: --train flag or put them in a bin
    if args.gradientrain {
        gradientrain::run();
        return;
    }
    if args.genetrain {
        genetrain::run();
        return;
    }

    match args.num_games {
        0 => panic!("Can't play 0 games."),
        1 => gui::run(),
        n if n.is_multiple_of(4) => {
            arena::run(&args.black_player, &args.white_player, args.num_games);
        }
        _ => panic!("TODO: good error message pls"), // TODO: warn instead, but run arena anyway
    }
}

fn init_thread_pool(num_threads: usize) {
    let available_cpus = available_parallelism().unwrap().get();
    assert!(num_threads > 0, "Can't run with 0 threads.");
    assert!(
        num_threads <= available_cpus,
        "You asked for {num_threads} threads but only {available_cpus} threads are available.",
    );
    ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
}
