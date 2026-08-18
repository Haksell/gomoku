use crate::{
    TIME_LIMIT,
    game::{Game, board::Position, state::GameState},
    heuristics::Heuristic,
};
use rand::{rng, seq::IndexedRandom as _};
use std::time::Instant;

// TODO: remove redundant fields
struct MCTSNode {
    last_position: Option<Position>, // redundant?
    untried_positions: Vec<Position>,
    children: Vec<usize>,
    // score (TODO: u64, with 1 for draw and 2 for win)
    visits: u64,
    wins: f64,
}

impl MCTSNode {
    fn new(game: &Game, last_position: Option<Position>) -> Self {
        Self {
            last_position,
            untried_positions: game.get_legal_moves(None), // TODO: Some(2) ?
            children: Vec::new(),
            visits: 0,
            wins: 0.,
        }
    }

    const fn is_fully_expanded(&self) -> bool {
        self.untried_positions.is_empty()
    }

    fn expand(&mut self, game: &mut Game, index: usize) -> Self {
        let position = self.untried_positions.pop().unwrap();
        game.do_move(position);
        let child = Self::new(game, Some(position));
        self.children.push(index);
        child
    }

    // TODO: rewrite
    fn best_child(&self, c: f64, nodes: &[Self]) -> usize {
        if self.children.iter().any(|&child| nodes[child].visits == 0) {
            for &child in &self.children {
                if nodes[child].visits == 0 {
                    return child;
                }
            }
        }

        *self
            .children
            .iter()
            .max_by_key(|&&child| {
                let node = &nodes[child];
                let exploit = node.wins / (node.visits as f64);
                let explore = c * ((self.visits as f64).ln() / node.visits as f64).sqrt();
                ((exploit + explore) * 1e9) as u64
            })
            .unwrap()
    }
}

#[expect(clippy::upper_case_acronyms)]
struct MCTS {
    game: Game,
    nodes: Vec<MCTSNode>,
}

impl MCTS {
    fn new(game: &Game) -> Self {
        Self { game: game.clone(), nodes: vec![MCTSNode::new(game, None)] }
    }

    fn step(&mut self) {
        let mut node_idx = 0;
        let mut visited_nodes = vec![0];

        let mut game = self.game.clone();

        while game.state.is_playing() && self.nodes[node_idx].is_fully_expanded() {
            node_idx = self.nodes[node_idx].best_child(1.4, &self.nodes); // TODO: find best constant
            visited_nodes.push(node_idx);
        }

        if game.state.is_playing() && !self.nodes[node_idx].is_fully_expanded() {
            let index = self.nodes.len();
            let new_node = self.nodes[node_idx].expand(&mut game, index);
            self.nodes.push(new_node);
            visited_nodes.push(index);
        }

        let winner = loop {
            match game.state {
                GameState::Playing(_) => {
                    let legal_moves = game.get_legal_moves(None);
                    let random_move = *legal_moves.choose(&mut rng()).unwrap();
                    game.do_move(random_move);
                }
                GameState::Draw => break None,
                GameState::Won(player_color, _) => break Some(player_color),
            }
        };

        let mut wins = match winner {
            Some(winner) => {
                if winner == self.game.current_color {
                    1.
                } else {
                    0.
                }
            }
            None => 0.5,
        };
        for node_idx in visited_nodes {
            self.nodes[node_idx].visits += 1;
            self.nodes[node_idx].wins += wins;
            wins = 1. - wins;
        }
    }

    fn best_move(&self) -> Position {
        let best_idx =
            *self.nodes[0].children.iter().max_by_key(|c| self.nodes[**c].visits).unwrap();
        self.nodes[best_idx].last_position.unwrap()
    }
}

/// # Panics
///
/// Will panic if `TIME_LIMIT` is not set.
pub fn mcts(game: &Game, _: &Heuristic) -> Position {
    let deadline = Instant::now() + *TIME_LIMIT.get().unwrap();
    let mut mcts = MCTS::new(game);
    // TODO: parallelize
    while Instant::now() < deadline {
        mcts.step();
    }
    mcts.best_move()
}
