use super::is_same_player;

use crate::{
    constants::{DIRECTIONS, REQUIRED_CAPTURES},
    Board, Model, Player,
};

fn check_five_in_a_row(
    board: &Board,
    player: Player,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> bool {
    let mut count = 1;

    for step in 1..5 {
        if is_same_player(
            board,
            player,
            x as isize + step * dx,
            y as isize + step * dy,
        ) {
            count += 1;
        } else {
            break;
        }
    }

    for step in 1..5 {
        if is_same_player(
            board,
            player,
            x as isize - step * dx,
            y as isize - step * dy,
        ) {
            count += 1;
        } else {
            break;
        }
    }

    count >= 5
}

pub fn check_winner(model: &Model, x: usize, y: usize) -> bool {
    model.current_player.captures(model) >= REQUIRED_CAPTURES
        || DIRECTIONS
            .into_iter()
            .any(|(dx, dy)| check_five_in_a_row(&model.board, model.current_player, x, y, dx, dy))
}
