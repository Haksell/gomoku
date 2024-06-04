mod check_winner;
mod constants;

use check_winner::check_winner;
use constants::{
    BOARD_SIZE, CELL_SIZE, COLOR_BACKGROUND, DOT_SPACING, HALF_BOARD_SIZE, WINDOW_MARGIN,
    WINDOW_SIZE,
};
use nannou::prelude::*;

// TODO: player.rs
// TODO: only two players, and winner is Some(Player)
// TODO: .color()
// TODO: .opponent()
#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    None,
    Black,
    White,
}

type Board = [[Player; BOARD_SIZE]; BOARD_SIZE];

struct Model {
    board: Board,
    current_player: Player,
    winner: Player,
}

fn main() {
    nannou::app(model).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .resizable(false)
        .mouse_pressed(mouse_pressed)
        .msaa_samples(4)
        .build()
        .unwrap();

    Model {
        board: [[Player::None; BOARD_SIZE]; BOARD_SIZE],
        current_player: Player::Black,
        winner: Player::None,
    }
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

fn get_intersection_position(x: usize, y: usize) -> Point2 {
    fn physical_position(z: usize) -> f32 {
        (z as isize - HALF_BOARD_SIZE as isize) as f32 * CELL_SIZE
    }

    pt2(physical_position(x), physical_position(y))
}

fn draw_dots(draw: &Draw) {
    const DOT_SIZE: f32 = CELL_SIZE * 0.25;
    for y in -1..=1 {
        for x in -1..=1 {
            let dot = get_intersection_position(
                (HALF_BOARD_SIZE as isize + x * DOT_SPACING as isize) as usize,
                (HALF_BOARD_SIZE as isize + y * DOT_SPACING as isize) as usize,
            );
            draw.ellipse()
                .x_y(dot.x, dot.y)
                .w_h(DOT_SIZE, DOT_SIZE)
                .color(BLACK);
        }
    }
}

fn draw_stones(draw: &Draw, model: &Model) {
    const STONE_SIZE: f32 = CELL_SIZE * 0.77;

    fn draw_stone(draw: &Draw, x: usize, y: usize, color: Srgb<u8>) {
        let pos = get_intersection_position(x, y);
        draw.ellipse()
            .x_y(pos.x, pos.y)
            .w_h(STONE_SIZE, STONE_SIZE)
            .color(color)
            .stroke(BLACK)
            .stroke_weight(2.0);
    }

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            match model.board[y][x] {
                Player::None => {}
                Player::Black => draw_stone(draw, x, y, BLACK),
                Player::White => draw_stone(draw, x, y, WHITE),
            }
        }
    }
}

// TODO: view.rs
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(COLOR_BACKGROUND);
    draw_grid(&draw);
    draw_dots(&draw);
    draw_stones(&draw, model);
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    if model.winner != Player::None {
        return;
    }
    let mouse_pos = app.mouse.position();
    let x = (mouse_pos.x / CELL_SIZE).round() as isize + HALF_BOARD_SIZE as isize;
    let y = (mouse_pos.y / CELL_SIZE).round() as isize + HALF_BOARD_SIZE as isize;
    if x < 0 || y < 0 {
        return;
    }
    let (x, y) = (x as usize, y as usize);
    if x >= BOARD_SIZE || y >= BOARD_SIZE || model.board[y][x] != Player::None {
        return;
    }

    model.board[y][x] = model.current_player;

    if check_winner(&model.board, model.current_player, x, y) {
        model.winner = model.current_player;
        println!("{:?} won.", model.winner);
    } else {
        model.current_player = if model.current_player == Player::Black {
            Player::White
        } else {
            Player::Black
        };
    }
}
