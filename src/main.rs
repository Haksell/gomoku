mod constants;
mod player;
mod rules;
mod view;

use constants::{BOARD_SIZE, CELL_SIZE, HALF_BOARD_SIZE, WINDOW_SIZE};
use nannou::prelude::*;
use player::Player;
use rules::{check_winner, handle_captures};
use view::view;

type Board = [[Player; BOARD_SIZE]; BOARD_SIZE];

struct Model {
    board: Board,
    current_player: Player,
    winner: Player,
    black_captures: usize,
    white_captures: usize,
}

fn main() {
    nannou::app(model).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("ligomoku.org")
        .size(WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .resizable(false)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    Model {
        board: [[Player::None; BOARD_SIZE]; BOARD_SIZE],
        current_player: Player::Black,
        winner: Player::None,
        black_captures: 0,
        white_captures: 0,
    }
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
    // TODO: || check_double_three()
    if x >= BOARD_SIZE || y >= BOARD_SIZE || model.board[y][x] != Player::None {
        return;
    }

    model.board[y][x] = model.current_player;
    handle_captures(model, x, y);

    if check_winner(model, x, y) {
        model.winner = model.current_player;
        // model.current_player = Player::None; ???
        println!("{:?} won.", model.winner);
    } else {
        model.current_player = model.current_player.opponent();
    }
}
