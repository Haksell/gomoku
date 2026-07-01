use crate::{
    constants::{BOARD_SIZE, CELL_SIZE, DOT_SPACING, HALF_BOARD_SIZE, WINDOW_MARGIN, WINDOW_SIZE},
    coordinates::board_to_physical,
    model::Model,
    rules::creates_double_three,
    textures::TEXTURE_BACKGROUND,
    turn::Turn,
};
use nannou::{
    App, Draw, Frame,
    color::{BLACK, LinSrgba, Srgb},
    geom::{Point2, pt2},
};

const DOT_SIZE: f32 = CELL_SIZE * 0.25;
const LINE_WIDTH: f32 = CELL_SIZE * 0.052;
const STONE_SIZE: f32 = CELL_SIZE * 0.77;

#[expect(clippy::needless_pass_by_value)]
pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw_background(&draw);
    draw_grid(&draw);
    draw_dots(&draw);
    draw_stones(&draw, model);
    if model.winner == Turn::None {
        if model.forced_moves.is_empty() {
            draw_invalid_moves(&draw, model);
        } else {
            draw_valid_moves(&draw, model);
        }
    } else {
        draw_game_over_overlay(&draw, model);
    }
    draw_hover_coords(&draw, model);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_background(draw: &Draw) {
    let background_texture = TEXTURE_BACKGROUND.get().unwrap().lock().unwrap();
    draw.texture(&*background_texture).w_h(WINDOW_SIZE as f32, WINDOW_SIZE as f32);
}

fn draw_grid(draw: &Draw) {
    const LIMIT: f32 = f32::midpoint(WINDOW_SIZE as f32, LINE_WIDTH) - WINDOW_MARGIN;

    fn draw_line(draw: &Draw, start: Point2, end: Point2) {
        draw.line().start(start).end(end).weight(LINE_WIDTH).color(BLACK);
    }

    for i in 0..BOARD_SIZE as isize {
        let pos = (i - HALF_BOARD_SIZE as isize) as f32 * CELL_SIZE;
        draw_line(draw, pt2(pos, -LIMIT), pt2(pos, LIMIT));
        draw_line(draw, pt2(-LIMIT, pos), pt2(LIMIT, pos));
    }
}

fn draw_dots(draw: &Draw) {
    for y in -1..=1 {
        for x in -1..=1 {
            let (px, py) = board_to_physical(
                (HALF_BOARD_SIZE as isize + x * DOT_SPACING as isize) as usize,
                (HALF_BOARD_SIZE as isize + y * DOT_SPACING as isize) as usize,
            );
            draw.ellipse().x_y(px, py).w_h(DOT_SIZE, DOT_SIZE).color(BLACK);
        }
    }
}

fn draw_stones(draw: &Draw, model: &Model) {
    fn draw_shadow(draw: &Draw, px: f32, py: f32) {
        draw.ellipse()
            .x_y(px + 1.5, py - 1.5)
            .w_h(STONE_SIZE * 1.03, STONE_SIZE * 1.03)
            .color(nannou::color::rgba(0.0, 0.0, 0.0, 0.65));
    }

    fn draw_stone(draw: &Draw, x: usize, y: usize, turn: Turn) {
        let (px, py) = board_to_physical(x, y);
        draw_shadow(draw, px, py);

        let texture_guard = turn.texture();
        draw.texture(&*texture_guard).x_y(px, py).w_h(STONE_SIZE, STONE_SIZE);
    }

    fn draw_hover_stone(draw: &Draw, x: usize, y: usize, color: LinSrgba) {
        let (px, py) = board_to_physical(x, y);
        draw.ellipse().x_y(px, py).w_h(STONE_SIZE, STONE_SIZE).color(color);
    }

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let turn = model.board[y][x];
            if turn != Turn::None {
                draw_stone(draw, x, y, turn);
            }
        }
    }

    if let Some((x, y)) = model.hover {
        let color = match model.current_player {
            Turn::Black => LinSrgba::new(0.0, 0.0, 0.0, 0.75),
            Turn::White => LinSrgba::new(1.0, 1.0, 1.0, 0.50),
            Turn::None => return,
        };
        draw_hover_stone(draw, x, y, color);
    }
}

fn draw_circle(draw: &Draw, x: usize, y: usize, color: Srgb<u8>) {
    let (px, py) = board_to_physical(x, y);
    draw.ellipse().x_y(px, py).w_h(STONE_SIZE, STONE_SIZE).color(color);
}

fn draw_valid_moves(draw: &Draw, model: &Model) {
    // Tailwind green-500
    const COLOR_VALID_MOVE: Srgb<u8> =
        Srgb { red: 0x22, green: 0xc5, blue: 0x5e, standard: core::marker::PhantomData };

    for &(x, y) in &model.forced_moves {
        if Some((x, y)) != model.hover {
            draw_circle(draw, x, y, COLOR_VALID_MOVE);
        }
    }
}

fn draw_invalid_moves(draw: &Draw, model: &Model) {
    // Tailwind red-500
    const COLOR_INVALID_MOVE: Srgb<u8> =
        Srgb { red: 0xef, green: 0x44, blue: 0x44, standard: core::marker::PhantomData };

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if model.board[y][x] == Turn::None
                && creates_double_three(&model.board, model.current_player, x, y)
            {
                draw_circle(draw, x, y, COLOR_INVALID_MOVE);
            }
        }
    }
}

fn draw_game_over_overlay(draw: &Draw, model: &Model) {
    use nannou::color::{WHITE, rgba};

    let msg = match model.winner {
        Turn::Black => "Black wins",
        Turn::White => "White wins",
        Turn::None => return,
    };

    draw.rect().w_h(WINDOW_SIZE as f32, WINDOW_SIZE as f32).color(rgba(0.0, 0.0, 0.0, 0.55));

    let title_y = WINDOW_SIZE as f32 * 0.03;
    draw.text(msg).color(WHITE).font_size((WINDOW_SIZE as f32 * 0.05) as u32).x_y(0.0, title_y);

    let subtitle_y = title_y - WINDOW_SIZE as f32 * 0.06;
    draw.text("Press Home to restart")
        .color(WHITE)
        .font_size((WINDOW_SIZE as f32 * 0.025) as u32)
        .x_y(0.0, subtitle_y);
}

fn draw_hover_coords(draw: &Draw, model: &Model) {
    use nannou::color::rgba;

    let Some((x, y)) = model.hover else {
        return;
    };

    let (px, py) = board_to_physical(x, y);
    let text = format!("({x}, {y})");
    draw.text(&text).x_y(px, py - CELL_SIZE * 0.65).font_size(16).color(rgba(1.0, 1.0, 1.0, 0.75));
}
