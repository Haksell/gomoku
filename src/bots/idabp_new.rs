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

const BITS_PER_MOVE: u32 = u32::BITS - ((BOARD_SIZE as u32).pow(2) + 1).leading_zeros();

type CacheKey = u128;

/// Benchmarked against rustc-hash, ahash and nohash-hasher.
type Cache = fxhash::FxHashMap<CacheKey, i64>;

/// # Panics
///
/// Will panic if `TIME_LIMIT` is not set.
#[inline]
pub fn idabp_new(game: &Game, heuristic: &Heuristic) -> Position {
    if game.ply == 0 {
        return BOARD_CENTER;
    }

    let deadline = Instant::now() + *TIME_LIMIT.get().unwrap();
    let random_move = random_mover(game, heuristic);
    let mut game = game.clone();
    let mut cache = Cache::default();
    let mut best_move = random_move;

    for max_depth in 0.. {
        alpha_beta_pruning_helper(
            &mut game,
            heuristic,
            (0, max_depth),
            (-i64::MAX, i64::MAX),
            (&mut cache, 0),
            &mut best_move,
            deadline,
        );

        if Instant::now() >= deadline {
            if game.black_player.is_human() || game.white_player.is_human() {
                println!("IDABP search depth: {}.5", max_depth - 1); // TODO: more precise
            }
            return best_move;
        }
    }

    unreachable!()
}

fn alpha_beta_pruning_helper(
    game: &mut Game,
    heuristic: &Heuristic,
    (depth, max_depth): (u32, u32),
    (mut min_h, max_h): (i64, i64),
    (cache, key): (&mut Cache, CacheKey),
    best_move: &mut Position,
    deadline: Instant,
) -> i64 {
    // Only check time limit at low depth to avoid useless syscalls
    if depth <= 3 && Instant::now() >= deadline {
        return 0;
    }

    if let Some(leaf_value) = leaf_value(game, heuristic, depth, max_depth) {
        cache.insert(key, leaf_value);
        return leaf_value;
    }

    let mut close_moves = game.get_legal_moves(Some(2));
    debug_assert!(!close_moves.is_empty());

    if depth + 1 < max_depth {
        let default_h = max_h / 2; // benchmarked
        close_moves.sort_by_cached_key(|&pos| {
            cache.get(&update_cache_key(key, depth, pos)).unwrap_or(&default_h)
        });
    }

    let mut best_h = i64::MIN;

    for pos in close_moves {
        let new_cache_key = update_cache_key(key, depth, pos); // TODO: NO (already done in sort)

        game.do_move(pos);
        let h = -alpha_beta_pruning_helper(
            game,
            heuristic,
            (depth + 1, max_depth),
            (-max_h, -min_h),
            (cache, new_cache_key),
            best_move,
            deadline,
        );
        game.undo_last_move();

        best_h = max(best_h, h);
        if depth == 0 && h == best_h && Instant::now() < deadline {
            *best_move = pos;
        }
        if best_h > max_h {
            break;
        }
        min_h = max(min_h, h);
    }

    cache.insert(key, best_h);
    best_h
}

const fn update_cache_key(key: CacheKey, depth: u32, (x, y): Position) -> CacheKey {
    let shift = depth * BITS_PER_MOVE;
    let bits_to_insert = (y * BOARD_SIZE + x + 1) as CacheKey;
    key | (bits_to_insert << shift)
}
