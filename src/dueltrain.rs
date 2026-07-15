use crate::{
    bots::idabp_new::idabp_new,
    game::{Game, state::GameState},
    heuristics::{Heuristic, coeff_heuristic::coeff_heuristic},
    player::{Player, PlayerColor},
};
use nannou::rand::{Rng as _, rngs::ThreadRng, thread_rng};
use std::{
    cmp::{max, min},
    fs::File,
    io::{self, BufWriter, Write as _},
};

const COEFFS_FILE: &str = "./weights/duel.rs";

const N_COEFFS: usize = 729 + 9;
const EPOCHS: usize = 1 << 20;
const N_MUTATIONS: i64 = 16;
const MAX_ADDITIVE_MUTATION: i64 = 8;
const MAX_MULTIPLICATIVE_MUTATION: f64 = 1.1;

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
        for _ in 0..N_MUTATIONS {
            let i = rng.gen_range(0..N_COEFFS);
            let div_value = (coeffs[i] as f64 / MAX_MULTIPLICATIVE_MUTATION).round() as i64;
            let mul_value = (coeffs[i] as f64 * MAX_MULTIPLICATIVE_MUTATION).round() as i64;
            let min_range = min(coeffs[i] - MAX_ADDITIVE_MUTATION, min(div_value, mul_value));
            let max_range = max(coeffs[i] + MAX_ADDITIVE_MUTATION, max(div_value, mul_value));
            new_coeffs[i] = rng.gen_range(min_range..=max_range);
        }

        // TODO: optimize
        for i in 0..729 {
            let swap_12 = |x| if x == 0 { 0 } else { 3 - x };
            let x0 = i % 3;
            let x1 = i / 3 % 3;
            let x2 = i / 9 % 3;
            let x3 = i / 27 % 3;
            let x4 = i / 81 % 3;
            let x5 = i / 243 % 3;
            if x0 == swap_12(x5) && x1 == swap_12(x4) && x2 == swap_12(x3) {
                new_coeffs[i] = 0;
                continue;
            }
            let sym = x5 + 3 * x4 + 9 * x3 + 27 * x2 + 81 * x1 + 243 * x0;
            new_coeffs[sym] = new_coeffs[i];
            let opp = swap_12(x0)
                + 3 * swap_12(x1)
                + 9 * swap_12(x2)
                + 27 * swap_12(x3)
                + 81 * swap_12(x4)
                + 243 * swap_12(x5);
            new_coeffs[opp] = -new_coeffs[i];
            let sym_opp = swap_12(x5)
                + 3 * swap_12(x4)
                + 9 * swap_12(x3)
                + 27 * swap_12(x2)
                + 81 * swap_12(x1)
                + 243 * swap_12(x0);
            new_coeffs[sym_opp] = -new_coeffs[i];
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
            match write_coeffs(&new_coeffs) {
                Ok(()) => println!("coeffs written to file {COEFFS_FILE}"),
                Err(err) => eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`"),
            }
        }

        if epoch.is_multiple_of(100) {
            let genetic_player = Player::Bot {
                bot: idabp_new,
                heuristic: Heuristic { fun: coeff_heuristic, coeffs: Some(coeffs) },
            };
            let (opponent, opponent_name) = if epoch.is_multiple_of(200) {
                (initial_player, "initial")
            } else {
                (Player::NEW, "manual")
            };
            let mut total_wins = 0;
            for _ in 0..10 {
                let wins = play_two_games(opponent, genetic_player, &mut rng);
                total_wins += wins;
            }
            println!("Current won {total_wins}/20 games against {opponent_name} bot");
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
    let mut buf = BufWriter::with_capacity(1 << 15, Vec::new());
    writeln!(buf, "[")?;

    for i in 0..729 {
        let c = coeffs[i];
        // TODO: check correct direction (might be symmetric)
        let pat: String = (0..6).map(|j| ['.', 'b', 'w'][i / 3usize.pow(j) % 3]).collect();
        let num = format!("{c},");
        writeln!(buf, "    {num:7}// {pat}")?;
    }

    for (i, poly_coeff) in
        ["ccc", "cc", "c", "ttt", "tt", "t", "ct", "cct", "ctt"].iter().enumerate()
    {
        let c = coeffs[729 + i];
        let num = format!("{c},");
        writeln!(buf, "    {num:7}// {poly_coeff}")?;
    }

    writeln!(buf, "]")?;

    let mut file = File::create(COEFFS_FILE)?;
    file.write_all(buf.buffer())
}
