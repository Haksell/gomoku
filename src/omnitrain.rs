// TODO: bring back true omnitrain and call this one incdec_train or whatever

use crate::{
    bots::idabp::idabp,
    game::{Game, state::GameState},
    heuristics::{
        Heuristic,
        coeffistic::{
            COEFFS_FILE, INITIAL_COEFFS, N_STENCIL_COEFFS, STENCIL_INDICES, STENCIL_INDICES_OPP,
            STENCIL_INDICES_SYM, STENCIL_INDICES_SYM_OPP, UNIQUE_STENCIL_INDICES, coeffistic,
            write_coeffs,
        },
    },
    player::{Player, PlayerColor},
};
use nannou::rand::{Rng as _, thread_rng};
use rayon::iter::ParallelIterator as _;
use std::{
    array,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
    },
};

const N_MUTATIONS: usize = UNIQUE_STENCIL_INDICES + 9;

pub fn run() {
    let epoch = AtomicU32::default();
    let best_coeffs = Arc::new(Mutex::new(INITIAL_COEFFS.clone()));

    // TODO: find a cleaner rayon infinite loop
    rayon::iter::repeat(()).for_each(|()| {
        let max_abs_coeff = best_coeffs.lock().unwrap().iter().map(|x| x.abs()).max().unwrap();
        let learning_rate = ((max_abs_coeff as f64).sqrt().ceil() as i64).max(1);

        let update_sums = (0..learning_rate)
            .map(|_| {
                let mut rng = thread_rng();
                let updates1: [i64; N_MUTATIONS] =
                    array::from_fn(|_| rng.gen_range(-learning_rate..=learning_rate));

                let mut coeffs1 = best_coeffs.lock().unwrap().clone();
                let mut coeffs2 = coeffs1.clone();
                for (i, &update1) in updates1.iter().enumerate() {
                    update_coeffs(&mut coeffs1, i, update1);
                    update_coeffs(&mut coeffs2, i, -update1);
                }

                let player1 = Player::Bot {
                    bot: idabp,
                    heuristic: Heuristic { fun: coeffistic, coeffs: Some(coeffs1) },
                };
                let player2 = Player::Bot {
                    bot: idabp,
                    heuristic: Heuristic { fun: coeffistic, coeffs: Some(coeffs2) },
                };

                match play_pair(&player1, &player2) {
                    0 => updates1,
                    1 => [0; N_MUTATIONS],
                    2 => updates1.map(|u| -u),
                    _ => unreachable!(),
                }
            })
            .fold([0; N_MUTATIONS], |acc, res| array::from_fn(|i| acc[i] + res[i]));

        {
            let mut best_coeffs = best_coeffs.lock().unwrap();
            for (i, &update_sum) in update_sums.iter().enumerate() {
                update_coeffs(&mut best_coeffs, i, update_sum.clamp(-1, 1));
            }
        }

        let epoch = epoch.fetch_add(1, Ordering::Relaxed) + 1;
        println!("Epoch {epoch} done with learning_rate={learning_rate}.");

        if epoch.is_multiple_of(5) {
            let best_coeffs = best_coeffs.lock().unwrap().clone();
            match write_coeffs(&best_coeffs) {
                Ok(()) => eprintln!("Best coeffs written to file {COEFFS_FILE}"),
                Err(err) => eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`"),
            }
        }

        if epoch.is_multiple_of(20) {
            let best_coeffs = best_coeffs.lock().unwrap().clone();
            stats(best_coeffs, 5);
        }
    });
}

fn update_coeffs(coeffs: &mut [i64], i: usize, update: i64) {
    if i >= UNIQUE_STENCIL_INDICES {
        coeffs[i - UNIQUE_STENCIL_INDICES + N_STENCIL_COEFFS] += update;
    } else {
        coeffs[STENCIL_INDICES[i]] += update;
        coeffs[STENCIL_INDICES_OPP[i]] -= update;
        if STENCIL_INDICES[i] != STENCIL_INDICES_SYM[i] {
            coeffs[STENCIL_INDICES_SYM[i]] += update;
            coeffs[STENCIL_INDICES_SYM_OPP[i]] -= update;
        }
    }
}

fn play_pair(old_player: &Player, new_player: &Player) -> u32 {
    let mut old_new = Game::new(old_player, new_player);
    let random_moves = thread_rng().gen_range(3..=4);
    old_new.play_random_moves(random_moves, 5);

    let mut new_old = old_new.clone();
    (new_old.black_player, new_old.white_player) = (new_old.white_player, new_old.black_player);

    old_new.play_game();
    new_old.play_game();

    matches!(old_new.state, GameState::Won(PlayerColor::White, _)) as u32
        + matches!(new_old.state, GameState::Won(PlayerColor::Black, _)) as u32
}

fn stats(best_coeffs: Box<[i64]>, pair_of_games: u32) {
    let new_player = Player::Bot {
        bot: idabp,
        heuristic: Heuristic { fun: coeffistic, coeffs: Some(best_coeffs) },
    };
    let initial_player = Player::Bot {
        bot: idabp,
        heuristic: Heuristic { fun: coeffistic, coeffs: Some(INITIAL_COEFFS.clone()) },
    };

    let wins_against_initial: u32 =
        (0..pair_of_games).map(|_| play_pair(&initial_player, &new_player)).sum();
    let wins_against_manual: u32 =
        (0..pair_of_games).map(|_| play_pair(&Player::MANUAL, &new_player)).sum();

    let total_games = 2 * pair_of_games;
    let dividing_line = "=".repeat(80);
    println!("{dividing_line}");
    println!("Current won {wins_against_initial}/{total_games} games against initial bot");
    println!("Current won {wins_against_manual}/{total_games} games against manual bot");
    println!("{dividing_line}");
}
