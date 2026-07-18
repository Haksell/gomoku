mod arena;
mod bots;
mod dueltrain;
mod game;
mod genetrain;
mod gridtrain;
mod gui;
mod heuristics;
mod omnitrain;
mod player;

use std::thread::available_parallelism;

use crate::player::Player;
use clap::Parser;
use rayon::ThreadPoolBuilder;

#[expect(clippy::struct_excessive_bools)] // TODO: fix with Training enum
#[derive(Debug, Parser)]
struct Args {
    black_player: Player,
    white_player: Player,
    #[arg(short('g'), long, default_value_t = 1)] // TODO: clap validate > 0
    num_games: usize, // TODO: Option like num_threads
    #[arg(short('t'), long)]
    num_threads: Option<usize>,
    #[arg(long)]
    genetrain: bool,
    #[arg(long)]
    dueltrain: bool,
    #[arg(long)]
    gridtrain: bool,
    #[arg(long)]
    omnitrain: bool,
}

fn main() {
    let args = Args::parse();
    init_thread_pool(args.num_threads);
    if args.omnitrain {
        omnitrain::run();
        return;
    }
    if args.gridtrain {
        gridtrain::run();
        return;
    }
    if args.dueltrain {
        dueltrain::run();
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

fn init_thread_pool(num_threads: Option<usize>) {
    let num_threads = num_threads.unwrap_or(1);
    let available_cpus = available_parallelism().unwrap().get();
    assert!(num_threads > 0, "Can't run with 0 threads.");
    assert!(
        num_threads <= available_cpus,
        "You asked for {num_threads} threads but only {available_cpus} threads are available.",
    );
    ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
}
