mod constants;
mod rules;

use constants::{CELL_SIZE, COLOR_BACKGROUND, HALF_SQUARES, SQUARES, WINDOW_SIZE};
use nannou::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    None,
    Black,
    White,
}

type Board = [[Player; SQUARES]; SQUARES];

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
        board: [[Player::None; SQUARES]; SQUARES],
        current_player: Player::Black,
        winner: Player::None,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(COLOR_BACKGROUND);

    for i in 0..SQUARES {
        for j in 0..SQUARES {
            let x = i as f32 * CELL_SIZE - (SQUARES as f32 * CELL_SIZE / 2.0) + CELL_SIZE / 2.0;
            let y = j as f32 * CELL_SIZE - (SQUARES as f32 * CELL_SIZE / 2.0) + CELL_SIZE / 2.0;

            draw.rect()
                .x_y(x, y)
                .w_h(CELL_SIZE, CELL_SIZE)
                .stroke(BLACK)
                .stroke_weight(2.0)
                .no_fill();

            if model.board[i][j] == Player::Black {
                draw.ellipse()
                    .x_y(x, y)
                    .w_h(CELL_SIZE * 0.8, CELL_SIZE * 0.8)
                    .rgb(0.0, 0.0, 0.0);
            } else if model.board[i][j] == Player::White {
                draw.ellipse()
                    .x_y(x, y)
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
    let i = (mouse_pos.x / CELL_SIZE).round() as isize + HALF_SQUARES as isize;
    let j = (mouse_pos.y / CELL_SIZE).round() as isize + HALF_SQUARES as isize;
    if i < 0 || j < 0 {
        return;
    }
    let (i, j) = (i as usize, j as usize);
    if i >= SQUARES || j >= SQUARES || model.board[i][j] != Player::None {
        return;
    }

    model.board[i][j] = model.current_player;

    if rules::check_winner(&model.board, model.current_player, i, j) {
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
