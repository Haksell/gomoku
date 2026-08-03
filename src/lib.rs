pub mod arena;
pub mod bots;
pub mod game;
#[cfg(feature = "gui")]
pub mod gui;
#[cfg(not(feature = "gui"))]
pub mod gui {
    #[inline]
    #[expect(clippy::missing_panics_doc)] // already self explanatory
    pub fn run() {
        panic!("GUI feature not enabled. Rebuild with `--features gui` to enable the GUI.");
    }
}
pub mod heuristics;
pub mod player;

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

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short('t'), long, default_value_t = NonZeroUsize::new(1).unwrap())]
    pub num_threads: NonZeroUsize,
    #[arg(short('l'), long, default_value_t = NonZeroU64::new(500).unwrap())]
    pub time_limit_ms: NonZeroU64,
}

#[derive(Debug, Parser)]
pub struct GomokuArgs {
    pub black_player: String,
    pub white_player: String,
    #[arg(short('g'), long, default_value_t = NonZeroUsize::new(1).unwrap())]
    pub num_games: NonZeroUsize,
    #[command(flatten)]
    pub common: Args,
}

static TIME_LIMIT: OnceLock<Duration> = OnceLock::new();

/// # Panics
///
/// Will panic if `TIME_LIMIT`, `INITIAL_COEFFS`, `OLD_COEFFS` or `COEFFS_FILE` is already set.
#[inline]
pub fn init_time_limit(time_limit_ms: NonZeroU64) {
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

/// # Panics
///
/// Will panic if it can't retrieve available cpus or `num_threads` > available cpus.
#[inline]
pub fn init_thread_pool(num_threads: NonZeroUsize) {
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
