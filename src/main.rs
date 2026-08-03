use clap::Parser as _;
use gomoku::{GomokuArgs, arena, gui, init_thread_pool, init_time_limit};

fn main() {
    let args = GomokuArgs::parse();

    init_time_limit(args.common.time_limit_ms);
    init_thread_pool(args.common.num_threads);

    // Must be done after init_time_limit because needs coeffs
    let black_player = args.black_player.as_str().into();
    let white_player = args.white_player.as_str().into();

    match args.num_games.get() {
        // type handled
        // 0 => panic!("Can't play 0 games."),
        1 => gui::run(),
        n => {
            if !n.is_multiple_of(2) {
                eprintln!("Only {} games will be played", n - 1);
            }
            arena::run(&black_player, &white_player, args.num_games);
        }
    }
}
