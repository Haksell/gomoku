use crate::{
    bots::idabp_new::idabp_new,
    game::{Game, state::GameState},
    heuristics::{Heuristic, coeff_heuristic::coeff_heuristic},
    player::{Player, PlayerColor},
};
use nannou::rand::{Rng as _, rngs::ThreadRng, thread_rng};
use rayon::{
    ThreadPoolBuilder,
    iter::{IntoParallelIterator as _, ParallelIterator as _},
};
use std::{
    cmp::{max, min},
    fs::File,
    io::{self, BufWriter, Write as _},
    sync::{Arc, Mutex},
    thread::available_parallelism,
};

const COEFFS_FILE: &str = "./weights/duel.rs";

const N_COEFFS: usize = 729 + 9;
const EPOCHS: usize = 1 << 20;
const N_MUTATIONS: i64 = 8;
const MAX_ADDITIVE_MUTATION: i64 = 8;
const MAX_MULTIPLICATIVE_MUTATION: f64 = 1.1;

#[expect(clippy::too_many_lines)]
pub fn run(num_threads: Option<usize>) {
    let initial_coeffs = include!("../weights/duel.rs");

    let coeffs = Arc::new(Mutex::new(initial_coeffs));
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

    (0..EPOCHS).into_par_iter().for_each(|_| {
        let mut rng = thread_rng();
        let old_player = Player::Bot {
            bot: idabp_new,
            heuristic: Heuristic { fun: coeff_heuristic, coeffs: Some(*coeffs.lock().unwrap()) },
        };

        let mut new_coeffs = *coeffs.lock().unwrap();
        for _ in 0..N_MUTATIONS {
            let i = rng.gen_range(0..N_COEFFS);
            let div_value = (new_coeffs[i] as f64 / MAX_MULTIPLICATIVE_MUTATION).round() as i64;
            let mul_value = (new_coeffs[i] as f64 * MAX_MULTIPLICATIVE_MUTATION).round() as i64;
            let min_range = min(new_coeffs[i] - MAX_ADDITIVE_MUTATION, min(div_value, mul_value));
            let max_range = max(new_coeffs[i] + MAX_ADDITIVE_MUTATION, max(div_value, mul_value));
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

        let mut stats = stats.lock().unwrap();
        if new_wins == 2 {
            stats.0 += 1;
            println!("Updated! ({} updates in {} epochs)", stats.0, stats.1);
            *coeffs.lock().unwrap() = new_coeffs;
            match write_coeffs(&new_coeffs) {
                Ok(()) => println!("coeffs written to file {COEFFS_FILE}"),
                Err(err) => eprintln!("Failed to write coeffs to file {COEFFS_FILE}: `{err}`"),
            }
        }
        stats.1 += 1;
        let epoch = stats.1;
        drop(stats);

        if epoch.is_multiple_of(100) {
            let genetic_player = Player::Bot {
                bot: idabp_new,
                heuristic: Heuristic {
                    fun: coeff_heuristic,
                    coeffs: Some(*coeffs.lock().unwrap()),
                },
            };
            let (opponent, opponent_name) = if epoch.is_multiple_of(200) {
                (
                    Player::Bot {
                        bot: idabp_new,
                        heuristic: Heuristic { fun: coeff_heuristic, coeffs: Some(initial_coeffs) },
                    },
                    "initial",
                )
            } else {
                (Player::NEW, "manual")
            };
            let mut total_wins = 0;
            for _ in 0..10 {
                let wins = play_two_games(opponent, genetic_player, &mut rng);
                total_wins += wins;
            }
            println!("{}", "=".repeat(80));
            println!("Current won {total_wins}/20 games against {opponent_name} bot");
            println!("{}", "=".repeat(80));
        }
    });
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
