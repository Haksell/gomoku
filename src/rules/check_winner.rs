use super::is_same_player;

use crate::{
    constants::{DIRECTIONS4, DIRECTIONS8, REQUIRED_CAPTURES, STONES_IN_A_ROW},
    model::Board,
    Model, Player,
};

fn is_unbreakable(board: &Board, player: Player, new_x: isize, new_y: isize) -> bool {
    !DIRECTIONS8.iter().any(|(dx, dy)| {
        (is_same_player(board, Player::None, new_x - dx, new_y - dy)
            && is_same_player(board, player, new_x + dx, new_y + dy)
            && is_same_player(board, player.opponent(), new_x + 2 * dx, new_y + 2 * dy))
            || (is_same_player(board, player.opponent(), new_x - dx, new_y - dy)
                && is_same_player(board, player, new_x + dx, new_y + dy)
                && is_same_player(board, Player::None, new_x + 2 * dx, new_y + 2 * dy))
    })
}

fn check_five_in_a_row(
    board: &Board,
    player: Player,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    check_unbreakable: bool,
) -> bool {
    let mut count = 1;

    // TODO: don't repeat loop. use brain

    for step in 1..STONES_IN_A_ROW as isize {
        let new_x = x as isize + step * dx;
        let new_y = y as isize + step * dy;
        if is_same_player(board, player, new_x, new_y)
            && (!check_unbreakable || is_unbreakable(board, player, new_x, new_y))
        {
            count += 1;
        } else {
            break;
        }
    }

    for step in (1..STONES_IN_A_ROW as isize).map(|x| -x) {
        let new_x = x as isize + step * dx;
        let new_y = y as isize + step * dy;
        if is_same_player(board, player, new_x, new_y)
            && (!check_unbreakable || is_unbreakable(board, player, new_x, new_y))
        {
            count += 1;
        } else {
            break;
        }
    }

    count >= STONES_IN_A_ROW
        && (check_unbreakable || check_five_in_a_row(board, player, x, y, dx, dy, true))
}

pub fn check_winner(model: &Model, x: usize, y: usize) -> bool {
    model.current_player.captures(model) >= REQUIRED_CAPTURES
        || DIRECTIONS4.into_iter().any(|(dx, dy)| {
            check_five_in_a_row(&model.board, model.current_player, x, y, dx, dy, false)
        })
}
