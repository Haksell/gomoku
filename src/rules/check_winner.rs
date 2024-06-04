use crate::{
    constants::{BOARD_SIZE, DIRECTIONS, REQUIRED_CAPTURES},
    Board, Model, Player,
};

fn is_same_player(board: &Board, player: Player, x: isize, y: isize) -> bool {
    x >= 0
        && y >= 0
        && x < BOARD_SIZE as isize
        && y < BOARD_SIZE as isize
        && board[y as usize][x as usize] == player
}

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
