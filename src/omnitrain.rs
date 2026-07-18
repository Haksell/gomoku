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

const N_MUTATIONS: usize = UNIQUE_STENCIL_INDICES + 9;

pub fn run() {
    let initial_player = Player::Bot {
        bot: idabp,
        heuristic: Heuristic { fun: coeffistic, coeffs: Some(INITIAL_COEFFS.clone()) },
    };
    let mut best_coeffs = INITIAL_COEFFS.clone();

    for epoch in 1u64.. {
        let mut coeffs1 = best_coeffs.clone();
        let mut coeffs2 = best_coeffs.clone();
        for i in 0..N_MUTATIONS {
            if thread_rng().gen_ratio(1, 2) {
                update_coeffs(&mut coeffs1, i, -1);
                update_coeffs(&mut coeffs2, i, 1);
            } else {
                update_coeffs(&mut coeffs1, i, 1);
                update_coeffs(&mut coeffs2, i, -1);
            }
        }

        let player1 = Player::Bot {
            bot: idabp,
            heuristic: Heuristic { fun: coeffistic, coeffs: Some(coeffs1.clone()) },
        };
        let player2 = Player::Bot {
            bot: idabp,
            heuristic: Heuristic { fun: coeffistic, coeffs: Some(coeffs2.clone()) },
        };

        match play_pair(&player1, &player2) {
            0 => best_coeffs.clone_from(&coeffs1),
            1 => {}
            2 => best_coeffs.clone_from(&coeffs2),
            _ => unreachable!(),
        }

        if epoch.is_multiple_of(10) {
            println!("Epoch {epoch} done.");
            if let Err(err) = write_coeffs(&best_coeffs) {
                eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`");
            }
        }

        if epoch.is_multiple_of(100) {
            let new_player = Player::Bot {
                bot: idabp,
                heuristic: Heuristic { fun: coeffistic, coeffs: Some(best_coeffs.clone()) },
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

fn update_coeffs(coeffs: &mut [i64], i: usize, update: i64) {
    if i >= UNIQUE_STENCIL_INDICES {
        coeffs[i - UNIQUE_STENCIL_INDICES + N_STENCIL_COEFFS] += update;
    } else {
        coeffs[STENCIL_INDICES[i]] += update;
        coeffs[STENCIL_INDICES_SYM[i]] += update;
        coeffs[STENCIL_INDICES_OPP[i]] -= update;
        coeffs[STENCIL_INDICES_SYM_OPP[i]] -= update;
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
