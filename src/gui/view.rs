use crate::{
    game::{
        board::{BOARD_SIZE, HALF_BOARD_SIZE, Position},
        state::{ForcedMoves, GameState},
    },
    gui::{
        CELL_SIZE, MARKER_DOTS_SPACING, Model, WINDOW_MARGIN, WINDOW_SIZE,
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

    if model.game.state.is_playing() {
        if let GameState::Playing(forced_moves) = &model.game.state
            && !forced_moves.is_empty()
        {
            draw_forced_moves(&draw, forced_moves, model.hover);
        } else {
            draw_invalid_moves(&draw, model);
        }
    }

    draw_hover_coords(&draw, model);
    draw_last_move(&draw, model);
    // draw_last_captures(&draw, model);

    draw_win_by_alignment(&draw, model);

    // TODO: use winning way
    match model.game.state {
        GameState::Playing(_) => {}
        GameState::Draw => draw_game_over_overlay(&draw, None),
        GameState::Won(winner, _) => draw_game_over_overlay(&draw, Some(winner)),
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_last_move(draw: &Draw, model: &Model) {
    if let Some(&pos) = model.game.moves.last() {
        let color = match model.game.current_color {
            PlayerColor::Black => BLACK,
            PlayerColor::White => WHITE,
        };
        draw_circle(draw, pos, DOT_SIZE_LAST_MOVE, color);
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
                draw_ring(draw, pos, CELL_SIZE * 0.425, stroke_weight, color);
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
            draw_circle(draw, (x, y), DOT_SIZE_MARKER, BLACK);
        }
    }
}

fn draw_stones(draw: &Draw, model: &Model) {
    // TODO: draw_circle
    fn draw_shadow(draw: &Draw, px: f32, py: f32) {
        draw.ellipse()
            .x_y(px + 1.5, py - 1.5)
            .w_h(STONE_SIZE * 1.03, STONE_SIZE * 1.03)
            .color(nannou::color::rgba(0.0, 0.0, 0.0, 0.65));
    }

    fn draw_stone(draw: &Draw, x: usize, y: usize, player_color: PlayerColor, transparent: bool) {
        let (px, py) = board_to_physical(x, y);
        if !transparent {
            draw_shadow(draw, px, py);
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
                draw_stone(draw, x, y, color, false);
            }
        }
    }

    if let Some((x, y)) = model.hover {
        draw_stone(draw, x, y, model.game.current_color, true);
    }
}

fn draw_forced_moves(draw: &Draw, forced_moves: &ForcedMoves, hover: Option<Position>) {
    // Tailwind green-500
    const COLOR_VALID_MOVE: Srgb<u8> =
        Srgb { red: 0x22, green: 0xc5, blue: 0x5e, standard: std::marker::PhantomData };

    for &pos in forced_moves {
        if hover != Some(pos) {
            draw_circle(draw, pos, STONE_SIZE, COLOR_VALID_MOVE);
        }
    }
}

fn draw_invalid_moves(draw: &Draw, model: &Model) {
    // Tailwind red-500
    const COLOR_INVALID_MOVE: Srgb<u8> =
        Srgb { red: 0xef, green: 0x44, blue: 0x44, standard: std::marker::PhantomData };

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            if model.game.board[y][x].is_none() && model.game.creates_double_three(x, y) {
                draw_circle(draw, (x, y), STONE_SIZE, COLOR_INVALID_MOVE);
            }
        }
    }
}

fn draw_game_over_overlay(draw: &Draw, winner: Option<PlayerColor>) {
    let msg = match winner {
        None => "Draw",
        Some(PlayerColor::Black) => "Black wins",
        Some(PlayerColor::White) => "White wins",
    };

    draw.rect().w_h(WINDOW_SIZE as f32, WINDOW_SIZE as f32).color(rgba(0.0, 0.0, 0.0, 0.47));

    let title_y = WINDOW_SIZE as f32 * 0.03;
    draw.text(msg).color(WHITE).font_size((WINDOW_SIZE as f32 * 0.05) as u32).x_y(0.0, title_y);

    let subtitle_y = title_y - WINDOW_SIZE as f32 * 0.06;
    draw.text("Press Home to restart")
        .color(WHITE)
        .font_size((WINDOW_SIZE as f32 * 0.025) as u32)
        .x_y(0.0, subtitle_y);
}

fn draw_hover_coords(draw: &Draw, model: &Model) {
    let Some((x, y)) = model.hover else {
        return;
    };

    let (px, py) = board_to_physical(x, y);
    let text = format!("({x}, {y})");
    draw.text(&text).x_y(px, py - CELL_SIZE * 0.65).font_size(16).color(rgba(1.0, 1.0, 1.0, 0.75));
}

fn draw_circle(draw: &Draw, (x, y): Position, size: f32, color: Srgb<u8>) {
    let (px, py) = board_to_physical(x, y);
    draw.ellipse().x_y(px, py).w_h(size, size).color(color);
}

fn draw_ring(draw: &Draw, (x, y): Position, ring_size: f32, stroke_weight: f32, color: Srgb<u8>) {
    let (px, py) = board_to_physical(x, y);
    draw.ellipse()
        .x_y(px, py)
        .w_h(ring_size, ring_size)
        .stroke_weight(stroke_weight)
        .stroke_color(color)
        .color(Srgba { color, alpha: 0 });
}
