use std::collections::HashSet;

use super::is_same_player;

use crate::{
    constants::{DIRECTIONS4, DIRECTIONS8},
    model::Board,
    Model, Player,
};

const STONES_IN_A_ROW: usize = 5;
const REQUIRED_CAPTURES: usize = 5;

fn find_breakable(
    board: &Board,
    player: Player,
    new_x: isize,
    new_y: isize,
) -> HashSet<(usize, usize)> {
    let mut breaking_positions: HashSet<(usize, usize)> = HashSet::new();
    for (dx, dy) in DIRECTIONS8.iter() {
        if is_same_player(board, Player::None, new_x - dx, new_y - dy)
            && is_same_player(board, player, new_x + dx, new_y + dy)
            && is_same_player(board, player.opponent(), new_x + 2 * dx, new_y + 2 * dy)
        {
            breaking_positions.insert(((new_x - dx) as usize, (new_y - dy) as usize));
        }
        if is_same_player(board, player.opponent(), new_x - dx, new_y - dy)
            && is_same_player(board, player, new_x + dx, new_y + dy)
            && is_same_player(board, Player::None, new_x + 2 * dx, new_y + 2 * dy)
        {
            breaking_positions.insert(((new_x + 2 * dx) as usize, (new_y + 2 * dy) as usize));
        }
    }
    breaking_positions
}

fn get_longest_row_in_dir(
    board: &Board,
    player: Player,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> Vec<(usize, usize)> {
    let mut row: Vec<(usize, usize)> = Vec::new();

    row.push((x, y));

    for step in 1..STONES_IN_A_ROW as isize {
        let new_x = x as isize + step * dx;
        let new_y = y as isize + step * dy;
        if is_same_player(board, player, new_x, new_y) {
            row.push((new_x as usize, new_y as usize))
        } else {
            break;
        }
    }

    for step in (1..STONES_IN_A_ROW as isize).map(|x| -x) {
        let new_x = x as isize + step * dx;
        let new_y = y as isize + step * dy;
        if is_same_player(board, player, new_x, new_y) {
            row.push((new_x as usize, new_y as usize));
        } else {
            break;
        }
    }
    row
}

fn get_break_possibilities(
    potential_winner: &mut Vec<(usize, usize)>,
    board: &Board,
    player: Player,
) -> HashSet<(usize, usize)> {
    let mut break_possibilities: HashSet<(usize, usize)> = HashSet::new();
    potential_winner.sort();
    let overflow = potential_winner.len() - STONES_IN_A_ROW;
    for &(x, y) in &potential_winner[overflow..STONES_IN_A_ROW] {
        break_possibilities.extend(find_breakable(board, player, x as isize, y as isize));
    }
    break_possibilities
}

// fn check_five_in_a_row(
//     board: &Board,
//     player: Player,
//     x: usize,
//     y: usize,
//     dx: isize,
//     dy: isize,
//     check_unbreakable: bool,
// ) -> Result {
//     let mut count = 1;

//     // TODO: don't repeat loop. use brain

//     for step in 1..STONES_IN_A_ROW as isize {
//         let new_x = x as isize + step * dx;
//         let new_y = y as isize + step * dy;
//         if is_same_player(board, player, new_x, new_y)
//             && (!check_unbreakable || is_unbreakable(board, player, new_x, new_y))
//         {
//             count += 1;
//         } else {
//             break;
//         }
//     }

//     for step in (1..STONES_IN_A_ROW as isize).map(|x| -x) {
//         let new_x = x as isize + step * dx;
//         let new_y = y as isize + step * dy;
//         if is_same_player(board, player, new_x, new_y)
//             && (!check_unbreakable || is_unbreakable(board, player, new_x, new_y))
//         {
//             count += 1;
//         } else {
//             break;
//         }
//     }

//     count >= STONES_IN_A_ROW
//         && (check_unbreakable || check_five_in_a_row(board, player, x, y, dx, dy, true))
// }

pub fn check_winner(model: &Model, x: usize, y: usize) -> (bool, HashSet<(usize, usize)>) {
    let mut breakable_positions: HashSet<(usize, usize)> = HashSet::new();
    if model.current_player.captures(model) >= REQUIRED_CAPTURES {
        return (true, breakable_positions);
    }
    let mut potential_winners: Vec<Vec<(usize, usize)>> = Vec::new();
    for (dx, dy) in DIRECTIONS4.into_iter() {
        let longest_row_in_dir =
            get_longest_row_in_dir(&model.board, model.current_player, x, y, dx, dy);
        if longest_row_in_dir.len() > 4 {
            potential_winners.push(longest_row_in_dir)
        }
    }
    if potential_winners.is_empty() {
        return (false, breakable_positions);
    }
    for potential_winner in potential_winners.iter_mut() {
        let break_possibilities =
            get_break_possibilities(potential_winner, &model.board, model.current_player);
        if break_possibilities.is_empty() {
            return (true, breakable_positions);
        }
        if breakable_positions.is_empty() {
            breakable_positions.extend(break_possibilities);
        } else {
            breakable_positions.retain(|item| break_possibilities.contains(item));
        }
        if breakable_positions.is_empty() {
            return (true, breakable_positions);
        }
    }
    (false, breakable_positions)
}
