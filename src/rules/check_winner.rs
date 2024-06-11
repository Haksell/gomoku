use std::collections::HashSet;

use super::is_same_player;

use crate::{
    constants::{DIRECTIONS4, DIRECTIONS8},
    model::Board,
    Model, Turn,
};

const STONES_IN_A_ROW: usize = 5;
const REQUIRED_CAPTURES: usize = 5;

fn find_breakable(
    board: &Board,
    player: Turn,
    new_x: isize,
    new_y: isize,
) -> HashSet<(usize, usize)> {
    let mut breaking_positions: HashSet<(usize, usize)> = HashSet::new();
    for (dx, dy) in DIRECTIONS8.iter() {
        if is_same_player(board, Turn::None, new_x - dx, new_y - dy)
            && is_same_player(board, player, new_x + dx, new_y + dy)
            && is_same_player(board, player.opponent(), new_x + 2 * dx, new_y + 2 * dy)
        {
            breaking_positions.insert(((new_x - dx) as usize, (new_y - dy) as usize));
        }
        if is_same_player(board, player.opponent(), new_x - dx, new_y - dy)
            && is_same_player(board, player, new_x + dx, new_y + dy)
            && is_same_player(board, Turn::None, new_x + 2 * dx, new_y + 2 * dy)
        {
            breaking_positions.insert(((new_x + 2 * dx) as usize, (new_y + 2 * dy) as usize));
        }
    }
    breaking_positions
}

fn get_longest_row_in_dir(
    board: &Board,
    player: Turn,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> Vec<(usize, usize)> {
    let mut row: Vec<(usize, usize)> = vec![(x, y)];

    let mut advance = |reverse: bool| {
        for mut step in 1..STONES_IN_A_ROW as isize {
            if reverse {
                step = -step;
            }
            let new_x = x as isize + step * dx;
            let new_y = y as isize + step * dy;
            if is_same_player(board, player, new_x, new_y) {
                row.push((new_x as usize, new_y as usize))
            } else {
                return;
            }
        }
    };

    advance(false);
    advance(true);
    row
}

fn get_break_possibilities(
    potential_winner: &mut Vec<(usize, usize)>,
    board: &Board,
    player: Turn,
) -> HashSet<(usize, usize)> {
    let mut break_possibilities: HashSet<(usize, usize)> = HashSet::new();
    potential_winner.sort();
    let overflow = potential_winner.len() - STONES_IN_A_ROW;
    for &(x, y) in &potential_winner[overflow..STONES_IN_A_ROW] {
        break_possibilities.extend(find_breakable(board, player, x as isize, y as isize));
    }
    break_possibilities
}

pub fn check_winner(model: &Model, x: usize, y: usize) -> (bool, HashSet<(usize, usize)>) {
    let mut breakable_positions: HashSet<(usize, usize)> = HashSet::new();
    if model.current_player.captures(model) >= REQUIRED_CAPTURES {
        return (true, breakable_positions);
    }
    let mut potential_winners: Vec<Vec<(usize, usize)>> = Vec::new();
    for (dx, dy) in DIRECTIONS4.into_iter() {
        let longest_row_in_dir =
            get_longest_row_in_dir(&model.board, model.current_player, x, y, dx, dy);
        if longest_row_in_dir.len() >= STONES_IN_A_ROW {
            potential_winners.push(longest_row_in_dir)
        }
    }
    for potential_winner in potential_winners.iter_mut() {
        let break_possibilities =
            get_break_possibilities(potential_winner, &model.board, model.current_player);
        if break_possibilities.is_empty() {
            return (true, breakable_positions);
        }
        if breakable_positions.is_empty() {
            breakable_positions = break_possibilities;
        } else {
            breakable_positions.retain(|item| break_possibilities.contains(item));
        }
        if breakable_positions.is_empty() {
            return (true, breakable_positions);
        }
    }
    (false, breakable_positions)
}
