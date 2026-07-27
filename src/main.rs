mod arena;
mod bots;
mod game;
mod genetrain; // TODO: remove?
mod gradientrain;
mod gui;
mod heuristics;
mod player;

use crate::{
    heuristics::coeffistic::{COEFFS_FILE, INITIAL_COEFFS, OLD_COEFFS},
    player::Player,
};
use clap::Parser;
use rayon::ThreadPoolBuilder;
use std::{
    num::{NonZeroU64, NonZeroUsize},
    sync::OnceLock,
    thread::available_parallelism,
    time::Duration,
};

static TIME_LIMIT: OnceLock<Duration> = OnceLock::new();

#[derive(Debug, Parser)]
struct Args {
    black_player: Player,
    white_player: Player,
    #[arg(short('g'), long, default_value_t = NonZeroUsize::new(1).unwrap())]
    num_games: NonZeroUsize,
    #[arg(short('t'), long, default_value_t = NonZeroUsize::new(1).unwrap())]
    num_threads: NonZeroUsize,
    #[arg(short('l'), long, default_value_t = NonZeroU64::new(500).unwrap())]
    time_limit_ms: NonZeroU64,
    #[arg(long)]
    genetrain: bool,
    #[arg(long)]
    gradientrain: bool,
}

fn main() {
    let args = Args::parse();

    init_time_limit(args.time_limit_ms);
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

    match args.num_games.get() {
        // type handled
        // 0 => panic!("Can't play 0 games."),
        1 => gui::run(),
        n if n.is_multiple_of(4) => {
            arena::run(&args.black_player, &args.white_player, args.num_games);
        }
        _ => panic!("TODO: good error message pls"), // TODO: warn instead, but run arena anyway
    }
}

fn init_time_limit(time_limit_ms: NonZeroU64) {
    let time_limit_ms = time_limit_ms.get();
    TIME_LIMIT.set(Duration::from_millis(time_limit_ms)).unwrap();
    match time_limit_ms {
        0..=4 => {
            INITIAL_COEFFS.set(include!("../coeffs/coeffs_002ms_new.rs")).unwrap();
            OLD_COEFFS.set(include!("../coeffs/coeffs_002ms_old.rs")).unwrap();
            COEFFS_FILE.set("./coeffs/coeffs_002ms_new.rs").unwrap();
        }
        5..=16 => {
            INITIAL_COEFFS.set(include!("../coeffs/coeffs_008ms_new.rs")).unwrap();
            OLD_COEFFS.set(include!("../coeffs/coeffs_008ms_old.rs")).unwrap();
            COEFFS_FILE.set("./coeffs/coeffs_008ms_new.rs").unwrap();
        }
        _ => {
            INITIAL_COEFFS.set(include!("../coeffs/coeffs_032ms_new.rs")).unwrap();
            OLD_COEFFS.set(include!("../coeffs/coeffs_032ms_old.rs")).unwrap();
            COEFFS_FILE.set("./coeffs/coeffs_032ms_new.rs").unwrap();
        }
    }
}

fn init_thread_pool(num_threads: NonZeroUsize) {
    let num_threads = num_threads.get();
    let available_cpus = available_parallelism().unwrap().get();
    // type handled
    // assert!(num_threads > 0, "Can't run with 0 threads.");
    assert!(
        num_threads <= available_cpus,
        "You asked for {num_threads} threads but only {available_cpus} threads are available.",
    );
    ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
}
