use crate::{
    bots::idabp_new::idabp_new,
    game::{Game, state::GameState},
    heuristics::{Heuristic, coeff_heuristic::coeff_heuristic},
    player::{Player, PlayerColor},
};
use nannou::rand::{Rng as _, rngs::ThreadRng, thread_rng};
use std::{
    fs::File,
    io::{self, Write as _},
};

const COEFFS_FILE: &str = "./weights/duel.rs";

const N_COEFFS: usize = 729 + 9;
const EPOCHS: usize = 1 << 20;
const MAX_MUTATION: i64 = 16;

pub fn run() {
    let mut rng = thread_rng();
    let mut coeffs = include!("../weights/duel.rs");

    let mut updates = 0;

    let initial_player = Player::Bot {
        bot: idabp_new,
        heuristic: Heuristic { fun: coeff_heuristic, coeffs: Some(coeffs) },
    };

    for epoch in 1..=EPOCHS {
        let old_player = Player::Bot {
            bot: idabp_new,
            heuristic: Heuristic { fun: coeff_heuristic, coeffs: Some(coeffs) },
        };

        let mut new_coeffs = coeffs;
        for coeff in &mut new_coeffs {
            let mutation = rng.gen_range(-MAX_MUTATION..=MAX_MUTATION);
            *coeff += mutation;
        }
        let new_player = Player::Bot {
            bot: idabp_new,
            heuristic: Heuristic { fun: coeff_heuristic, coeffs: Some(new_coeffs) },
        };

        let new_wins = play_two_games(old_player, new_player, &mut rng);

        if new_wins == 2 {
            updates += 1;
            println!("Updated! ({updates} updates in {epoch} epochs)");
            coeffs = new_coeffs;
            match write_coeffs(&coeffs) {
                Ok(()) => println!("coeffs written to file {COEFFS_FILE}"),
                Err(err) => eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`"),
            }
        }

        if epoch.is_multiple_of(100) {
            let genetic_player = Player::Bot {
                bot: idabp_new,
                heuristic: Heuristic { fun: coeff_heuristic, coeffs: Some(coeffs) },
            };
            let mut total_wins = 0;
            for _ in 0..10 {
                let wins = play_two_games(initial_player, genetic_player, &mut rng);
                total_wins += wins;
            }
            println!("Current won {total_wins}/20 games against initial bot");
        }
    }
}

fn play_two_games(old_player: Player, new_player: Player, rng: &mut ThreadRng) -> u8 {
    let mut old_new = Game::new(old_player, new_player);
    let random_moves = rng.gen_range(3..=4);
    old_new.play_random_moves(random_moves, 5);

    let mut new_old = old_new.clone();
    (new_old.black_player, new_old.white_player) = (new_old.white_player, new_old.black_player);

    old_new.play_game();
    new_old.play_game();

    matches!(old_new.state, GameState::Won(PlayerColor::White, _)) as u8
        + matches!(new_old.state, GameState::Won(PlayerColor::Black, _)) as u8
}

fn write_coeffs(coeffs: &[i64; N_COEFFS]) -> io::Result<()> {
    let mut file = File::create(COEFFS_FILE)?;
    writeln!(file, "[")?;

    for i in 0..729 {
        let c = coeffs[i];
        // TODO: check correct direction (might be symmetric)
        let pat: String = (0..6).map(|j| ['.', 'b', 'w'][i / 3usize.pow(j) % 3]).collect();
        let num = format!("{c},");
        writeln!(file, "    {num:7}// {pat}")?;
    }

    for (i, poly_coeff) in
        ["ccc", "cc", "c", "ttt", "tt", "t", "ct", "cct", "ctt"].iter().enumerate()
    {
        let c = coeffs[729 + i];
        let num = format!("{c},");
        writeln!(file, "    {num:7}// {poly_coeff}")?;
    }

    writeln!(file, "]")?;
    Ok(())
}
