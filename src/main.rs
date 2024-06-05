mod constants;
mod player;
mod rules;
mod view;

use constants::{BOARD_SIZE, CELL_SIZE, HALF_BOARD_SIZE, WINDOW_SIZE};
use nannou::prelude::*;
use player::Player;
use rules::{check_double_three, check_winner, handle_captures};
use view::view;

type Board = [[Player; BOARD_SIZE]; BOARD_SIZE];

struct Model {
    board: Board,
    current_player: Player,
    winner: Player,
    black_captures: usize,
    white_captures: usize,
    last_move: Option<(usize, usize)>,
}

fn main() {
    nannou::app(model).view(view).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("ligomoku.org")
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .resizable(false)
        .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        board: [[Player::None; BOARD_SIZE]; BOARD_SIZE],
        current_player: Player::Black,
        winner: Player::None,
        black_captures: 0,
        white_captures: 0,
        last_move: None,
    }
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if button != MouseButton::Left {
        return;
    }
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
    if x >= BOARD_SIZE
        || y >= BOARD_SIZE
        || model.board[y][x] != Player::None
        || check_double_three(&model.board, model.current_player, x, y)
    {
        return;
    }

    model.board[y][x] = model.current_player;
    model.last_move = Some((x, y));
    handle_captures(model, x, y);

    if check_winner(model, x, y) {
        model.winner = model.current_player;
        // model.current_player = Player::None; ???
        println!("{:?} won.", model.winner);
    } else {
        model.current_player = model.current_player.opponent();
    }
}

//TODO handle if there was a capture
fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    if key == Key::Back {
        if let Some((x, y)) = model.last_move {
            model.board[y][x] = Player::None;
            model.current_player = model.current_player.opponent();
        }
    }
}
