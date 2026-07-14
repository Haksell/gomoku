use crate::{
    bots::idabp_new::idabp_new,
    game::{Game, state::GameState},
    heuristics::coeff_heuristic::coeff_heuristic,
    player::{Player, PlayerColor},
};
use indicatif::ParallelProgressIterator as _;
use nannou::rand::{Rng as _, rngs::ThreadRng};
use rayon::iter::{IntoParallelRefMutIterator as _, ParallelIterator as _};
use std::array;

const POP_SIZE: usize = 100;
const ELITE_COUNT: usize = POP_SIZE / 10;
const TEST_GAMES: usize = 8;
const EPOCHS: usize = 1;
const GENES_SIZE: usize = 729 + 9;
const MUTATION_PROBABILITY: f64 = 0.05;

#[derive(Clone)]
struct Genome {
    fitness: Option<i64>,
    genes: [i16; GENES_SIZE],
}

impl Genome {
    const PLAYER: Player = Player::Bot { bot: idabp_new, heuristic: coeff_heuristic };

    fn random(rng: &mut ThreadRng) -> Self {
        Self { fitness: None, genes: array::from_fn(|_| rng.r#gen()) }
    }

    fn evaluate(&mut self) {
        if self.fitness.is_some() {
            return;
        }

        let mut score = 0;

        for _ in 0..TEST_GAMES / 2 {
            let mut game = Game::new(Self::PLAYER, Player::RANDOM);
            game.coeffs = Some(self.genes.map(|x| x as i64));

            let mut switched_game = game.clone();
            (switched_game.black_player, switched_game.white_player) =
                (switched_game.white_player, switched_game.black_player);

            game.play_game();
            switched_game.play_game();

            let black_win = match game.state {
                GameState::Playing(_) => unreachable!(),
                GameState::Won(PlayerColor::Black, _) => true,
                GameState::Draw | GameState::Won(PlayerColor::White, _) => false,
            };
            let white_win = match switched_game.state {
                GameState::Playing(_) => unreachable!(),
                GameState::Won(PlayerColor::White, _) => true,
                GameState::Draw | GameState::Won(PlayerColor::Black, _) => false,
            };

            score += black_win as i64 + white_win as i64; // TODO guez
        }

        self.fitness = Some(score);
    }
}

pub fn run() {
    let mut best = Option::<Genome>::None;
    let mut best_score = Option::<i64>::None;
    let mut rng = nannou::rand::thread_rng();
    let mut pop: [Genome; POP_SIZE] = array::from_fn(|_| Genome::random(&mut rng));

    for epoch in 0..EPOCHS {
        pop.par_iter_mut().progress_count(POP_SIZE as u64).for_each(Genome::evaluate);
        pop.sort_unstable_by_key(|player| player.fitness);

        // update best
        let last = pop.last().expect("Expected player");
        if last.fitness > best_score {
            best_score = last.fitness;
            best = Some(last.clone());
        }
        println!("epoch {} best_score {}/{}", epoch, best_score.unwrap_or(0), TEST_GAMES);

        // crossover
        for i in ELITE_COUNT..POP_SIZE {
            let a = rng.gen_range(0..ELITE_COUNT);
            let b = rng.gen_range(0..ELITE_COUNT);

            let crossover_point = rng.gen_range(0..GENES_SIZE);
            pop[i].fitness = None;
            for mi in 0..GENES_SIZE {
                if mi <= crossover_point {
                    pop[i].genes[mi] = pop[a].genes[mi];
                } else {
                    pop[i].genes[mi] = pop[b].genes[mi];
                }
            }
        }

        // mutate
        for player in &mut pop {
            let mut has_mutated = false;
            for i in 0..GENES_SIZE {
                if rng.gen_bool(MUTATION_PROBABILITY) {
                    player.genes[i] = rng.r#gen();
                    has_mutated = true;
                }
            }
            if has_mutated {
                player.fitness = None;
            }
        }
    }

    match &best {
        Some(best) => println!("Best individual parameters: {:?}", best.genes),
        None => eprintln!("No best individual :c"),
    }
}
