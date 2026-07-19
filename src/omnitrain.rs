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
const PAIRS_BY_EPOCH: u32 = 8;
const MAX_MUTATION: i64 = 8;

pub fn run() {
    let epoch = AtomicU32::default();
    let best_coeffs = Arc::new(Mutex::new(INITIAL_COEFFS.clone()));

    // TODO: find a cleaner rayon infinite loop
    rayon::iter::repeat(()).for_each(|()| {
        let update_sums = (0..PAIRS_BY_EPOCH)
            .map(|_| {
                let mut coeffs1 = best_coeffs.lock().unwrap().clone();
                let mut coeffs2 = coeffs1.clone();
                let mut rng = thread_rng();
                let updates1: [i64; N_MUTATIONS] =
                    array::from_fn(|_| rng.gen_range(-MAX_MUTATION..=MAX_MUTATION));
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
        println!("Epoch {epoch} done.");

        if epoch.is_multiple_of(10) {
            let best_coeffs = best_coeffs.lock().unwrap().clone();
            match write_coeffs(&best_coeffs) {
                Ok(()) => eprintln!("Best coeffs written to file {COEFFS_FILE}"),
                Err(err) => eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`"),
            }
        }

        if epoch.is_multiple_of(100) {
            let best_coeffs = best_coeffs.lock().unwrap().clone();
            stats(best_coeffs, 50);
        }
    });
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
