use crate::{
    bots::idabp::idabp,
    game::{Game, state::GameState},
    heuristics::{
        Heuristic,
        coeffistic::{
            COEFFS_FILE, INITIAL_COEFFS, N_COEFFS, N_STENCIL_COEFFS, STENCIL_INDICES,
            STENCIL_INDICES_OPP, STENCIL_INDICES_SYM, STENCIL_INDICES_SYM_OPP,
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

const N_MUTATIONS: usize = UNIQUE_STENCIL_INDICES + 9;

const MAX_MULTIPLICATIVE_FACTOR: f64 = 0.1;
const MAX_ADDITIVE_FACTOR: f64 = 16.;
const LEARNING_RATE: f64 = 0.005;
const GAMMA: f64 = 0.9;

struct Params {
    coeffs: Vec<f64>,
    velocity: Vec<f64>,
    epoch: u32,
}

pub fn run() {
    let params = Arc::new(Mutex::new(Params {
        coeffs: INITIAL_COEFFS.iter().map(|c| *c as f64).collect_vec(),
        velocity: vec![0.; N_COEFFS],
        epoch: 0,
    }));

    rayon::iter::repeat(()).for_each(|()| {
        let mut coeffs1 = params.lock().unwrap().coeffs.clone();
        let mut coeffs2 = coeffs1.clone();

        let mut rng = thread_rng();
        let updates1: [f64; N_MUTATIONS] = array::from_fn(|i| {
            let old_coeff = get_coeff(&coeffs1, i);
            let update_range =
                (old_coeff.abs() * MAX_MULTIPLICATIVE_FACTOR).max(MAX_ADDITIVE_FACTOR);
            rng.gen_range(-update_range..=update_range)
        });

        for (i, &update1) in updates1.iter().enumerate() {
            update_coeffs(&mut coeffs1, i, update1);
            update_coeffs(&mut coeffs2, i, -update1);
        }

        let player1 = Player::Bot {
            bot: idabp,
            heuristic: Heuristic { fun: coeffistic, coeffs: Some(round_coeffs(&coeffs1).into()) },
        };
        let player2 = Player::Bot {
            bot: idabp,
            heuristic: Heuristic { fun: coeffistic, coeffs: Some(round_coeffs(&coeffs2).into()) },
        };

        let grads = match play_pair(&player1, &player2) {
            0 => updates1.map(|u1| -u1),
            1 => [0.; N_MUTATIONS],
            2 => updates1,
            _ => unreachable!(),
        };

        let epoch = {
            let mut params = params.lock().unwrap();
            params.epoch += 1;
            for i in 0..N_MUTATIONS {
                params.velocity[i] = GAMMA * params.velocity[i] + LEARNING_RATE * grads[i];
                let update = -params.velocity[i];
                update_coeffs(&mut params.coeffs, i, update);
            }
            params.epoch
        };

        if epoch.is_multiple_of(100) {
            let best_coeffs = round_coeffs(&params.lock().unwrap().coeffs);
            match write_coeffs(&best_coeffs) {
                Ok(()) => println!("Epoch {epoch} done and saved."),
                Err(err) => eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`"),
            }
        }

        // if epoch.is_multiple_of(500) {
        //     let best_coeffs = round_coeffs(&params.lock().unwrap().coeffs).into_boxed_slice();
        //     stats(best_coeffs, 50);
        // }
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
