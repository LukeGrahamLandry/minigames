use crate::level::{EntityID, EntityType, Level, TileType};
use crate::Game;
use nannou::color::{BLACK, BLUE, BROWN, GRAY, ORANGE, WHITE};
use nannou::geom::{Rect, Vec2};
use nannou::prelude::GREEN;
use nannou::{App, Draw, Frame};

pub fn view(app: &App, game: &Game, frame: Frame) {
    let draw = app.draw();
    match game {
        Game::Playing { level, player_id } => {
            render_level(level, *player_id, &draw, app.main_window().rect());
        }
    }

    draw.text(&format!("FPS: {}", app.fps()));
    draw.to_frame(app, &frame).unwrap();
}

pub struct ScreenTransform {
    scale: Vec2,
    screen_offset: Vec2,
    right_tile: usize,
    bottom_tile: usize,
    left_tile: usize,
    top_tile: usize,
}

impl ScreenTransform {
    pub fn new(level: &Level, player_id: EntityID, window: Rect) -> ScreenTransform {
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
        let bottom_tile = (top_tile + TILE_COUNT + 1).min(level.size);
        let right_tile = (left_tile + TILE_COUNT + 1).min(level.size);

        ScreenTransform {
            top_tile,
            left_tile,
            bottom_tile,
            right_tile,
            screen_offset,
            scale,
        }
    }

    pub fn pixel_to_tile(&self, screen_pos: Vec2) -> (isize, isize) {
        let tile_pos = (screen_pos + self.screen_offset + (self.scale / 2.0)) / self.scale;
        (tile_pos.x as isize, tile_pos.y as isize)
    }
}

const TILE_COUNT: usize = 50;

fn render_level(level: &Level, player_id: EntityID, draw: &Draw, window: Rect) {
    draw.background().color(BLACK);

    let cam = ScreenTransform::new(level, player_id, window);

    // TODO: batch draws of identical tiles.
    for y in cam.top_tile..cam.bottom_tile {
        for x in cam.left_tile..cam.right_tile {
            let tile = level.get(x, y);
            let pos = (Vec2::new(x as f32, y as f32) * cam.scale) - cam.screen_offset;
            draw_tile(pos, cam.scale, tile, draw);
        }
    }

    for entity in level.entities.values() {
        let colour = match entity.ty {
            EntityType::Player(_) => BLUE,
            EntityType::Box => ORANGE,
        };
        draw.rect()
            .color(colour)
            .xy((entity.pos * cam.scale) - cam.screen_offset)
            .wh(cam.scale);

        let center = (entity.pos * cam.scale) - cam.screen_offset;
        draw.line()
            .color(WHITE)
            .start(center)
            .end(center + (entity.velocity * cam.scale / 5.0));
    }

    let mouse_hover = (Vec2::new(level.mouse_pos.0 as f32, level.mouse_pos.1 as f32) * cam.scale)
        - cam.screen_offset;
    draw.rect().color(GREEN).xy(mouse_hover).wh(cam.scale);
}

fn draw_tile(screen_pos: Vec2, size: Vec2, tile: TileType, draw: &Draw) {
    let colour = match tile {
        TileType::Empty => GRAY,
        TileType::Dirt => BROWN,
        TileType::Void => unreachable!("Tried to render outside the world."),
    };
    draw.rect().color(colour).xy(screen_pos).wh(size);
}
