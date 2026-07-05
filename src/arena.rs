use crate::{
    Args,
    game::{Game, GameState},
    player::PlayerColor,
};
use rayon::{ThreadPoolBuilder, prelude::*};
use std::{sync::Mutex, thread::available_parallelism};

#[derive(Debug, Default, Clone, Copy)]
struct Stats {
    win_win: u32,
    win_loss: u32,
    loss_win: u32,
    loss_loss: u32,
}

impl Stats {
    const fn wins(self) -> u32 {
        self.black_wins() + self.white_wins()
    }

    const fn black_wins(self) -> u32 {
        self.win_win + self.win_loss
    }

    const fn white_wins(self) -> u32 {
        self.win_win + self.loss_win
    }

    const fn games(self) -> u32 {
        2 * (self.win_win + self.win_loss + self.loss_win + self.loss_loss)
    }
}

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

    let stats = Mutex::new(Stats::default());

    // TODO: if 1 thread, no parallelism
    // TODO: no global (if we need to do stuff after arena)
    // TODO: understand why 10 threads is faster than 20
    // TODO: flags to configure random board (n_moves and dist_to_center)
    // TODO: rehandle draws
    ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
    (1..=args.num_games / 2).into_par_iter().for_each(|i| {
        let mut game = Game::new(args.black_player, args.white_player);
        let random_moves = 3 + (i & 1) as u32;
        game.play_random_moves(random_moves, 5);

        let mut switched_game = game.clone();
        (switched_game.black_player, switched_game.white_player) =
            (switched_game.white_player, switched_game.black_player);

        game.play_game();
        let black_win = match game.state {
            GameState::Playing => unreachable!(),
            GameState::Won(PlayerColor::Black) => true,
            GameState::Draw | GameState::Won(PlayerColor::White) => false,
        };

        switched_game.play_game();
        let white_win = match switched_game.state {
            GameState::Playing => unreachable!(),
            GameState::Won(PlayerColor::White) => true,
            GameState::Draw | GameState::Won(PlayerColor::Black) => false,
        };

        let mut stats = stats.lock().unwrap();
        match (black_win, white_win) {
            (true, true) => stats.win_win += 1,
            (true, false) => stats.win_loss += 1,
            (false, true) => stats.loss_win += 1,
            (false, false) => stats.loss_loss += 1,
        }

        let player1_wins = stats.wins();
        let finished_games = stats.games();
        drop(stats);

        let player1_percentage = 100. * player1_wins as f64 / finished_games as f64;
        println!("Player 1 won {player1_wins}/{finished_games} games ({player1_percentage:.1}%)");
    });

    let stats = *stats.lock().unwrap();
    let black_wins = stats.black_wins();
    let white_wins = stats.white_wins();
    let games_per_color = stats.games() / 2;
    for (color, wins) in [("black", black_wins), ("white", white_wins)] {
        let percentage = 100. * wins as f64 / games_per_color as f64;
        println!("Player 1 won {wins}/{games_per_color} games as {color} ({percentage:.1}%)");
    }

    let doubles = stats.win_win + stats.loss_loss;
    if doubles == 0 {
        println!("All pairs of game were shared!");
    } else {
        let percentage_double_good = 100. * stats.win_win as f64 / doubles as f64;
        println!(
            "Player 1 had {} double wins and {} double losses ({percentage_double_good:.1}%)",
            stats.win_win, stats.loss_loss
        );
    }
}
