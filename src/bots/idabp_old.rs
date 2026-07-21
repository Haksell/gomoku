// TODO: overwrite values of searches at low depth with value of next depth
// Then we should be able to use best_move_at_depth, whether the search finished or not

use crate::{
    TIME_LIMIT,
    bots::{leaf_value, random_mover::random_mover},
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

pub fn idabp_old(game: &Game, heuristic: &Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let t0 = Instant::now();
    let random_move = random_mover(game, heuristic);
    let mut game = game.clone();
    let mut cache = Cache::default();
    let mut searched_depth = -1;
    let mut best_move = random_move;

    for depth in 0.. {
        let mut best_move_at_depth = random_move;
        alpha_beta_pruning_helper(
            &mut game,
            heuristic,
            0,
            depth,
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

        searched_depth = depth as i32;
        best_move = best_move_at_depth;
    }

    if game.black_player.is_human() || game.white_player.is_human() {
        println!("IDABP search depth: {searched_depth}");
    }

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
    // Only check time limit at low depth to avoid useless syscalls
    if depth <= 3 && t0.elapsed() > TIME_LIMIT {
        return min_h;
    }

    if let Some(leaf_value) = leaf_value(game, heuristic, depth, max_depth) {
        cache.insert(cache_key, leaf_value);
        return leaf_value;
    }

    let mut close_moves = game.get_legal_moves(Some(2));
    debug_assert!(!close_moves.is_empty());

    if depth + 1 < max_depth {
        let default_h = max_h / 2; // benchmarked
        close_moves.sort_by_cached_key(|&pos| {
            cache.get(&update_cache_key(cache_key, pos)).unwrap_or(&default_h)
        });
    }

    let mut best_h = i64::MIN;

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
