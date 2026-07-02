use crate::game::{
    Game,
    board::{DIRECTIONS4, DIRECTIONS8, is_capture, is_same_player},
};

impl Game {
    pub fn creates_double_three(&self, x: usize, y: usize) -> bool {
        DIRECTIONS8
            .iter()
            .all(|&(dx, dy)| !is_capture(&self.board, self.current_color, x, y, dx, dy))
            && DIRECTIONS4
                .iter()
                .filter(|&&(dx, dy)| {
                    self.is_open_three(x, y, dx, dy) || self.is_open_three(x, y, -dx, -dy)
                })
                .count()
                >= 2
    }

    fn is_open_three(&self, x: usize, y: usize, dx: isize, dy: isize) -> bool {
        let Self { board, current_color: player, .. } = self;

        let (x, y) = (x as isize, y as isize);
        let (xm1, ym1) = (x - dx, y - dy);
        let (xm2, ym2) = (x - 2 * dx, y - 2 * dy);
        let (xp1, yp1) = (x + dx, y + dy);
        let (xp2, yp2) = (x + 2 * dx, y + 2 * dy);
        let (xp3, yp3) = (x + 3 * dx, y + 3 * dy);
        let (xp4, yp4) = (x + 4 * dx, y + 4 * dy);

        // TODO: test if it is faster directly with booleans

        let straight_border = || {
            is_same_player(board, Some(*player), xp1, yp1)
                && is_same_player(board, Some(*player), xp2, yp2)
                && is_same_player(board, None, xp3, yp3)
                && is_same_player(board, None, xm1, ym1)
        };

        let straight_center = || {
            is_same_player(board, Some(*player), xp1, yp1)
                && is_same_player(board, Some(*player), xm1, ym1)
                && is_same_player(board, None, xp2, yp2)
                && is_same_player(board, None, xm2, ym2)
        };

        let separated_alone = || {
            is_same_player(board, Some(*player), xp2, yp2)
                && is_same_player(board, Some(*player), xp3, yp3)
                && is_same_player(board, None, xm1, ym1)
                && is_same_player(board, None, xp1, yp1)
                && is_same_player(board, None, xp4, yp4)
        };

        let separated_center = || {
            is_same_player(board, Some(*player), xm1, ym1)
                && is_same_player(board, Some(*player), xp2, yp2)
                && is_same_player(board, None, xm2, ym2)
                && is_same_player(board, None, xp1, yp1)
                && is_same_player(board, None, xp3, yp3)
        };

        let separated_border = || {
            is_same_player(board, Some(*player), xp1, yp1)
                && is_same_player(board, Some(*player), xp3, yp3)
                && is_same_player(board, None, xm1, ym1)
                && is_same_player(board, None, xp2, yp2)
                && is_same_player(board, None, xp4, yp4)
        };

        // TODO: order by most common to optimize short-circuiting
        straight_border()
            || straight_center()
            || separated_alone()
            || separated_center()
            || separated_border()
    }
}
