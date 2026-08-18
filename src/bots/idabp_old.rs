use crate::{
    TIME_LIMIT,
    bots::{leaf_value, random_mover::random_mover},
    game::{
        Game,
        bitboard::BitBoard,
        board::{BOARD_CENTER, Position},
    },
    heuristics::Heuristic,
};
use std::{
    cmp::{max, min},
    time::Instant,
};

enum NodeType {
    Cut,
    All,
    PV,
}

struct CacheValue {
    depth: u32,
    max_depth: u32,
    value: i64,
    node_type: NodeType,
}

/// Benchmarked against rustc-hash, ahash and nohash-hasher.
type Cache = fxhash::FxHashMap<BitBoard, CacheValue>;

/// # Panics
///
/// Will panic if `TIME_LIMIT` is not set.
pub fn idabp_old(game: &Game, heuristic: &Heuristic) -> Position {
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
            &mut cache,
            &mut best_move,
            deadline,
        );

        if Instant::now() >= deadline {
            if game.black_player.is_human() || game.white_player.is_human() {
                println!("NEW IDABP search depth: {}.5", max_depth - 1); // TODO: more precise
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
    (mut alpha, mut beta): (i64, i64),
    cache: &mut Cache,
    best_move: &mut Position,
    deadline: Instant,
) -> i64 {
    // Only check time limit at low depth to avoid useless syscalls
    if depth <= 3 && Instant::now() >= deadline {
        return 0;
    }

    if let Some(cache_value) = cache.get(&game.bitboard)
        && cache_value.depth == depth
        && cache_value.max_depth == max_depth
    {
        let v = cache_value.value;
        match cache_value.node_type {
            NodeType::Cut => {
                if v >= beta {
                    return v;
                }
                alpha = max(alpha, v);
            }
            NodeType::All => {
                if v <= alpha {
                    return v;
                }
                beta = min(beta, v);
            }
            NodeType::PV => return v,
        }
    }

    if let Some(leaf_value) = leaf_value(game, heuristic, depth, max_depth) {
        let node_type = if leaf_value > beta {
            NodeType::Cut
        } else if leaf_value > alpha {
            NodeType::PV
        } else {
            NodeType::All
        };
        cache.insert(game.bitboard, CacheValue { depth, max_depth, value: leaf_value, node_type });
        return leaf_value;
    }

    let mut close_moves = game.get_legal_moves(Some(2));
    debug_assert!(!close_moves.is_empty());

    if depth + 1 < max_depth {
        let default_h = beta / 2; // benchmarked
        close_moves.sort_by_cached_key(|&pos| {
            game.do_move(pos);
            let cache_value = cache.get(&game.bitboard);
            game.undo_last_move();
            cache_value.map_or(default_h, |value| value.value)
        });
    }

    let mut best_h = i64::MIN;
    let mut node_type = NodeType::All;

    for pos in close_moves {
        game.do_move(pos);
        let h = -alpha_beta_pruning_helper(
            game,
            heuristic,
            (depth + 1, max_depth),
            (-beta, -alpha),
            cache,
            best_move,
            deadline,
        );
        game.undo_last_move();

        best_h = max(best_h, h);
        if depth == 0 && h == best_h && Instant::now() < deadline {
            *best_move = pos;
        }
        if best_h > beta {
            node_type = NodeType::Cut;
            break;
        }
        if best_h > alpha {
            node_type = NodeType::PV;
        }
        alpha = max(alpha, h);
    }

    cache.insert(game.bitboard, CacheValue { depth, max_depth, value: best_h, node_type });
    best_h
}
