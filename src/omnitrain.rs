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
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};
use std::{
    array,
    cmp::{max, min},
};

const N_MUTATIONS: usize = UNIQUE_STENCIL_INDICES + 9;

const GAMES_BY_EPOCH: usize = 200;
const PAIRS_BY_EPOCH: usize = GAMES_BY_EPOCH / 2;
const REQUIRED_WIN_DIFFERENTIAL: i32 = 18;

const MAX_ADDITIVE_MUTATION: i64 = 64;
// bias towards values closer to 0
const MIN_MULTIPLICATIVE_MUTATION: f64 = 0.79;
const MAX_MULTIPLICATIVE_MUTATION: f64 = 1.2;

const MAX_COEFF_VALUE: i64 = 999_999;
const MIN_COEFF_VALUE: i64 = -MAX_COEFF_VALUE;

pub fn run() {
    let mut best_coeffs = INITIAL_COEFFS.clone();

    for epoch in 1u64.. {
        let mutations: [i64; N_MUTATIONS] = array::from_fn(|i| {
            if i < UNIQUE_STENCIL_INDICES {
                random_coeff(best_coeffs[STENCIL_INDICES[i]])
            } else {
                random_coeff(best_coeffs[i - UNIQUE_STENCIL_INDICES + N_STENCIL_COEFFS]).max(0)
            }
        });

        #[expect(clippy::needless_range_loop, clippy::large_stack_arrays)]
        let should_mutate_2: [[bool; N_MUTATIONS]; PAIRS_BY_EPOCH] = {
            let mut should_mutate_2 = [[false; N_MUTATIONS]; PAIRS_BY_EPOCH];
            for i in 0..N_MUTATIONS {
                let mut remaining_mutations = PAIRS_BY_EPOCH / 2;
                for j in 0..PAIRS_BY_EPOCH {
                    let mutate_2 = thread_rng()
                        .gen_ratio(remaining_mutations as u32, (PAIRS_BY_EPOCH - j) as u32);
                    should_mutate_2[j][i] = mutate_2;
                    remaining_mutations -= mutate_2 as usize;
                }
            }
            should_mutate_2
        };

        let all_wins: Vec<u32> = (0..PAIRS_BY_EPOCH)
            .into_par_iter()
            .map(|pair_idx| {
                let mut coeffs1 = best_coeffs.clone();
                let mut coeffs2 = best_coeffs.clone();
                for (i, &mutate_2) in should_mutate_2[pair_idx].iter().enumerate() {
                    update_coeffs(
                        if mutate_2 { &mut coeffs2 } else { &mut coeffs1 },
                        i,
                        mutations[i],
                    );
                }
                let player1 = Player::Bot {
                    bot: idabp,
                    heuristic: Heuristic { fun: coeffistic, coeffs: Some(coeffs1) },
                };
                let player2 = Player::Bot {
                    bot: idabp,
                    heuristic: Heuristic { fun: coeffistic, coeffs: Some(coeffs2) },
                };
                play_pair(&player1, &player2)
            })
            .collect();

        let mut win_differentials = [0i32; N_MUTATIONS];
        for (pair_idx, wins) in all_wins.iter().enumerate() {
            let wins = *wins as i32;
            for (i, &mutate_2) in should_mutate_2[pair_idx].iter().enumerate() {
                win_differentials[i] += if mutate_2 { wins } else { -wins };
            }
        }

        let mut updates = 0;
        for (i, &win_differential) in win_differentials.iter().enumerate() {
            // TODO: lerp between 0 and REQUIRED_WIN_DIFFERENTIAL (if so, increase the constant)
            if win_differential >= REQUIRED_WIN_DIFFERENTIAL {
                updates += 1;
                update_coeffs(&mut best_coeffs, i, mutations[i]);
            }
        }

        println!("Epoch #{epoch}: {updates} updates");
        if updates != 0
            && let Err(err) = write_coeffs(&best_coeffs)
        {
            eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`");
        }

        if epoch.is_multiple_of(10) {
            let new_player = Player::Bot {
                bot: idabp,
                heuristic: Heuristic { fun: coeffistic, coeffs: Some(best_coeffs.clone()) },
            };
            let initial_player = Player::Bot {
                bot: idabp,
                heuristic: Heuristic { fun: coeffistic, coeffs: Some(INITIAL_COEFFS.clone()) },
            };
            let pairs = 50;
            let total_games = 2 * pairs;
            let wins_against_initial = play_pairs(pairs, &initial_player, &new_player);
            let wins_against_manual = play_pairs(pairs, &Player::MANUAL, &new_player);
            let dividing_line = "=".repeat(80);
            println!("{dividing_line}");
            println!("Current won {wins_against_initial}/{total_games} games against initial bot");
            println!("Current won {wins_against_manual}/{total_games} games against manual bot");
            println!("{dividing_line}");
        }
    }
}

fn random_coeff(old_coeff: i64) -> i64 {
    let min_mul = (old_coeff as f64 * MIN_MULTIPLICATIVE_MUTATION).round() as i64;
    let max_mul = (old_coeff as f64 * MAX_MULTIPLICATIVE_MUTATION).round() as i64;
    let min_range =
        max(MIN_COEFF_VALUE, min(old_coeff - MAX_ADDITIVE_MUTATION, min(min_mul, max_mul)));
    let max_range =
        min(MAX_COEFF_VALUE, max(old_coeff + MAX_ADDITIVE_MUTATION, max(min_mul, max_mul)));

    // trick to avoid returning old_coeff
    let new_coeff = thread_rng().gen_range(min_range..max_range);
    if new_coeff * old_coeff < 0 {
        0 // prefer setting to 0 than opposite sign to keep a parcimonious model
    } else if new_coeff >= old_coeff {
        new_coeff + 1
    } else {
        new_coeff
    }
}

fn update_coeffs(coeffs: &mut [i64], i: usize, new_value: i64) {
    if i >= UNIQUE_STENCIL_INDICES {
        coeffs[i - UNIQUE_STENCIL_INDICES + N_STENCIL_COEFFS] = new_value;
    } else {
        coeffs[STENCIL_INDICES[i]] = new_value;
        coeffs[STENCIL_INDICES_SYM[i]] = new_value;
        coeffs[STENCIL_INDICES_OPP[i]] = -new_value;
        coeffs[STENCIL_INDICES_SYM_OPP[i]] = -new_value;
    }
}

fn play_pairs(pairs: usize, old_player: &Player, new_player: &Player) -> u32 {
    (0..pairs).into_par_iter().map(|_| play_pair(old_player, new_player)).sum()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn games_by_epoch() {
        // mutate/keep, black/white
        assert!(GAMES_BY_EPOCH.is_multiple_of(4));
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
