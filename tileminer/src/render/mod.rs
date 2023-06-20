use crate::level::{EntityID, Level, TileType};
use crate::Game;
use nannou::color::{BLACK, BLUE, BROWN, GRAY, RED, WHITE};
use nannou::draw::primitive::polygon::{PolygonOptions, SetPolygon};
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

const TILE_COUNT: usize = 50;

fn render_level(level: &Level, player_id: EntityID, draw: &Draw, window: Rect) {
    draw.background().color(BLACK);

    let tile_size = window.w() / TILE_COUNT as f32;
    // Nannou does screen coordinates on a cartesian plane which isn't how I think about things.
    // So flip y and shift the origin so top-left is (0, 0) and shift by the radius of a tile so positions later are top-left instead of center.
    let scale = Vec2::new(tile_size, -tile_size);
    let screen_offset = -window.top_left() - (scale / 2.0);

    let player = level
        .entities
        .get(&player_id)
        .expect("Tried to render level without player.");

    let top = player.pos.y - (TILE_COUNT / 2) as f32;
    let left = player.pos.x - (TILE_COUNT / 2) as f32;
    let screen_offset = screen_offset + (Vec2::new(left, top) * scale);

    // Decide which area of the grid is in view.
    let top_tile = (player.pos.y - (TILE_COUNT as f32 / 2.0)).floor().max(0.0) as usize;
    let left_tile = (player.pos.x - (TILE_COUNT as f32 / 2.0)).floor().max(0.0) as usize;
    let bottom_tile = (top_tile + TILE_COUNT).min(level.size);
    let right_tile = (left_tile + TILE_COUNT).min(level.size);

    // TODO: batch draws of identical tiles.
    for x in left_tile..right_tile {
        for y in top_tile..bottom_tile {
            let tile = level.get(x, y);
            let pos = (Vec2::new(x as f32, y as f32) * scale) - screen_offset;
            draw_tile(pos, scale, tile, draw);
        }
    }

    draw.rect()
        .color(BLUE)
        .xy((player.pos * scale) - screen_offset)
        .wh(scale);
}

fn draw_tile(screen_pos: Vec2, size: Vec2, tile: TileType, draw: &Draw) {
    let colour = match tile {
        TileType::Empty => GRAY,
        TileType::Dirt => BROWN,
    };
    draw.rect().color(colour).xy(screen_pos).wh(size);
}
