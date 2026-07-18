use crate::{
    bots::idabp::idabp,
    game::{Game, state::GameState},
    heuristics::{
        Heuristic,
        coeffistic::{
            COEFFS_FILE, INITIAL_COEFFS, N_COEFFS, N_STENCIL_COEFFS, STENCIL_INDICES,
            STENCIL_INDICES_OPP, STENCIL_INDICES_SYM, STENCIL_INDICES_SYM_OPP, coeffistic,
            write_coeffs,
        },
    },
    player::{Player, PlayerColor},
};
use nannou::rand::{Rng as _, rngs::ThreadRng, thread_rng};
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};
use std::{
    cmp::{max, min},
    sync::{Arc, Mutex},
};

const EPOCHS: usize = 100_000;
const N_MUTATIONS: i64 = 1;
const MAX_ADDITIVE_MUTATION: i64 = 16;
const MAX_MULTIPLICATIVE_MUTATION: f64 = 1.2;
const MAX_COEFF_VALUE: i64 = 999_999;
const MIN_COEFF_VALUE: i64 = -MAX_COEFF_VALUE;

pub fn run() {
    let best_coeffs = Arc::new(Mutex::new(INITIAL_COEFFS.clone()));
    let stats = Arc::new(Mutex::new((0u32, 0u32)));

    (0..EPOCHS).into_par_iter().for_each(|_| {
        let mut rng = thread_rng();
        let old_player = Player::Bot {
            bot: idabp,
            heuristic: Heuristic {
                fun: coeffistic,
                coeffs: Some(best_coeffs.lock().unwrap().clone()),
            },
        };

        let mut new_coeffs = best_coeffs.lock().unwrap().clone();
        let mut mutations = vec![];
        for _ in 0..N_MUTATIONS {
            if rng.gen_ratio(1, 8) {
                let i = rng.gen_range(N_STENCIL_COEFFS..N_COEFFS);
                let new_coeff = random_coeff(&mut rng, new_coeffs[i]).max(0);
                mutations.push((i, new_coeff));
                continue;
            }

            let i = rng.gen_range(0..STENCIL_INDICES.len());
            let stencil_idx = STENCIL_INDICES[i];
            let stencil_idx_sym = STENCIL_INDICES_SYM[i];
            let stencil_idx_opp = STENCIL_INDICES_OPP[i];
            let stencil_idx_sym_opp = STENCIL_INDICES_SYM_OPP[i];
            let new_coeff = random_coeff(&mut rng, new_coeffs[stencil_idx]);

            mutations.push((stencil_idx, new_coeff));
            mutations.push((stencil_idx_sym, new_coeff));
            mutations.push((stencil_idx_opp, -new_coeff));
            mutations.push((stencil_idx_sym_opp, -new_coeff));
        }

        for &(i, mutation) in &mutations {
            new_coeffs[i] = mutation;
        }

        let new_player = Player::Bot {
            bot: idabp,
            heuristic: Heuristic { fun: coeffistic, coeffs: Some(new_coeffs) },
        };

        let total_wins = play_pairs(6, &old_player, &new_player, &mut rng);
        let should_update = total_wins >= 9;

        let mut stats = stats.lock().unwrap();
        stats.1 += 1;
        if should_update {
            stats.0 += 1;
            println!("Updated! ({} updates in {} epochs)", stats.0, stats.1);
        }
        let epoch = stats.1;
        drop(stats);

        if should_update {
            let mut coeffs_lock = best_coeffs.lock().unwrap();
            for &(i, mutation) in &mutations {
                coeffs_lock[i] = mutation;
            }
            let coeffs_to_write = coeffs_lock.clone();
            drop(coeffs_lock);
            if let Err(err) = write_coeffs(&coeffs_to_write) {
                eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`");
            }
        }

        if epoch.is_multiple_of(500) {
            let best_player = Player::Bot {
                bot: idabp,
                heuristic: Heuristic {
                    fun: coeffistic,
                    coeffs: Some(best_coeffs.lock().unwrap().clone()),
                },
            };
            let initial_player = Player::Bot {
                bot: idabp,
                heuristic: Heuristic { fun: coeffistic, coeffs: Some(INITIAL_COEFFS.clone()) },
            };
            let pairs = 25;
            let total_games = 2 * pairs;
            let wins_against_initial = play_pairs(pairs, &initial_player, &best_player, &mut rng);
            let wins_against_manual = play_pairs(pairs, &Player::MANUAL, &best_player, &mut rng);
            let dividing_line = "=".repeat(80);
            println!("{dividing_line}");
            println!("Current won {wins_against_initial}/{total_games} games against initial bot");
            println!("Current won {wins_against_manual}/{total_games} games against manual bot");
            println!("{dividing_line}");
        }
    });
}

fn random_coeff(rng: &mut ThreadRng, old_coeff: i64) -> i64 {
    let div_value = (old_coeff as f64 / MAX_MULTIPLICATIVE_MUTATION).round() as i64;
    let mul_value = (old_coeff as f64 * MAX_MULTIPLICATIVE_MUTATION).round() as i64;
    let min_range =
        max(MIN_COEFF_VALUE, min(old_coeff - MAX_ADDITIVE_MUTATION, min(div_value, mul_value)));
    let max_range =
        min(MAX_COEFF_VALUE, max(old_coeff + MAX_ADDITIVE_MUTATION, max(div_value, mul_value)));

    // trick to avoid returning old_coeff
    let new_coeff = rng.gen_range(min_range..max_range);
    if new_coeff >= old_coeff { new_coeff + 1 } else { new_coeff }
}

fn play_pairs(pairs: usize, old_player: &Player, new_player: &Player, rng: &mut ThreadRng) -> u32 {
    std::iter::repeat_with(|| play_pair(old_player, new_player, rng)).take(pairs).sum()
}

fn play_pair(old_player: &Player, new_player: &Player, rng: &mut ThreadRng) -> u32 {
    let mut old_new = Game::new(old_player, new_player);
    let random_moves = rng.gen_range(3..=4);
    old_new.play_random_moves(random_moves, 5);

    let mut new_old = old_new.clone();
    (new_old.black_player, new_old.white_player) = (new_old.white_player, new_old.black_player);

    old_new.play_game();
    new_old.play_game();

    matches!(old_new.state, GameState::Won(PlayerColor::White, _)) as u32
        + matches!(new_old.state, GameState::Won(PlayerColor::Black, _)) as u32
}
