use crate::constants::{
    BOARD_SIZE, CELL_SIZE, COLOR_BACKGROUND, DOT_SPACING, HALF_BOARD_SIZE, WINDOW_MARGIN,
    WINDOW_SIZE,
};
use crate::model::Model;
use crate::player::Player;
use nannou::prelude::*;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(COLOR_BACKGROUND);
    draw_grid(&draw);
    draw_dots(&draw);
    draw_stones(&draw, model);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_grid(draw: &Draw) {
    const WEIGHT: f32 = 2.0;
    const LIMIT: f32 = (WINDOW_SIZE as f32 + WEIGHT) / 2.0 - WINDOW_MARGIN;

    fn draw_line(draw: &Draw, start: Point2, end: Point2) {
        draw.line()
            .start(start)
            .end(end)
            .weight(WEIGHT)
            .color(BLACK);
    }

    for i in 0..BOARD_SIZE as isize {
        let pos = (i - HALF_BOARD_SIZE as isize) as f32 * CELL_SIZE;
        draw_line(draw, pt2(pos, -LIMIT), pt2(pos, LIMIT));
        draw_line(draw, pt2(-LIMIT, pos), pt2(LIMIT, pos));
    }
}

fn board_to_physical((x, y): (usize, usize)) -> (f32, f32) {
    fn btp1d(z: usize) -> f32 {
        (z as isize - HALF_BOARD_SIZE as isize) as f32 * CELL_SIZE
    }

    (btp1d(x), btp1d(y))
}

fn draw_dots(draw: &Draw) {
    const DOT_SIZE: f32 = CELL_SIZE * 0.25;
    for y in -1..=1 {
        for x in -1..=1 {
            let (px, py) = board_to_physical((
                (HALF_BOARD_SIZE as isize + x * DOT_SPACING as isize) as usize,
                (HALF_BOARD_SIZE as isize + y * DOT_SPACING as isize) as usize,
            ));
            draw.ellipse()
                .x_y(px, py)
                .w_h(DOT_SIZE, DOT_SIZE)
                .color(BLACK);
        }
    }
}

fn draw_stones(draw: &Draw, model: &Model) {
    const STONE_SIZE: f32 = CELL_SIZE * 0.77;

    fn draw_stone(draw: &Draw, x: usize, y: usize, color: Srgb<u8>) {
        let (px, py) = board_to_physical((x, y));
        draw.ellipse()
            .x_y(px, py)
            .w_h(STONE_SIZE, STONE_SIZE)
            .color(color)
            .stroke(BLACK)
            .stroke_weight(2.0);
    }

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if model.board[y][x] != Player::None {
                draw_stone(draw, x, y, model.board[y][x].color());
            }
        }
    }
}
