use nannou::rand::{Rng as _, rngs::ThreadRng};
use std::array;

const POP_SIZE: usize = 100;
const EPOCHS: usize = 100;
const GENES_SIZE: usize = 729 + 9;
const MUTATION_PROBABILITY: f64 = 0.02;

#[derive(Clone)]
struct Genome {
    fitnesss: Option<i64>,
    genes: [i16; GENES_SIZE],
}

impl Genome {
    fn random(rng: &mut ThreadRng) -> Self {
        Self { fitnesss: None, genes: array::from_fn(|_| rng.r#gen()) }
    }

    fn evaluate(&mut self) {
        if self.fitnesss.is_some() {
            return;
        }
        todo!();
        let mut score = 0;
        self.fitnesss = Some(score);
    }
}

pub fn run() {
    let mut best = None;
    let mut best_score = None;
    let mut rng = nannou::rand::thread_rng();
    let mut pop: [Genome; POP_SIZE] = array::from_fn(|_| Genome::random(&mut rng));

    for _ in 0..EPOCHS {
        for player in &mut pop {
            player.evaluate();
        }
        pop.sort_unstable_by_key(|player| player.fitnesss);

        // update best
        let last = pop.last().expect("Expected player");
        if last.fitnesss > best_score {
            best_score = last.fitnesss;
            best = Some(last.clone());
        }

        // crossover
        for i in 10..POP_SIZE {
            let a = rng.gen_range(0..10);
            let b = rng.gen_range(0..10);

            let crossover_point = rng.gen_range(0..GENES_SIZE);
            pop[i].fitnesss = None;
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
                player.fitnesss = None;
            }
        }
    }

    match &best {
        Some(best) => println!("Best individual parameters: {:?}", best.genes),
        None => eprintln!("No best individual :c"),
    }
}
