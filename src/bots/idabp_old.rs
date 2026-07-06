use crate::{
    bots::{MAX_DEPTH, leaf_value},
    game::{
        Game,
        board::{BOARD_CENTER, BOARD_SIZE, Position},
    },
    heuristics::Heuristic,
};
use std::cmp::max;

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
    let mut game = game.clone();
    for max_depth in 0..=MAX_DEPTH {
        alpha_beta_pruning_helper(
            &mut game,
            heuristic,
            0,
            max_depth,
            -i64::MAX,
            i64::MAX,
            &mut best_move,
            &mut cache,
            0,
        );
    }
    best_move
}

#[expect(clippy::too_many_arguments)]
fn alpha_beta_pruning_helper(
    game: &mut Game,
    heuristic: Heuristic,
    depth: usize,
    max_depth: usize,
    mut min_h: i64,
    max_h: i64,
    best_move: &mut Position,
    cache: &mut Cache,
    cache_key: u64,
) -> i64 {
    if let Some(leaf_value) = leaf_value(game, heuristic, depth, max_depth) {
        let previous = cache.insert(cache_key, leaf_value);
        debug_assert!(previous.is_none());
        return leaf_value;
    }

    let mut close_moves = game.get_legal_moves(Some(2), depth == 0);
    debug_assert!(!close_moves.is_empty());

    let mut best_h = i64::MIN;

    if depth + 1 < max_depth {
        let default_h = max_h / 2; // benchmarked
        close_moves.sort_by_cached_key(|&pos| {
            cache.get(&update_cache_key(cache_key, pos)).unwrap_or(&default_h)
        });
    }

    for pos in close_moves {
        game.do_move(pos);
        let h = -alpha_beta_pruning_helper(
            game,
            heuristic,
            depth + 1,
            max_depth,
            -max_h,
            -min_h,
            best_move,
            cache,
            update_cache_key(cache_key, pos), // TODO: NO ALREADY DONE in sort
        );
        game.undo_last_move();

        best_h = max(best_h, h);
        if depth == 0 && h == best_h {
            *best_move = pos;
        }
        if best_h > max_h {
            break;
        }
        min_h = max(min_h, h);
    }

    best_h
}

const fn update_cache_key(cache_key: u64, (x, y): Position) -> u64 {
    (cache_key << BITS_PER_MOVE) | (y * BOARD_SIZE + x + 1) as u64
}

// Cache key with transpositions
// 13.940 new
// 13.538 old
// 3:01 new
// 3:42

// TODO: handle captures
// TODO: test
// const fn update_cache_key(mut cache_key: u64, (x, y): Position) -> u64 {
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
