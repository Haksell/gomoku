use std::io;

use crate::{
    bots::idabp_new::idabp_new,
    game::{Game, state::GameState},
    heuristics::{Heuristic, coeff_heuristic::coeff_heuristic},
    player::{Player, PlayerColor},
};
use nannou::rand::{Rng as _, thread_rng};

const COEFFS_FILE: &str = "./weights/duel.rs";

const N_COEFFS: usize = 729 + 9;
const EPOCHS: usize = 1 << 16;
const COUNT_MUTATIONS: usize = 64;
const MAX_MUTATION: i64 = 64;

pub fn run() {
    let mut rng = thread_rng();
    let mut coeffs = include!("../weights/duel.rs");

    for epoch in 0..EPOCHS {
        let old_player = Player::Bot {
            bot: idabp_new,
            heuristic: Heuristic { fun: coeff_heuristic, coeffs: Some(coeffs) },
        };

        let mut new_coeffs = coeffs;
        for _ in 0..COUNT_MUTATIONS {
            let i = rng.gen_range(0..N_COEFFS);
            let mutation = rng.gen_range(-MAX_MUTATION..=MAX_MUTATION);
            new_coeffs[i] += mutation;
        }
        let new_player = Player::Bot {
            bot: idabp_new,
            heuristic: Heuristic { fun: coeff_heuristic, coeffs: Some(new_coeffs) },
        };

        let mut old_new = Game::new(old_player, new_player);
        let random_moves = rng.gen_range(3..=4);
        old_new.play_random_moves(random_moves, 5);

        let mut new_old = old_new.clone();
        (new_old.black_player, new_old.white_player) = (new_old.white_player, new_old.black_player);

        old_new.play_game();
        new_old.play_game();

        let new_wins = matches!(old_new.state, GameState::Won(PlayerColor::White, _)) as u8
            + matches!(new_old.state, GameState::Won(PlayerColor::Black, _)) as u8;

        println!("{epoch}: {new_wins}");

        match new_wins {
            0 => {
                println!("Bad!");
                coeffs = std::array::from_fn(|i| (3 * coeffs[i] - new_coeffs[i]) / 2);
            }
            1 => {}
            2 => {
                println!("Good!");
                coeffs = new_coeffs;
            }
            _ => unreachable!(),
        }

        if new_wins != 1 {
            match write_coeffs(&coeffs) {
                Ok(()) => println!("coeffs written to file {COEFFS_FILE}"),
                Err(err) => eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`"),
            }
        }
    }
}

fn write_coeffs(coeffs: &[i64; N_COEFFS]) -> io::Result<()> {
    use std::io::Write as _;
    let mut file = std::fs::File::create(COEFFS_FILE)?;
    write!(file, "{coeffs:?}")
}
