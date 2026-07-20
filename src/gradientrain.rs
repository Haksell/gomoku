use crate::{
    bots::idabp::idabp,
    game::{Game, state::GameState},
    heuristics::{
        Heuristic,
        coeffistic::{
            COEFFS_FILE, INITIAL_COEFFS, MAX_THREATS, N_MUTATIONS, N_STENCIL_COEFFS,
            STENCIL_INDICES, STENCIL_INDICES_OPP, STENCIL_INDICES_SYM, STENCIL_INDICES_SYM_OPP,
            UNIQUE_STENCIL_INDICES, coeffistic, write_coeffs,
        },
    },
    player::{Player, PlayerColor},
};
use itertools::Itertools as _;
use nannou::rand::{Rng as _, thread_rng};
use rayon::iter::ParallelIterator as _;
use std::{
    array,
    sync::{Arc, Mutex},
};

const MAX_MULTIPLICATIVE_FACTOR: f64 = 0.1;
const MAX_ADDITIVE_FACTOR: f64 = 10.;
const LEARNING_RATE: f64 = 1. / 128.;
const GAMES_PER_EPOCH: usize = 20;

struct Params {
    coeffs: Vec<f64>,
    epoch: u32,
}

pub fn run() {
    assert!(GAMES_PER_EPOCH.is_multiple_of(4)); // TODO: test

    let params = Arc::new(Mutex::new(Params {
        coeffs: INITIAL_COEFFS.iter().map(|c| *c as f64).collect_vec(),
        epoch: 0,
    }));

    rayon::iter::repeat(()).for_each(|()| {
        let best_coeffs = params.lock().unwrap().coeffs.clone();

        let grads = (0..GAMES_PER_EPOCH / 4)
            .map(|_| {
                let mut coeffs1 = best_coeffs.clone();
                let mut coeffs2 = best_coeffs.clone();

                let mut rng = thread_rng();
                let updates1: [f64; N_MUTATIONS] = array::from_fn(|i| {
                    let update_range = (get_coeff(&coeffs1, i).abs() * MAX_MULTIPLICATIVE_FACTOR)
                        .max(MAX_ADDITIVE_FACTOR);
                    rng.gen_range(-update_range..=update_range)
                });

                for (i, &update1) in updates1.iter().enumerate() {
                    update_coeffs(&mut coeffs1, i, update1);
                    update_coeffs(&mut coeffs2, i, -update1);
                }

                let player1 = player_from_coeffs(round_coeffs(&coeffs1).into());
                let player2 = player_from_coeffs(round_coeffs(&coeffs2).into());

                let wins1 = play_four_games(&player1, &player2);
                let grad_factor = wins1 as f64 - 2.;
                updates1.map(|u1| u1 * grad_factor)
            })
            .reduce(|acc, res| array::from_fn(|i| acc[i] + res[i]))
            .unwrap();

        let epoch = {
            let mut params = params.lock().unwrap();
            params.epoch += 1;
            for i in 0..N_MUTATIONS {
                update_coeffs(&mut params.coeffs, i, LEARNING_RATE * grads[i]);
            }
            params.epoch
        };

        if epoch.is_multiple_of(10) {
            let rounded_coeffs = round_coeffs(&params.lock().unwrap().coeffs);
            match write_coeffs(&rounded_coeffs) {
                Ok(()) => println!("Epoch {epoch} done and saved."),
                Err(err) => eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`"),
            }
        }

        if epoch.is_multiple_of(100) {
            let rounded_coeffs = round_coeffs(&params.lock().unwrap().coeffs).into();
            stats(rounded_coeffs, 100);
        }
    });
}

fn round_coeffs(coeffs: &[f64]) -> Vec<i64> {
    coeffs.iter().map(|c| c.round() as i64).collect()
}

fn get_coeff(coeffs: &[f64], i: usize) -> f64 {
    if i >= UNIQUE_STENCIL_INDICES {
        coeffs[i - UNIQUE_STENCIL_INDICES + N_STENCIL_COEFFS]
    } else {
        coeffs[STENCIL_INDICES[i]]
    }
}

fn update_coeffs(coeffs: &mut [f64], i: usize, update: f64) {
    if i >= UNIQUE_STENCIL_INDICES {
        let ct_idx = i - UNIQUE_STENCIL_INDICES;
        let coeffs_idx = ct_idx + N_STENCIL_COEFFS;

        let is_additional = (ct_idx + 1).is_multiple_of(MAX_THREATS + 2);
        let is_nonzero_threat = !is_additional && !ct_idx.is_multiple_of(MAX_THREATS + 2);
        let is_nonzero_capture = ct_idx >= MAX_THREATS + 2;

        let mut new_coeff = coeffs[coeffs_idx] + update;
        if new_coeff < 0. {
            new_coeff = 0.;
        }
        if ct_idx == 0 {
            new_coeff = 0.;
        }
        if is_nonzero_threat {
            new_coeff = new_coeff.max(coeffs[coeffs_idx - 1]);
        }
        if is_nonzero_capture && !is_additional {
            new_coeff = new_coeff.max(coeffs[coeffs_idx - (MAX_THREATS + 2)]);
        }

        coeffs[coeffs_idx] = new_coeff;
    } else {
        coeffs[STENCIL_INDICES[i]] += update;
        coeffs[STENCIL_INDICES_OPP[i]] -= update;
        if STENCIL_INDICES[i] != STENCIL_INDICES_SYM[i] {
            coeffs[STENCIL_INDICES_SYM[i]] += update;
            coeffs[STENCIL_INDICES_SYM_OPP[i]] -= update;
        }
    }
}

fn play_four_games(player1: &Player, player2: &Player) -> u32 {
    let mut wins1 = 0;

    for random_moves in [3, 4] {
        let mut game_12 = Game::new(player1, player2);
        game_12.play_random_moves(random_moves, 5);

        let mut game_21 = game_12.clone();
        (game_21.black_player, game_21.white_player) = (game_21.white_player, game_21.black_player);

        game_12.play_game();
        game_21.play_game();

        wins1 += matches!(game_12.state, GameState::Won(PlayerColor::Black, _)) as u32;
        wins1 += matches!(game_21.state, GameState::Won(PlayerColor::White, _)) as u32;
    }

    wins1
}

fn stats(best_coeffs: Box<[i64]>, games: u32) {
    assert!(games.is_multiple_of(4));

    let new_player = player_from_coeffs(best_coeffs);
    let initial_player = player_from_coeffs(INITIAL_COEFFS.clone());

    let new_wins: u32 = (0..games / 4).map(|_| play_four_games(&new_player, &initial_player)).sum();

    let infooooo = format!("Current won {new_wins}/{games} games against initial bot.");
    let horizontal = "═".repeat(infooooo.len() + 2);
    println!("╔{horizontal}╗");
    println!("║ {infooooo} ║");
    println!("╚{horizontal}╝");
}

fn player_from_coeffs(coeffs: Box<[i64]>) -> Player {
    Player::Bot { bot: idabp, heuristic: Heuristic { fun: coeffistic, coeffs: Some(coeffs) } }
}
