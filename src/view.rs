use crate::constants::{
    BOARD_SIZE, CELL_SIZE, DOT_SPACING, HALF_BOARD_SIZE, WINDOW_MARGIN, WINDOW_SIZE,
};
use crate::coordinates::board_to_physical;
use crate::model::Model;
use crate::player::Player;
use crate::rules::creates_double_three;
use crate::textures::TEXTURE_BACKGROUND;
use nannou::color::{Srgb, BLACK};
use nannou::geom::{pt2, Point2};
use nannou::wgpu::Texture;
use nannou::{App, Draw, Frame};

const STROKE_WEIGHT: f32 = WINDOW_SIZE as f32 * 0.0022;
const STONE_SIZE: f32 = CELL_SIZE * 0.77;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw_background(&draw);
    draw_grid(&draw);
    draw_dots(&draw);
    draw_stones(&draw, model);
    if model.winner == Player::None {
        if model.forced_moves.is_empty() {
            draw_invalid_moves(&draw, model);
        } else {
            draw_valid_moves(&draw, model);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn draw_background(draw: &Draw) {
    let background_texture = TEXTURE_BACKGROUND.get().unwrap().lock().unwrap();
    draw.texture(&*background_texture)
        .w_h(WINDOW_SIZE as f32, WINDOW_SIZE as f32);
}

fn draw_grid(draw: &Draw) {
    const LIMIT: f32 = (WINDOW_SIZE as f32 + STROKE_WEIGHT) / 2.0 - WINDOW_MARGIN;

    fn draw_line(draw: &Draw, start: Point2, end: Point2) {
        draw.line()
            .start(start)
            .end(end)
            .weight(STROKE_WEIGHT)
            .color(BLACK);
    }

    for i in 0..BOARD_SIZE as isize {
        let pos = (i - HALF_BOARD_SIZE as isize) as f32 * CELL_SIZE;
        draw_line(draw, pt2(pos, -LIMIT), pt2(pos, LIMIT));
        draw_line(draw, pt2(-LIMIT, pos), pt2(LIMIT, pos));
    }
}

fn draw_dots(draw: &Draw) {
    const DOT_SIZE: f32 = CELL_SIZE * 0.24;
    for y in -1..=1 {
        for x in -1..=1 {
            let (px, py) = board_to_physical(
                (HALF_BOARD_SIZE as isize + x * DOT_SPACING as isize) as usize,
                (HALF_BOARD_SIZE as isize + y * DOT_SPACING as isize) as usize,
            );
            draw.ellipse()
                .x_y(px, py)
                .w_h(DOT_SIZE, DOT_SIZE)
                .color(BLACK);
        }
    }
}

fn draw_stones(draw: &Draw, model: &Model) {
    fn draw_stone(draw: &Draw, x: usize, y: usize, texture: &Texture) {
        let (px, py) = board_to_physical(x, y);
        draw.texture(texture)
            .x_y(px, py)
            .w_h(STONE_SIZE, STONE_SIZE);
    }

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if model.board[y][x] != Player::None {
                draw_stone(draw, x, y, &model.board[y][x].texture());
            }
        }
    }
    if let Some((x, y)) = model.hover {
        draw_stone(draw, x, y, &model.current_player.texture());
    }
}

fn draw_circle(draw: &Draw, x: usize, y: usize, color: Srgb<u8>) {
    let (px, py) = board_to_physical(x, y);
    draw.ellipse()
        .x_y(px, py)
        .w_h(STONE_SIZE, STONE_SIZE)
        .color(color);
}

fn draw_valid_moves(draw: &Draw, model: &Model) {
    // Tailwind green-500
    const COLOR_VALID_MOVE: Srgb<u8> = Srgb {
        red: 0x22,
        green: 0xc5,
        blue: 0x5e,
        standard: core::marker::PhantomData,
    };

    for &(x, y) in model.forced_moves.iter() {
        if Some((x, y)) != model.hover {
            draw_circle(draw, x, y, COLOR_VALID_MOVE);
        }
    }
}

fn draw_invalid_moves(draw: &Draw, model: &Model) {
    // Tailwind red-500
    const COLOR_INVALID_MOVE: Srgb<u8> = Srgb {
        red: 0xef,
        green: 0x44,
        blue: 0x44,
        standard: core::marker::PhantomData,
    };

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if model.board[y][x] == Player::None
                && creates_double_three(&model.board, model.current_player, x, y)
            {
                draw_circle(draw, x, y, COLOR_INVALID_MOVE);
            }
        }
    }
}
