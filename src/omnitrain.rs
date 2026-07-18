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
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};
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

    (0..u128::MAX).into_par_iter().for_each(|_| {
        let mut coeffs1 = best_coeffs.lock().unwrap().clone();
        let mut coeffs2 = coeffs1.clone();
        let updates1: [i64; N_MUTATIONS] =
            array::from_fn(|_| if thread_rng().gen_ratio(1, 2) { 1 } else { -1 });
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

        let wins2 = play_pair(&player1, &player2);
        if wins2 != 1 {
            let mut best_coeffs = best_coeffs.lock().unwrap();
            for (i, &update1) in updates1.iter().enumerate() {
                update_coeffs(&mut best_coeffs, i, if wins2 == 0 { update1 } else { -update1 });
            }
        }

        let epoch = epoch.fetch_add(1, Ordering::Relaxed) + 1;
        println!("Epoch {epoch} done.");

        if epoch.is_multiple_of(10) {
            // clone to release the lock
            let best_coeffs = best_coeffs.lock().unwrap().clone();
            match write_coeffs(&best_coeffs) {
                Ok(()) => eprintln!("Best coeffs written to file {COEFFS_FILE}"),
                Err(err) => eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`"),
            }
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
