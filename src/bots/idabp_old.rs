use crate::{
    game::{
        Game, GameState,
        board::{BOARD_CENTER, BOARD_SIZE, Position},
    },
    heuristics::Heuristic,
    player::PlayerColor,
};
use std::cmp::{max, min};

// TODO: different max_dist and number of best moves to check depending on depth
const MAX_DEPTH: usize = 3;

const BITS_PER_MOVE: u64 = u64::BITS as u64 - (BOARD_SIZE * BOARD_SIZE + 1).leading_zeros() as u64;

// TODO: if MAX_DEPTH > 7, u64 is too small for the key
/// Benchmarked against rustc-hash, ahash and nohash-hasher.
type Cache = fxhash::FxHashMap<u64, i64>;

pub fn idabp_old(game: &Game, heuristic: Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let mut best_move = (usize::MAX, usize::MAX);
    let mut cache = Cache::default();
    for max_depth in 0..=MAX_DEPTH {
        alpha_beta_pruning_helper(
            &mut game.clone(),
            heuristic,
            game.current_color,
            0,
            max_depth,
            i64::MIN,
            i64::MAX,
            &mut best_move,
            0,
            &mut cache,
        );
    }
    best_move
}

#[expect(clippy::too_many_arguments)]
fn alpha_beta_pruning_helper(
    game: &mut Game,
    heuristic: Heuristic,
    maximizing_player: PlayerColor,
    depth: usize,
    max_depth: usize,
    mut min_h: i64,
    mut max_h: i64,
    best_move: &mut Position,
    cache_key: u64,
    cache: &mut Cache,
) -> i64 {
    let leaf_h = match game.state {
        GameState::Playing => (depth == max_depth).then(|| match maximizing_player {
            PlayerColor::Black => heuristic(game),
            PlayerColor::White => -heuristic(game),
        }),
        GameState::Draw => Some(0),
        GameState::Won(winner) => Some(if winner == maximizing_player {
            i64::MAX - depth as i64
        } else {
            -(i64::MAX - depth as i64)
        }),
    };

    if let Some(leaf_h) = leaf_h {
        let previous = cache.insert(cache_key, leaf_h);
        debug_assert!(previous.is_none());
        return leaf_h;
    }

    let is_maximizing_player = game.current_color == maximizing_player;
    let mut best_h = if is_maximizing_player { i64::MIN } else { i64::MAX };

    // TODO: sort by depth 1 heuristic
    let mut close_moves = game.get_legal_moves(Some(2), depth == 0);
    debug_assert!(!close_moves.is_empty());

    if depth + 1 < max_depth {
        // TODO: profile min vs median vs max vs 0 vs some other lerp
        let median_h = i64::midpoint(min_h, max_h);
        close_moves.sort_by_cached_key(|&(x, y)| {
            cache.get(&update_cache_key(cache_key, x, y)).unwrap_or(&median_h)
        });
        if is_maximizing_player {
            close_moves.reverse();
        }
    }

    for (x, y) in close_moves {
        game.do_move(x, y);

        // TODO: NO ALREADY DONE
        let new_cache_key = update_cache_key(cache_key, x, y);

        let h = alpha_beta_pruning_helper(
            game,
            heuristic,
            maximizing_player,
            depth + 1,
            max_depth,
            min_h,
            max_h,
            best_move,
            new_cache_key,
            cache,
        );

        game.undo_last_move();

        if is_maximizing_player {
            best_h = max(best_h, h);
            if depth == 0 && h == best_h {
                *best_move = (x, y);
            }
            if h > max_h {
                break;
            }
            min_h = max(min_h, h);
        } else {
            best_h = min(best_h, h);
            if h < min_h {
                break;
            }
            max_h = min(max_h, h);
        }
    }

    best_h
}

const fn update_cache_key(cache_key: u64, x: usize, y: usize) -> u64 {
    (cache_key << BITS_PER_MOVE) | (y * BOARD_SIZE + x + 1) as u64
}

// Cache key with transpositions
// 13.940 new
// 13.538 old
// 3:01 new
// 3:42

// TODO: handle captures
// TODO: test
// const fn update_cache_key(mut cache_key: u64, x: usize, y: usize) -> u64 {
//     const MASK: u64 = (1 << BITS_PER_MOVE) - 1;
//     // [B].B.B
//     let new_value = (y * BOARD_SIZE + x + 1) as u64;
//     cache_key = (cache_key << BITS_PER_MOVE) | new_value;
//     let mut shift = 0;
//     loop {
//         let old_value = (cache_key >> (shift + 2 * BITS_PER_MOVE)) & MASK;
//         if new_value >= old_value {
//             break;
//         }

//         let mask_except_swapped = !(MASK << shift) & !(MASK << (shift + 2 * BITS_PER_MOVE));
//         cache_key = (cache_key & mask_except_swapped)
//             | (old_value << shift)
//             | (new_value << (shift + 2 * BITS_PER_MOVE));

//         shift += 2 * BITS_PER_MOVE;
//     }
//     cache_key
// }
