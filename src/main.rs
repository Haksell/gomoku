use nannou::prelude::*;

const BOARD_SIZE: usize = 19;
const WINDOW_SIZE: usize = 800;
const CELL_SIZE: f32 = WINDOW_SIZE as f32 / BOARD_SIZE as f32;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    None,
    Black,
    White,
}

struct Model {
    board: [[Player; BOARD_SIZE]; BOARD_SIZE],
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
    draw.background().color(WHITE);

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let x = i as f32 * CELL_SIZE - (BOARD_SIZE as f32 * CELL_SIZE / 2.0) + CELL_SIZE / 2.0;
            let y = j as f32 * CELL_SIZE - (BOARD_SIZE as f32 * CELL_SIZE / 2.0) + CELL_SIZE / 2.0;

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
        println!("{:?} already won.", model.winner);
        return;
    }

    let mouse_pos = app.mouse.position();
    let i = ((mouse_pos.x + (BOARD_SIZE as f32 * CELL_SIZE / 2.0)) / CELL_SIZE).floor() as usize;
    let j = ((mouse_pos.y + (BOARD_SIZE as f32 * CELL_SIZE / 2.0)) / CELL_SIZE).floor() as usize;

    if i < BOARD_SIZE && j < BOARD_SIZE && model.board[i][j] == Player::None {
        model.board[i][j] = model.current_player.clone();

        if check_winner(&model.board, i, j, &model.current_player) {
            model.winner = model.current_player.clone();
        } else {
            model.current_player = if model.current_player == Player::Black {
                Player::White
            } else {
                Player::Black
            };
        }
    }
}

fn check_winner(
    board: &[[Player; BOARD_SIZE]; BOARD_SIZE],
    x: usize,
    y: usize,
    player: &Player,
) -> bool {
    let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];

    for (dx, dy) in directions.iter() {
        let mut count = 1;

        for step in 1..5 {
            let nx = x as isize + step * dx;
            let ny = y as isize + step * dy;
            if nx < 0 || ny < 0 || nx >= BOARD_SIZE as isize || ny >= BOARD_SIZE as isize {
                break;
            }
            if board[nx as usize][ny as usize] == *player {
                count += 1;
            } else {
                break;
            }
        }

        for step in 1..5 {
            let nx = x as isize - step * dx;
            let ny = y as isize - step * dy;
            if nx < 0 || ny < 0 || nx >= BOARD_SIZE as isize || ny >= BOARD_SIZE as isize {
                break;
            }
            if board[nx as usize][ny as usize] == *player {
                count += 1;
            } else {
                break;
            }
        }

        if count >= 5 {
            return true;
        }
    }

    false
}
