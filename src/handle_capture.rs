use crate::{constants::BOARD_SIZE, Board, Player};

fn is_same_player(board: &Board, player: Player, x: isize, y: isize) -> bool {
    x >= 0
        && y >= 0
        && x < BOARD_SIZE as isize
        && y < BOARD_SIZE as isize
        && board[y as usize][x as usize] == player
}

fn check_winner_in_direction(
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

pub fn handle_capture(board: &Board, player: Player, x: usize, y: usize) {
    static DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];
    DIRECTIONS
        .into_iter()
        .any(|(dx, dy)| check_winner_in_direction(board, player, x, y, dx, dy));
}
