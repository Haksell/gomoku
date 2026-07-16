use crate::{
    bots::idabp::idabp,
    game::{Game, state::GameState},
    heuristics::{Heuristic, duelistic::duelistic},
    player::{Player, PlayerColor},
};
use indicatif::ParallelProgressIterator as _;
use nannou::rand::{Rng as _, rngs::ThreadRng};
use rayon::iter::{IntoParallelRefMutIterator as _, ParallelIterator as _};
use std::array;

const POP_SIZE: usize = 50;
const ELITE_COUNT: usize = POP_SIZE / 10;
const TEST_GAMES: usize = 8;
const EPOCHS: usize = 25;
const GENES_SIZE: usize = 729 + 9;
const MUTATION_PROBABILITY: f64 = 0.03;
const CROSSOVER_PROBABILITY: f64 = 0.3;

#[derive(Clone)]
struct Genome {
    fitness: Option<i64>,
    genes: [i16; GENES_SIZE],
}

impl Genome {
    fn random(rng: &mut ThreadRng) -> Self {
        Self { fitness: None, genes: array::from_fn(|_| rng.gen_range(-2048..=2048)) }
    }

    fn as_player(&self) -> Player {
        Player::Bot {
            bot: idabp,
            heuristic: Heuristic {
                fun: duelistic,
                coeffs: Some(self.genes.map(|x| x as i64)),
            },
        }
    }

    fn evaluate(&mut self) {
        if self.fitness.is_some() {
            return;
        }

        let mut score = 0;
        let player = self.as_player();

        for i in 0..TEST_GAMES / 2 {
            let mut game = Game::new(&player, &Player::MANUAL);

            let random_moves = 3 + (i & 1) as u32;
            game.play_random_moves(random_moves, 5);

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
        pop.par_iter_mut().progress().for_each(Genome::evaluate);
        pop.sort_unstable_by_key(|player| player.fitness.unwrap());

        // update best
        let last = pop.last().expect("Expected player");
        if last.fitness > best_score {
            best_score = last.fitness;
            if let Some(score) = best_score
                && score > 3
            {
                println!("{:?}", last.genes.clone());
            }
            best = Some(last.clone());
        }
        println!(
            "epoch {} best_score {}/{} total score {}/{}",
            epoch,
            best_score.unwrap_or(0),
            TEST_GAMES,
            pop.iter().map(|player| player.fitness.unwrap()).sum::<i64>(),
            8 * POP_SIZE
        );

        // crossover
        for i in 0..POP_SIZE - ELITE_COUNT {
            let a = rng.gen_range(POP_SIZE - ELITE_COUNT - 1..POP_SIZE);
            let b = rng.gen_range(POP_SIZE - ELITE_COUNT - 1..POP_SIZE);

            pop[i].fitness = None;
            for mi in 0..GENES_SIZE {
                if rng.gen_bool(CROSSOVER_PROBABILITY) {
                    pop[i].genes[mi] = i16::midpoint(pop[a].genes[mi], pop[b].genes[mi]);
                }
            }
        }

        // mutate
        #[expect(clippy::needless_range_loop)]
        for i in 0..POP_SIZE - ELITE_COUNT {
            let player = &mut pop[i];
            let mut has_mutated = false;
            for mi in 0..GENES_SIZE {
                if rng.gen_bool(MUTATION_PROBABILITY) {
                    player.genes[mi] += rng.gen_range(-64..=64);
                    has_mutated = true;
                }
            }
            if has_mutated {
                player.fitness = None;
            }
        }
    }

    match &best {
        Some(best) => {
            println!("Best individual parameters: {:?} (score {:?})", best.genes, best_score);
        }
        None => eprintln!("No best individual :c"),
    }
}
