use crate::{
    TIME_LIMIT,
    game::{
        Game,
        board::{BOARD_CENTER, Position},
        state::GameState,
    },
    heuristics::Heuristic,
    player::PlayerColor,
};
use rand::{
    rng,
    seq::{IndexedRandom as _, SliceRandom as _},
};
use std::time::Instant;

// TODO: remove redundant fields
struct MCTSNode {
    last_position: Position,
    untried_positions: Vec<Position>,
    children: Vec<usize>,
    // score (TODO: u64, with 1 for draw and 2 for win)
    visits: u64,
    wins: f64,
}

impl MCTSNode {
    fn new(game: &Game, last_position: Position) -> Self {
        let mut untried_positions = game.get_legal_moves(None);
        untried_positions.shuffle(&mut rng());
        Self { last_position, untried_positions, children: Vec::new(), visits: 0, wins: 0. }
    }

    const fn is_fully_expanded(&self) -> bool {
        self.untried_positions.is_empty()
    }

    fn expand(&mut self, game: &mut Game, index: usize) -> Self {
        let position = self.untried_positions.pop().unwrap();
        game.do_move(position);
        let child = Self::new(game, position);
        self.children.push(index);
        child
    }

    fn best_child(&self, nodes: &[Self]) -> usize {
        const EXPLORATION: f64 = 0.2;

        *self
            .children
            .iter()
            .max_by(|&&a, &&b| {
                let score = |child: usize| {
                    let node = &nodes[child];
                    if node.visits == 0 {
                        return f64::INFINITY;
                    }

                    let exploit = 1.0 - node.wins / node.visits as f64;
                    let explore =
                        EXPLORATION * ((self.visits as f64).ln() / node.visits as f64).sqrt();
                    exploit + explore
                };

                score(a).total_cmp(&score(b))
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
        Self { game: game.clone(), nodes: vec![MCTSNode::new(game, (usize::MAX, usize::MAX))] }
    }

    fn step(&mut self) {
        let mut node_idx = 0;
        let mut visited_nodes = vec![0];

        let mut game = self.game.clone();

        while game.state.is_playing() && self.nodes[node_idx].is_fully_expanded() {
            node_idx = self.nodes[node_idx].best_child(&self.nodes); // TODO: find best constant
            let position = self.nodes[node_idx].last_position;
            game.do_move(position);
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

        self.backpropagate(winner, &visited_nodes);
    }

    fn backpropagate(&mut self, winner: Option<PlayerColor>, visited_nodes: &[usize]) {
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
        for &node_idx in visited_nodes {
            self.nodes[node_idx].visits += 1;
            self.nodes[node_idx].wins += wins;
            wins = 1. - wins;
        }
    }

    fn best_move(&self) -> Position {
        let best_idx =
            *self.nodes[0].children.iter().max_by_key(|c| self.nodes[**c].visits).unwrap();
        self.nodes[best_idx].last_position
    }
}

/// # Panics
///
/// Will panic if `TIME_LIMIT` is not set.
pub fn mcts_old(game: &Game, _: &Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let deadline = Instant::now() + *TIME_LIMIT.get().unwrap();
    let mut mcts = MCTS::new(game);
    // TODO: parallelize
    while Instant::now() < deadline {
        mcts.step();
    }
    mcts.best_move()
}
