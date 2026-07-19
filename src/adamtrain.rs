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

const LEARNING_RATE: f64 = 1.;
const BETA_M: f64 = 0.95;
const BETA_V: f64 = 0.9995;
const EPS: f64 = 1e-8;

struct Params {
    coeffs: Vec<f64>,
    m: Vec<f64>,
    v: Vec<f64>,
    epoch: u32,
}

pub fn run() {
    let params = Arc::new(Mutex::new(Params {
        coeffs: INITIAL_COEFFS.iter().map(|c| *c as f64).collect_vec(),
        m: vec![0.; N_COEFFS],
        v: vec![0.; N_COEFFS],
        epoch: 0,
    }));

    rayon::iter::repeat(()).for_each(|()| {
        let mut rng = thread_rng();
        let updates1: [f64; N_MUTATIONS] = array::from_fn(|_| rng.gen_range(-32.0..=32.0));

        let mut coeffs1 = params.lock().unwrap().coeffs.clone();
        let mut coeffs2 = coeffs1.clone();
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
            0 => updates1,
            1 => [0.; N_MUTATIONS],
            2 => updates1.map(|u| -u),
            _ => unreachable!(),
        };

        let epoch = {
            let mut params = params.lock().unwrap();
            params.epoch += 1;
            let bias_correction_m = 1. / (1. - BETA_M.powi(params.epoch as i32));
            let bias_correction_v = 1. / (1. - BETA_V.powi(params.epoch as i32));
            for i in 0..N_MUTATIONS {
                params.m[i] = BETA_M * params.m[i] + (1. - BETA_M) * grads[i];
                params.v[i] = BETA_V * params.v[i] + (1. - BETA_V) * grads[i].powi(2);
                let m_hat = params.m[i] * bias_correction_m;
                let v_hat = params.v[i] * bias_correction_v;
                let update = LEARNING_RATE * m_hat / (v_hat.sqrt() + EPS);
                update_coeffs(&mut params.coeffs, i, update);
            }
            params.epoch
        };

        println!("Epoch {epoch} done.");

        if epoch.is_multiple_of(25) {
            let best_coeffs = round_coeffs(&params.lock().unwrap().coeffs);
            match write_coeffs(&best_coeffs) {
                Ok(()) => eprintln!("Best coeffs written to file {COEFFS_FILE}"),
                Err(err) => eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`"),
            }
        }

        if epoch.is_multiple_of(100) {
            println!("{:?}", &params.lock().unwrap().coeffs[0..32]);
            println!("{:?}", &params.lock().unwrap().m[0..32]);
            println!("{:?}", &params.lock().unwrap().v[0..32]);
            // let best_coeffs = round_coeffs(&params.lock().unwrap().coeffs).into_boxed_slice();
            // stats(best_coeffs, 50);
        }
    });
}

fn round_coeffs(coeffs: &[f64]) -> Vec<i64> {
    coeffs.iter().map(|c| c.round() as i64).collect()
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
