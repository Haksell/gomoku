use crate::{
    bots::{MAX_DEPTH, TIME_LIMIT, leaf_value},
    game::{
        Game,
        board::{BOARD_CENTER, BOARD_SIZE, Position},
    },
    heuristics::Heuristic,
};
use std::{cmp::max, time::Instant};

const BITS_PER_MOVE: u64 = u64::BITS as u64 - (BOARD_SIZE * BOARD_SIZE + 1).leading_zeros() as u64;

// TODO: u128 -> u64 if possible: (MAX_DEPTH+1) & BITS_PER_MOVE <= 64
type CacheKey = u128;

/// Benchmarked against rustc-hash, ahash and nohash-hasher.
type Cache = fxhash::FxHashMap<CacheKey, i64>;

pub fn idabp(game: &Game, heuristic: &Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let t0 = Instant::now();
    let mut cache = Cache::default();
    let mut game = game.clone();

    let mut best_move = (usize::MAX, usize::MAX);
    // let mut explored_depth = -1;
    for max_depth in 0..=MAX_DEPTH {
        let mut best_move_at_depth = (usize::MAX, usize::MAX);
        alpha_beta_pruning_helper(
            &mut game,
            heuristic,
            0,
            max_depth,
            -i64::MAX,
            i64::MAX,
            &mut best_move_at_depth,
            &mut cache,
            0,
            t0,
        );
        // TODO: break if close to time limit (predict time for next depth)
        if t0.elapsed() > TIME_LIMIT {
            // TODO: try using best_move_at_depth if possible
            break;
        }
        // explored_depth = max_depth as i32;
        best_move = best_move_at_depth;
    }
    // println!("{explored_depth}");
    best_move
}

#[expect(clippy::too_many_arguments)]
fn alpha_beta_pruning_helper(
    game: &mut Game,
    heuristic: &Heuristic,
    depth: usize,
    max_depth: usize,
    mut min_h: i64,
    max_h: i64,
    best_move: &mut Position,
    cache: &mut Cache,
    cache_key: CacheKey,
    t0: Instant,
) -> i64 {
    if t0.elapsed() > TIME_LIMIT {
        return min_h;
    }

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
            t0,
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

const fn update_cache_key(cache_key: CacheKey, (x, y): Position) -> CacheKey {
    (cache_key << BITS_PER_MOVE) | (y * BOARD_SIZE + x + 1) as CacheKey
}
