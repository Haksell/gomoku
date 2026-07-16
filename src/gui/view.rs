use crate::{
    game::{
        board::{BOARD_CENTER, BOARD_SIZE, HALF_BOARD_SIZE, Position},
        state::{ForcedMoves, GameState},
    },
    gui::{
        CELL_SIZE, MARKER_DOTS_SPACING, Model, ScreenPosition, WINDOW_MARGIN, WINDOW_SIZE,
        coordinates::board_to_physical,
        textures::{
            TEXTURE_BACKGROUND, TEXTURE_BLACK, TEXTURE_HOVER_BLACK, TEXTURE_HOVER_WHITE,
            TEXTURE_WHITE,
        },
    },
    player::PlayerColor,
};
use nannou::{
    App, Draw, Frame,
    color::{BLACK, Srgb, Srgba, WHITE, rgba},
    geom::{Point2, pt2},
};
use nannou::{
    noise::{NoiseFn as _, OpenSimplex, Seedable as _},
    rand::random,
};
use std::{f32::consts::TAU, sync::LazyLock, time::Instant};

const NO_SCREEN_SHAKE: ScreenPosition = (0., 0.);

const DOT_SIZE_MARKER: f32 = CELL_SIZE * 0.25;
const DOT_SIZE_LAST_MOVE: f32 = CELL_SIZE * 0.125;

const LINE_WIDTH: f32 = CELL_SIZE * 0.052;
const STONE_SIZE: f32 = CELL_SIZE * 0.77;

#[expect(clippy::needless_pass_by_value)]
pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw_background(&draw);
    draw_grid(&draw);
    draw_marker_dots(&draw);

    draw_stones(&draw, model);
    draw_last_move(&draw, model);
    draw_win_by_alignment(&draw, model);

    if model.game.state.is_playing() {
        if let GameState::Playing(forced_moves) = &model.game.state
            && !forced_moves.is_empty()
        {
            draw_forced_moves(&draw, forced_moves, model.hover);
        } else {
            draw_invalid_moves(&draw, model);
        }
    }

    // draw_hover_coords(&draw, model);
    // draw_last_captures(&draw, model);

    // TODO: use winning way
    // TODO: info on the right instead of overlay
    match model.game.state {
        GameState::Playing(_) => {}
        GameState::Draw => draw_game_over_overlay(&draw, None, model.finished_time),
        GameState::Won(winner, _) => {
            draw_game_over_overlay(&draw, Some(winner), model.finished_time);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn compute_screen_shake(
    finished_time: Option<Instant>,
    (board_x, board_y): Position,
) -> ScreenPosition {
    /// Shamelessly stolen from <https://easings.net/#easeOutBounce>.
    fn ease_out_bounce(x: f32) -> f32 {
        const N1: f32 = 7.5625;
        const D1: f32 = 2.75;

        if x < 1. / D1 {
            N1 * x * x
        } else if x < 2. / D1 {
            N1 * (x - 1.5 / D1) * x + 0.75
        } else if x < 2.5 / D1 {
            N1 * (x - 2.25 / D1) * x + 0.9375
        } else {
            N1 * (x - 2.625 / D1) * x + 0.984_375
        }
    }

    static NOISE: LazyLock<OpenSimplex> = LazyLock::new(|| OpenSimplex::new().set_seed(random()));

    const SCREEN_SHAKE_DURATION: f32 = 0.5;
    const MAGNITUDE: f32 = 13.;
    const SPEED: f32 = 11.;
    const EASING_EXPONENT: f32 = 1.5;

    let Some(finished_time) = finished_time else {
        return NO_SCREEN_SHAKE;
    };

    let elapsed = finished_time.elapsed().as_secs_f32().min(SCREEN_SHAKE_DURATION);
    let noise = NOISE.get([board_x as f64, board_y as f64, elapsed as f64]) as f32;

    let factor = (1. - (elapsed / SCREEN_SHAKE_DURATION)).powf(EASING_EXPONENT);
    let noise_factor = 1. + noise / 3.; // ~1
    let shake_y = (elapsed * SPEED * TAU * noise_factor).cos() * MAGNITUDE * factor * noise_factor;
    let x_factor = -0.2 - noise / 4.;
    let shake_x = ease_out_bounce(shake_y * x_factor);

    (shake_x, shake_y)
}

fn draw_last_move(draw: &Draw, model: &Model) {
    if let Some(&pos) = model.game.moves.last() {
        let color = match model.game.current_color {
            PlayerColor::Black => BLACK,
            PlayerColor::White => WHITE,
        };
        draw_circle(
            draw,
            pos,
            DOT_SIZE_LAST_MOVE,
            color,
            compute_screen_shake(model.finished_time, pos),
        );
    }
}

fn draw_win_by_alignment(draw: &Draw, model: &Model) {
    if let GameState::Won(player_color, winning_way) = &model.game.state {
        let (color, stroke_weight) = match player_color {
            PlayerColor::Black => (WHITE, CELL_SIZE * 0.05),
            PlayerColor::White => (BLACK, CELL_SIZE * 0.065),
        };
        for alignment in &winning_way.winning_alignments {
            for &pos in alignment {
                draw_ring(
                    draw,
                    pos,
                    CELL_SIZE * 0.425,
                    stroke_weight,
                    color,
                    compute_screen_shake(model.finished_time, pos),
                );
            }
        }
    }
}

// fn draw_last_captures(draw: &Draw, model: &Model) {
//     for &(_, pos1, pos2) in
//         model.game.captures.iter().take_while(|(ply, _, _)| *ply == model.game.ply)
//     {
//         let color = match model.game.current_color {
//             PlayerColor::Black => BLACK,
//             PlayerColor::White => WHITE,
//         };
//         draw_circle(draw, pos1, STONE_SIZE, color);
//         draw_circle(draw, pos2, STONE_SIZE, color);
//     }
// }

fn draw_background(draw: &Draw) {
    let background_texture = TEXTURE_BACKGROUND.get().unwrap();
    draw.texture(background_texture).w_h(WINDOW_SIZE as f32, WINDOW_SIZE as f32);
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

fn draw_marker_dots(draw: &Draw) {
    for y in -1..=1 {
        for x in -1..=1 {
            let x = (HALF_BOARD_SIZE as isize + x * MARKER_DOTS_SPACING as isize) as usize;
            let y = (HALF_BOARD_SIZE as isize + y * MARKER_DOTS_SPACING as isize) as usize;
            draw_circle(draw, (x, y), DOT_SIZE_MARKER, BLACK, NO_SCREEN_SHAKE);
        }
    }
}

fn draw_stones(draw: &Draw, model: &Model) {
    // TODO: draw_circle
    fn draw_shadow(draw: &Draw, (px, py): ScreenPosition) {
        draw.ellipse()
            .x_y(px + 1.5, py - 1.5)
            .w_h(STONE_SIZE * 1.03, STONE_SIZE * 1.03)
            .color(nannou::color::rgba(0., 0., 0., 0.65));
    }

    fn draw_stone(
        draw: &Draw,
        pos: Position,
        player_color: PlayerColor,
        transparent: bool,
        (shake_x, shake_y): ScreenPosition,
    ) {
        let (mut px, mut py) = board_to_physical(pos);
        px += shake_x;
        py += shake_y;

        if !transparent {
            draw_shadow(draw, (px, py));
        }

        let texture_guard = match (player_color, transparent) {
            (PlayerColor::Black, false) => TEXTURE_BLACK.get().unwrap(),
            (PlayerColor::White, false) => TEXTURE_WHITE.get().unwrap(),
            (PlayerColor::Black, true) => TEXTURE_HOVER_BLACK.get().unwrap(),
            (PlayerColor::White, true) => TEXTURE_HOVER_WHITE.get().unwrap(),
        };
        draw.texture(texture_guard).x_y(px, py).w_h(STONE_SIZE, STONE_SIZE);
    }

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if let Some(color) = model.game.board[y][x] {
                draw_stone(
                    draw,
                    (x, y),
                    color,
                    false,
                    compute_screen_shake(model.finished_time, (x, y)),
                );
            }
        }
    }

    if let Some(pos) = model.hover {
        draw_stone(
            draw,
            pos,
            model.game.current_color,
            true,
            compute_screen_shake(model.finished_time, pos),
        );
    }
}

fn draw_forced_moves(draw: &Draw, forced_moves: &ForcedMoves, hover: Option<Position>) {
    // Tailwind green-500
    const COLOR_VALID_MOVE: Srgb<u8> =
        Srgb { red: 0x22, green: 0xc5, blue: 0x5e, standard: std::marker::PhantomData };

    for &pos in forced_moves {
        if hover != Some(pos) {
            draw_circle(draw, pos, STONE_SIZE, COLOR_VALID_MOVE, NO_SCREEN_SHAKE);
        }
    }
}

fn draw_invalid_moves(draw: &Draw, model: &Model) {
    // Tailwind red-500
    const COLOR_INVALID_MOVE: Srgb<u8> =
        Srgb { red: 0xef, green: 0x44, blue: 0x44, standard: std::marker::PhantomData };

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if model.game.board[y][x].is_none() && model.game.creates_double_three((x, y)) {
                draw_circle(draw, (x, y), STONE_SIZE, COLOR_INVALID_MOVE, NO_SCREEN_SHAKE);
            }
        }
    }
}

fn draw_game_over_overlay(
    draw: &Draw,
    winner: Option<PlayerColor>,
    finished_time: Option<Instant>,
) {
    let msg = match winner {
        None => "Draw",
        Some(PlayerColor::Black) => "Black wins",
        Some(PlayerColor::White) => "White wins",
    };

    draw.rect().w_h(WINDOW_SIZE as f32, WINDOW_SIZE as f32).color(rgba(0., 0., 0., 0.47));

    let (mut shake_x, mut shake_y) = compute_screen_shake(finished_time, BOARD_CENTER);
    shake_x *= 4.;
    shake_y *= 4.;

    let title_y = WINDOW_SIZE as f32 * 0.03;
    draw.text(msg)
        .color(WHITE)
        .font_size((WINDOW_SIZE as f32 * 0.05) as u32)
        .x_y(shake_x, title_y + shake_y);

    let subtitle_y = title_y - WINDOW_SIZE as f32 * 0.06;
    draw.text("Press Home to restart")
        .color(WHITE)
        .font_size((WINDOW_SIZE as f32 * 0.025) as u32)
        .x_y(shake_x, subtitle_y + shake_y);
}

// fn draw_hover_coords(draw: &Draw, model: &Model) {
//     let Some((x, y)) = model.hover else {
//         return;
//     };

//     let (px, py) = board_to_physical(x, y);
//     let text = format!("({x}, {y})");
//     draw.text(&text).x_y(px, py - CELL_SIZE * 0.65).font_size(16).color(rgba(1., 1., 1., 0.75));
// }

fn draw_circle(
    draw: &Draw,
    pos: Position,
    size: f32,
    color: Srgb<u8>,
    (shake_x, shake_y): ScreenPosition,
) {
    let (px, py) = board_to_physical(pos);
    draw.ellipse().x_y(px + shake_x, py + shake_y).w_h(size, size).color(color);
}

fn draw_ring(
    draw: &Draw,
    pos: Position,
    ring_size: f32,
    stroke_weight: f32,
    color: Srgb<u8>,
    (shake_x, shake_y): ScreenPosition,
) {
    let (px, py) = board_to_physical(pos);
    draw.ellipse()
        .x_y(px + shake_x, py + shake_y)
        .w_h(ring_size, ring_size)
        .stroke_weight(stroke_weight)
        .stroke_color(color)
        .color(Srgba { color, alpha: 0 });
}
