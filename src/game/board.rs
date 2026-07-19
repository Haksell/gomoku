use crate::player::PlayerColor;

pub type Board = [[Option<PlayerColor>; BOARD_SIZE]; BOARD_SIZE];
pub type Position = (usize, usize);
pub type Direction = (isize, isize);

pub const BOARD_SIZE: usize = 19;
pub const HALF_BOARD_SIZE: usize = BOARD_SIZE / 2;
pub const BOARD_CENTER: Position = (HALF_BOARD_SIZE, HALF_BOARD_SIZE);

pub const DIRECTIONS4: [Direction; 4] = [(0, 1), (1, 1), (1, 0), (1, -1)];
pub const DIRECTIONS8: [Direction; 8] =
    [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

pub const MANHATTAN_TO_CENTER: [[u64; BOARD_SIZE]; BOARD_SIZE] = {
    let mut out = [[0; BOARD_SIZE]; BOARD_SIZE];
    let mut y = 0;
    while y < BOARD_SIZE {
        let dy = usize::abs_diff(y, HALF_BOARD_SIZE);
        let mut x = 0;
        while x < BOARD_SIZE {
            let dx = usize::abs_diff(x, HALF_BOARD_SIZE);
            out[y][x] = (dx + dy) as u64;
            x += 1;
        }
        y += 1;
    }
    out
};

#[expect(unused)]
pub fn print_board(board: &Board) {
    for row in board {
        for player_color in row {
            print!(
                "{}",
                match player_color {
                    Some(PlayerColor::Black) => 'B',
                    Some(PlayerColor::White) => 'W',
                    None => '.',
                }
            );
        }
        println!();
    }
}

pub fn is_same_color(board: &Board, player: Option<PlayerColor>, (x, y): (isize, isize)) -> bool {
    x >= 0
        && y >= 0
        && x < BOARD_SIZE as isize
        && y < BOARD_SIZE as isize
        && board[y as usize][x as usize] == player
}

pub fn is_capture(
    board: &Board,
    player_color: PlayerColor,
    (x, y): Position,
    (dx, dy): Direction,
) -> bool {
    let (x, y) = (x as isize, y as isize);
    is_same_color(board, Some(player_color), (x + 3 * dx, y + 3 * dy))
        && is_same_color(board, Some(!player_color), (x + 2 * dx, y + 2 * dy))
        && is_same_color(board, Some(!player_color), (x + dx, y + dy))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions() {
        assert_eq!(DIRECTIONS8[..4], DIRECTIONS4);
        assert!(
            DIRECTIONS8[4..]
                .iter()
                .zip(DIRECTIONS4)
                .all(|(&(x1, y1), (x2, y2))| x1 == -x2 && y1 == -y2)
        );
    }
}
