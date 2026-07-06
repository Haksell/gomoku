use crate::{
    Args,
    game::{Game, state::GameState},
    player::PlayerColor,
};
use rayon::{ThreadPoolBuilder, prelude::*};
use std::{sync::Mutex, thread::available_parallelism};

// TODO: random initial position
pub fn run(args: &Args) {
    assert!(
        args.black_player.is_bot() || args.white_player.is_bot(),
        "`num_games` is reserved for bot vs bot matches."
    );

    let num_threads = args.num_threads.unwrap_or(1); // TODO: if 1, no par_iter
    let available_cpus = available_parallelism().unwrap().get();

    assert!(num_threads > 0, "Can't run with 0 threads.");
    assert!(
        num_threads <= available_cpus,
        "You asked for {num_threads} threads but only {available_cpus} threads are available.",
    );

    let wins_and_games = Mutex::new((0., 0));

    // TODO: if 1 thread, no parallelism
    // TODO: no global (if we need to do stuff after arena)
    // TODO: understand why 10 threads is faster than 20
    ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
    (1..=args.num_games).into_par_iter().for_each(|game_idx| {
        let (player1_color, black, white) = match game_idx & 1 {
            0 => (PlayerColor::White, args.white_player, args.black_player),
            1 => (PlayerColor::Black, args.black_player, args.white_player),
            _ => unreachable!(),
        };

        let mut game = Game::new(black, white);
        // TODO: flags to configure random board (n_moves and dist_to_center)
        game.play_random_moves(4, 5);
        game.play_game();

        let mut wins_and_games = wins_and_games.lock().unwrap();

        match game.state {
            GameState::Playing(_) => unreachable!(),
            GameState::Draw => wins_and_games.0 += 0.5,
            GameState::Won(winner, _) if winner == player1_color => {
                wins_and_games.0 += 1.;
            }
            GameState::Won(_, _) => {}
        }

        wins_and_games.1 += 1;

        let (player1_wins, finished_games) = *wins_and_games;
        drop(wins_and_games);

        let player1_percentage = 100. * player1_wins / finished_games as f64;
        println!("Player 1 won {player1_wins}/{finished_games} games ({player1_percentage:.1}%)");
    });
}
