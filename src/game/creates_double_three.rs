use crate::game::{
    Game,
    board::{DIRECTIONS4, DIRECTIONS8, Direction, Position, is_capture, is_same_color},
};

impl Game {
    pub fn creates_double_three(&self, pos: Position) -> bool {
        DIRECTIONS8.iter().all(|&dir| !is_capture(&self.board, self.current_color, pos, dir))
            && DIRECTIONS4
                .iter()
                .filter(|&&(dx, dy)| {
                    self.is_open_three(pos, (dx, dy)) || self.is_open_three(pos, (-dx, -dy))
                })
                .count()
                >= 2
    }

    fn is_open_three(&self, (x, y): Position, (dx, dy): Direction) -> bool {
        let Self { board, current_color, .. } = self;

        let (x, y) = (x as isize, y as isize);
        let m1 = (x - dx, y - dy);
        let m2 = (x - 2 * dx, y - 2 * dy);
        let p1 = (x + dx, y + dy);
        let p2 = (x + 2 * dx, y + 2 * dy);
        let p3 = (x + 3 * dx, y + 3 * dy);
        let p4 = (x + 4 * dx, y + 4 * dy);

        // TODO: test if it is faster directly with booleans

        let straight_border = || {
            is_same_color(board, Some(*current_color), p1)
                && is_same_color(board, Some(*current_color), p2)
                && is_same_color(board, None, p3)
                && is_same_color(board, None, m1)
        };

        let straight_center = || {
            is_same_color(board, Some(*current_color), p1)
                && is_same_color(board, Some(*current_color), m1)
                && is_same_color(board, None, p2)
                && is_same_color(board, None, m2)
        };

        let separated_alone = || {
            is_same_color(board, Some(*current_color), p2)
                && is_same_color(board, Some(*current_color), p3)
                && is_same_color(board, None, m1)
                && is_same_color(board, None, p1)
                && is_same_color(board, None, p4)
        };

        let separated_center = || {
            is_same_color(board, Some(*current_color), m1)
                && is_same_color(board, Some(*current_color), p2)
                && is_same_color(board, None, m2)
                && is_same_color(board, None, p1)
                && is_same_color(board, None, p3)
        };

        let separated_border = || {
            is_same_color(board, Some(*current_color), p1)
                && is_same_color(board, Some(*current_color), p3)
                && is_same_color(board, None, m1)
                && is_same_color(board, None, p2)
                && is_same_color(board, None, p4)
        };

        // TODO: order by most common to optimize short-circuiting
        straight_border()
            || straight_center()
            || separated_alone()
            || separated_center()
            || separated_border()
    }
}
