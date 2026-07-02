pub mod check_winner;
pub mod creates_double_three;
pub mod handle_captures;

pub const DIRECTIONS4: [(isize, isize); 4] = [(0, 1), (1, 1), (1, 0), (1, -1)];
pub const DIRECTIONS8: [(isize, isize); 8] =
    [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions() {
        assert!(DIRECTIONS8[..4] == DIRECTIONS4);
        assert!(
            DIRECTIONS8[4..]
                .iter()
                .zip(DIRECTIONS4)
                .all(|(&(x1, y1), (x2, y2))| x1 == -x2 && y1 == -y2)
        );
    }
}
