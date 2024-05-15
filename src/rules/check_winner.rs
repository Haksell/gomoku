use crate::{constants::SQUARES, Board, Player};

fn check_is_player(board: &Board, player: Player, x: isize, y: isize) -> bool {
    x >= 0
        && y >= 0
        && x < SQUARES as isize
        && y < SQUARES as isize
        && board[x as usize][y as usize] == player
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
        if check_is_player(
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
        if check_is_player(
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

pub fn check_winner(board: &Board, player: Player, x: usize, y: usize) -> bool {
    static DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];
    DIRECTIONS
        .into_iter()
        .any(|(dx, dy)| check_winner_in_direction(board, player, x, y, dx, dy))
}
