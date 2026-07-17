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
use itertools::Itertools as _;
use nannou::rand::{Rng as _, rngs::ThreadRng, thread_rng};
use rayon::{
    ThreadPoolBuilder,
    iter::{IntoParallelIterator as _, ParallelIterator as _},
};
use std::{
    cmp::{max, min},
    sync::{Arc, Mutex},
    thread::available_parallelism,
};

const N_MUTATIONS: usize = 6;
const REQUIRED_WINS: u32 = 40;

const MAX_ADDITIVE_MUTATION: i64 = 256;
// bias towards values closer to 0
const MIN_MULTIPLICATIVE_MUTATION: f64 = 0.69;
const MAX_MULTIPLICATIVE_MUTATION: f64 = 1.3;

const MAX_COEFF_VALUE: i64 = 999_999;
const MIN_COEFF_VALUE: i64 = -MAX_COEFF_VALUE;

#[expect(clippy::too_many_lines)]
pub fn run(num_threads: Option<usize>) {
    let best_coeffs = Arc::new(Mutex::new(INITIAL_COEFFS.clone()));
    let stats = Arc::new(Mutex::new((0u32, 0u32)));

    // TODO: if 1 thread, no parallelism
    // TODO: no global (if we need to do stuff after training)
    // TODO: understand why 10 threads is faster than 20
    let num_threads = num_threads.unwrap_or(1); // TODO: if 1, no par_iter
    let available_cpus = available_parallelism().unwrap().get();
    assert!(num_threads > 0, "Can't run with 0 threads.");
    assert!(
        num_threads <= available_cpus,
        "You asked for {num_threads} threads but only {available_cpus} threads are available.",
    );
    ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();

    // TODO: find a cleaner infinite loop
    (0..usize::MAX).into_par_iter().for_each(|_| {
        let prev_coeffs = best_coeffs.lock().unwrap().clone();
        let prev_player = Player::Bot {
            bot: idabp,
            heuristic: Heuristic { fun: coeffistic, coeffs: Some(prev_coeffs.clone()) },
        };

        let mut rng = thread_rng();

        let mut mutations = [(usize::MAX, 0); N_MUTATIONS];
        let mut n_mutations = 0;
        while n_mutations < N_MUTATIONS {
            if rng.gen_ratio(1, 8) {
                let coeff_idx = rng.gen_range(N_STENCIL_COEFFS..N_COEFFS);
                if mutations.iter().any(|(j, _)| coeff_idx == *j) {
                    continue;
                }
                let new_coeff = random_coeff(&mut rng, prev_coeffs[coeff_idx]).max(0);
                mutations[n_mutations] = (coeff_idx, new_coeff);
                n_mutations += 1;
                continue;
            }

            let i = rng.gen_range(0..STENCIL_INDICES.len());
            if mutations.iter().any(|(j, _)| i == *j) {
                continue;
            }

            let new_coeff = random_coeff(&mut rng, prev_coeffs[STENCIL_INDICES[i]]);
            mutations[n_mutations] = (i, new_coeff);
            n_mutations += 1;
        }

        let mut total_wins = [0; N_MUTATIONS];
        for mutated_indices in (0..N_MUTATIONS).powerset() {
            if mutated_indices.is_empty() {
                continue;
            }

            let mut new_coeffs = prev_coeffs.clone();
            for i in &mutated_indices {
                let (coeff_idx, new_value) = mutations[*i];
                if coeff_idx >= N_STENCIL_COEFFS {
                    new_coeffs[coeff_idx] = new_value;
                } else {
                    new_coeffs[STENCIL_INDICES[coeff_idx]] = new_value;
                    new_coeffs[STENCIL_INDICES_SYM[coeff_idx]] = new_value;
                    new_coeffs[STENCIL_INDICES_OPP[coeff_idx]] = -new_value;
                    new_coeffs[STENCIL_INDICES_SYM_OPP[coeff_idx]] = -new_value;
                }
            }

            let new_player = Player::Bot {
                bot: idabp,
                heuristic: Heuristic { fun: coeffistic, coeffs: Some(new_coeffs) },
            };
            let wins = play_pair(&prev_player, &new_player, &mut rng);
            for i in &mutated_indices {
                total_wins[*i] += wins;
            }
        }

        let updates = (0..N_MUTATIONS).filter(|i| total_wins[*i] >= REQUIRED_WINS).collect_vec();

        let mut stats = stats.lock().unwrap();
        stats.1 += 1;
        if !updates.is_empty() {
            stats.0 += updates.len() as u32;
            println!("Updated! ({} updates in {} epochs)", stats.0, stats.1);
        }
        let epoch = stats.1;
        drop(stats);

        if !updates.is_empty() {
            let mut coeffs_lock = best_coeffs.lock().unwrap();
            for i in updates {
                let (coeff_idx, new_value) = mutations[i];
                if coeff_idx >= N_STENCIL_COEFFS {
                    coeffs_lock[coeff_idx] = new_value;
                } else {
                    coeffs_lock[STENCIL_INDICES[coeff_idx]] = new_value;
                    coeffs_lock[STENCIL_INDICES_SYM[coeff_idx]] = new_value;
                    coeffs_lock[STENCIL_INDICES_OPP[coeff_idx]] = -new_value;
                    coeffs_lock[STENCIL_INDICES_SYM_OPP[coeff_idx]] = -new_value;
                }
            }
            let coeffs_to_write = coeffs_lock.clone();
            drop(coeffs_lock);
            if let Err(err) = write_coeffs(&coeffs_to_write) {
                eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`");
            }
        }

        if epoch.is_multiple_of(200) {
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
            let pairs = 50;
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
    let min_mul = (old_coeff as f64 * MIN_MULTIPLICATIVE_MUTATION).round() as i64;
    let max_mul = (old_coeff as f64 * MAX_MULTIPLICATIVE_MUTATION).round() as i64;
    let min_range =
        max(MIN_COEFF_VALUE, min(old_coeff - MAX_ADDITIVE_MUTATION, min(min_mul, max_mul)));
    let max_range =
        min(MAX_COEFF_VALUE, max(old_coeff + MAX_ADDITIVE_MUTATION, max(min_mul, max_mul)));

    // trick to avoid returning old_coeff
    let new_coeff = rng.gen_range(min_range..max_range);
    if new_coeff * old_coeff < 0 {
        0 // prefer setting to 0 than opposite sign to keep a parcimonious model
    } else if new_coeff >= old_coeff {
        new_coeff + 1
    } else {
        new_coeff
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_wins() {
        assert!(REQUIRED_WINS > 2u32.pow(N_MUTATIONS as u32 - 1));
        assert!(REQUIRED_WINS <= 2u32.pow(N_MUTATIONS as u32));
    }

    #[test]
    fn multiplicative_bias_towards_zero() {
        assert!(MIN_MULTIPLICATIVE_MUTATION > 0.);
        assert!(MIN_MULTIPLICATIVE_MUTATION < 1.);
        assert!(MAX_MULTIPLICATIVE_MUTATION > 1.);
        assert!(MAX_MULTIPLICATIVE_MUTATION < 2.);
        let mean = f64::midpoint(MIN_MULTIPLICATIVE_MUTATION, MAX_MULTIPLICATIVE_MUTATION);
        assert!(mean > 0.95);
        assert!(mean < 1.00);
    }
}
