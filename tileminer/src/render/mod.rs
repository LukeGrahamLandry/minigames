use crate::level::{EntityID, Level, TileType};
use crate::Game;
use nannou::color::{BLUE, BROWN};
use nannou::geom::{Point2, Rect, Vec2};
use nannou::text::Point;
use nannou::{App, Draw, Frame};

pub fn view(app: &App, game: &Game, frame: Frame) {
    let draw = app.draw();
    match game {
        Game::Playing { level, player_id } => {
            render_level(level, *player_id, &draw, app.main_window().rect());
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

const TILE_COUNT: usize = 15;

fn render_level(level: &Level, player_id: EntityID, draw: &Draw, window: Rect) {
    let scale = 10.0;
    // Nannou does screen coordinates on a cartesian plane which isn't how I think about things.
    // So flip y and shift the origin so top-left is (0, 0) and shift by the radius of a tile so positions later are top-left instead of center.
    let scale = Vec2::new(scale, -scale);
    let screen_offset = -window.top_left() - (scale / 2.0);

    let top = 0;
    let left = 0;
    for x in left..(left + TILE_COUNT) {
        for y in top..(top + TILE_COUNT) {
            let tile = level.get(x, y);
            let pos = (Vec2::new(x as f32, y as f32) * scale) - screen_offset;
            draw_tile(pos, scale, tile, draw);
        }
    }

    let player = level
        .entities
        .get(&player_id)
        .expect("Tried to render level without player.");
    draw.rect()
        .color(BLUE)
        .xy((player.pos * scale) - screen_offset)
        .wh(scale);
}

fn draw_tile(screen_pos: Vec2, scale: Vec2, tile: TileType, draw: &Draw) {
    let colour = match tile {
        TileType::Empty => return,
        TileType::Dirt => BROWN,
    };
    draw.rect().color(colour).xy(screen_pos).wh(scale);
}
