pub mod board;
pub mod creates_double_three;
pub mod handle_captures;
pub mod lines;
pub mod state;

use crate::{
    Player,
    game::{
        board::{HALF_BOARD_SIZE, MANHATTAN_TO_CENTER, SPIRALLING_POSITIONS},
        state::{ForcedMoves, GameState, REQUIRED_CAPTURES},
    },
    player::PlayerColor,
};
use board::{BOARD_SIZE, Board, Position};
use nannou::rand::{Rng as _, seq::SliceRandom as _, thread_rng};

const MAX_POSSIBLE_MOVES: usize = BOARD_SIZE * BOARD_SIZE + 4 * (REQUIRED_CAPTURES - 1);
const MAX_POSSIBLE_CAPTURES: usize = 2 * (REQUIRED_CAPTURES - 1) + 8;
const MAX_POSSIBLE_FORCED_POSITIONS: usize = 16; // TODO: find the real value

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub state: GameState,
    pub board: Board,
    pub close_moves: [[u8; BOARD_SIZE]; BOARD_SIZE],
    pub current_color: PlayerColor,
    pub black_captures: usize,
    pub white_captures: usize,
    // TODO: store forbidden moves too (double threes)
    pub black_player: Player,
    pub white_player: Player,
    pub black_dist_to_center: u64,
    pub white_dist_to_center: u64,
    // TODO: outside this struct
    pub ply: usize,
    pub moves: Vec<Position>,
    pub captures: Vec<(usize, Position, Position)>,
    pub forced_moves_history: Vec<(usize, ForcedMoves)>,
}

impl Game {
    pub fn new(black_player: Player, white_player: Player) -> Self {
        Self {
            state: GameState::init(),
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            close_moves: [[0; BOARD_SIZE]; BOARD_SIZE],
            current_color: PlayerColor::Black,
            black_captures: 0,
            white_captures: 0,
            black_player,
            white_player,
            black_dist_to_center: 0,
            white_dist_to_center: 0,
            ply: 0,
            moves: Vec::with_capacity(MAX_POSSIBLE_MOVES),
            captures: Vec::with_capacity(MAX_POSSIBLE_CAPTURES),
            forced_moves_history: Vec::with_capacity(MAX_POSSIBLE_FORCED_POSITIONS),
        }
    }

    pub fn do_move(&mut self, x: usize, y: usize) {
        debug_assert!(self.state.is_playing());
        debug_assert!(self.board[y][x].is_none());

        self.ply += 1;

        self.board[y][x] = Some(self.current_color);
        match self.current_color {
            PlayerColor::Black => self.black_dist_to_center += MANHATTAN_TO_CENTER[y][x],
            PlayerColor::White => self.white_dist_to_center += MANHATTAN_TO_CENTER[y][x],
        }
        self.update_close_moves(x, y, UpdateSign::Positive);
        self.handle_captures(x, y);

        self.state = self.update_state(x, y);

        if let GameState::Playing(forced_moves) = &self.state
            && !forced_moves.is_empty()
        {
            self.forced_moves_history.push((self.ply, forced_moves.clone()));
        }

        self.current_color = !self.current_color;
        self.moves.push((x, y));
    }

    /// Every operation from [`Self::do_move`] in reverse order.
    pub fn undo_last_move(&mut self) {
        let (x, y) = self.moves.pop().unwrap();
        self.current_color = !self.current_color;
        self.state = GameState::Playing;
        self.forced_moves_history.pop_if(|(ply, _)| *ply == self.ply);

        if let Some((ply, forced_moves)) = self.forced_moves_history.last()
            && *ply == self.ply - 1
        {
            self.forced_moves.clone_from(forced_moves);
        } else {
            self.forced_moves.clear();
        }

        // undo capture
        while self.captures.last().is_some_and(|(ply, _, _)| *ply == self.ply) {
            let (_, (x1, y1), (x2, y2)) = self.captures.pop().unwrap();

            let dist_to_center = MANHATTAN_TO_CENTER[y1][x1] + MANHATTAN_TO_CENTER[y2][x2];

            match self.current_color {
                PlayerColor::Black => {
                    self.black_captures -= 1;
                    self.white_dist_to_center += dist_to_center;
                }
                PlayerColor::White => {
                    self.white_captures -= 1;
                    self.black_dist_to_center += dist_to_center;
                }
            }

            self.update_close_moves(x1, y1, UpdateSign::Positive);
            self.update_close_moves(x2, y2, UpdateSign::Positive);
            self.board[y1][x1] = Some(!self.current_color);
            self.board[y2][x2] = Some(!self.current_color);
        }

        self.update_close_moves(x, y, UpdateSign::Negative);

        match self.current_color {
            PlayerColor::Black => self.black_dist_to_center -= MANHATTAN_TO_CENTER[y][x],
            PlayerColor::White => self.white_dist_to_center -= MANHATTAN_TO_CENTER[y][x],
        }
        self.board[y][x] = None;

        self.ply -= 1;
    }

    pub const fn current_player(&self) -> &Player {
        match self.current_color {
            PlayerColor::Black => &self.black_player,
            PlayerColor::White => &self.white_player,
        }
    }

    pub fn play_game(&mut self) {
        assert!(self.black_player.is_bot());
        assert!(self.white_player.is_bot());

        while self.state.is_playing() {
            let Player::Bot { bot, heuristic } = self.current_player() else { unreachable!() };
            let (x, y) = bot(self, *heuristic);
            self.do_move(x, y);
        }
    }

    // TODO: dynamic ajustable en tout cas
    fn update_close_moves(&mut self, x: usize, y: usize, update_sign: UpdateSign) {
        const MANHATTAN2: [(isize, isize); 13] = [
            (0, 0),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 2),
            (2, 0),
            (0, -2),
            (-2, 0),
        ];

        for (dx, dy) in &MANHATTAN2 {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= BOARD_SIZE as isize || ny < 0 || ny >= BOARD_SIZE as isize {
                continue;
            }
            self.close_moves[ny as usize][nx as usize] =
                self.close_moves[ny as usize][nx as usize].wrapping_add_signed(update_sign as i8);
        }
    }

    pub fn get_legal_moves(&self, max_dist: Option<usize>, shuffle: bool) -> Vec<Position> {
        // TODO: stop hardcoding 2
        debug_assert!(matches!(max_dist, None | Some(2)));
        if !self.forced_moves.is_empty() {
            return self.forced_moves.clone().into_iter().collect();
        }

        // TODO: preallocate with number of close moves (or forced moves)
        let mut legal_moves = Vec::new();
        for (x, y) in SPIRALLING_POSITIONS {
            if (max_dist.is_none() || self.close_moves[y][x] > 0)
                && self.board[y][x].is_none()
                && !self.creates_double_three(x, y)
            {
                legal_moves.push((x, y));
            }
        }

        if shuffle {
            let mut rng = thread_rng();
            legal_moves.shuffle(&mut rng);
        }

        legal_moves
    }

    pub fn play_random_moves(&mut self, n_moves: u32, dist_to_center: usize) {
        assert!(n_moves <= BOARD_SIZE as u32);

        let mut rng = thread_rng();
        for _ in 0..n_moves {
            loop {
                let rx = rng
                    .gen_range(HALF_BOARD_SIZE - dist_to_center..=HALF_BOARD_SIZE + dist_to_center);
                let ry = rng
                    .gen_range(HALF_BOARD_SIZE - dist_to_center..=HALF_BOARD_SIZE + dist_to_center);
                if MANHATTAN_TO_CENTER[ry][rx] as usize <= dist_to_center
                    && self.board[ry][rx].is_none()
                {
                    self.do_move(rx, ry);
                    break;
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
#[repr(i8)]
pub enum UpdateSign {
    Positive = 1,
    Negative = -1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_size() {
        assert!(BOARD_SIZE % 2 == 1);
        assert!(BOARD_SIZE >= 3);
    }

    // TODO: seed
    #[test]
    fn test_undo_last_move() {
        for _ in 0..10 {
            let mut game = Game::new(Player::RANDOM, Player::RANDOM);
            let mut game_states = Vec::new();

            while game.state.is_playing() {
                game_states.push(game.clone());
                let Player::Bot { bot, heuristic } = game.current_player() else { unreachable!() };
                let (x, y) = bot(&game, *heuristic);
                game.do_move(x, y);
            }

            while let Some(game_state) = game_states.pop() {
                game.undo_last_move();
                assert_eq!(game_state, game);
            }
        }
    }
}
