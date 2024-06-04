mod constants;
mod rules;

use constants::{BOARD_SIZE, CELL_SIZE, COLOR_BACKGROUND, HALF_BOARD_SIZE, WINDOW_SIZE};
use nannou::prelude::*;

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
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    Model {
        board: [[Player::None; BOARD_SIZE]; BOARD_SIZE],
        current_player: Player::Black,
        winner: Player::None,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(COLOR_BACKGROUND);

    for y in 0..BOARD_SIZE {
        let py = y as f32 * CELL_SIZE - (BOARD_SIZE as f32 * CELL_SIZE / 2.0) + CELL_SIZE / 2.0;
        for x in 0..BOARD_SIZE {
            let px = x as f32 * CELL_SIZE - (BOARD_SIZE as f32 * CELL_SIZE / 2.0) + CELL_SIZE / 2.0;

            draw.rect()
                .x_y(px, py)
                .w_h(CELL_SIZE, CELL_SIZE)
                .stroke(BLACK)
                .stroke_weight(2.0)
                .no_fill();

            if model.board[y][x] == Player::Black {
                draw.ellipse()
                    .x_y(px, py)
                    .w_h(CELL_SIZE * 0.8, CELL_SIZE * 0.8)
                    .rgb(0.0, 0.0, 0.0);
            } else if model.board[y][x] == Player::White {
                draw.ellipse()
                    .x_y(px, py)
                    .w_h(CELL_SIZE * 0.8, CELL_SIZE * 0.8)
                    .rgb(1.0, 1.0, 1.0)
                    .stroke(BLACK)
                    .stroke_weight(2.0);
            }
        }
    }

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

    if rules::check_winner(&model.board, model.current_player, x, y) {
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
