use crate::gpu::draw::{Drawing, *};
use crate::level::{EntityID, EntityType, Level, TileType};
use glam::Vec2;

pub struct ScreenTransform {
    scale: Vec2,
    screen_offset: Vec2,
    right_tile: i32,
    bottom_tile: i32,
    left_tile: i32,
    top_tile: i32,
    tile_count: i32,
}

impl ScreenTransform {
    pub fn new(level: &Level, player_id: EntityID, window_size: Vec2) -> ScreenTransform {
        let tile_count = (BASE_TILE_COUNT as f32 * level.view_scale) as i32;
        let tile_size = window_size.x / tile_count as f32;
        let scale = Vec2::new(tile_size, tile_size);
        let screen_offset = -window_size - (scale / 2.0);

        let player = level
            .entities
            .get(&player_id)
            .expect("Tried to gpu level without player.");

        let top = player.pos.y - (tile_count / 2 - 2) as f32;
        let left = player.pos.x - (tile_count / 2) as f32;
        let screen_offset = screen_offset + (Vec2::new(left, top) * scale);

        // Decide which area of the grid is in view.
        let top_tile = (player.pos.y - (tile_count as f32 / 2.0 - 2.0))
            .floor()
            .max(0.0) as i32;
        let left_tile = (player.pos.x - (tile_count as f32 / 2.0)).floor().max(0.0) as i32;
        let bottom_tile = (top_tile + tile_count).min(level.tiles.size);
        let right_tile = (left_tile + tile_count).min(level.tiles.size);

        ScreenTransform {
            top_tile,
            left_tile,
            bottom_tile,
            right_tile,
            screen_offset,
            scale,
            tile_count,
        }
    }

    pub fn pixel_to_tile(&self, screen_pos: Vec2) -> (i32, i32) {
        let tile_pos = (screen_pos + self.screen_offset + (self.scale / 2.0)) / self.scale;
        (tile_pos.x as i32, tile_pos.y as i32)
    }

    pub fn tile_to_pixel(&self, x: i32, y: i32) -> Vec2 {
        self.world_to_pixel(Vec2::new(x as f32, y as f32))
    }

    pub fn world_to_pixel(&self, pos: Vec2) -> Vec2 {
        (pos * self.scale) - self.screen_offset
    }
}

const BASE_TILE_COUNT: i32 = 20;

pub fn render_level(level: &Level, player_id: EntityID, draw: &mut Drawing, window_size: Vec2) {
    let cam = ScreenTransform::new(level, player_id, window_size);

    // TODO: do multiple rows at once if they're the same.
    for y in cam.top_tile..cam.bottom_tile {
        let mut last_ty = TileType::OutOfWorld;
        let mut start_pos = cam.left_tile;
        let mut count = 0;
        for x in cam.left_tile..cam.right_tile {
            let tile = level.tiles.get(x, y);
            if tile == last_ty {
                // Just continue the block of the same colour
                count += 1;
            } else {
                // Draw the row we've built up.
                draw_tile(
                    cam.tile_to_pixel(start_pos, y),
                    cam.scale,
                    last_ty,
                    draw,
                    count,
                );
                count = 1;
                start_pos = x;
                last_ty = tile;
            }
        }
        // Draw the last part of the row.
        draw_tile(
            cam.tile_to_pixel(start_pos, y),
            cam.scale,
            last_ty,
            draw,
            count,
        );
    }

    for entity in level.entities.values() {
        let colour = match entity.ty {
            EntityType::Player(_) => BLUE,
            EntityType::FallingTile(tile) => tile_colour(tile),
        };

        let center = cam.world_to_pixel(entity.pos);
        draw.rect().color(colour).xy(center).wh(cam.scale);

        draw.line()
            .color(WHITE)
            .start(center)
            .end(center + (entity.velocity * cam.scale / 5.0));
    }

    let mouse_hover = (Vec2::new(level.mouse_pos.0 as f32, level.mouse_pos.1 as f32) * cam.scale)
        - cam.screen_offset;
    draw.rect()
        .color(LinSrgba::new(0.0, 0.0, 0.0, 0.0))
        .xy(mouse_hover)
        .wh(cam.scale)
        .stroke(GREEN)
        .stroke_weight(3.0);
}

fn draw_tile(
    screen_pos: Vec2,
    size: Vec2,
    tile: TileType,
    draw: &Drawing,
    horizontal_tile_count: i32,
) {
    if horizontal_tile_count == 0 {
        return;
    }

    let center = screen_pos + Vec2::new(size.x * (horizontal_tile_count - 1) as f32 / 2.0, 0.0);
    // TODO: the way they do this seems really stupid. this is like 8 hash table lookups somehow. even just call map_ty myself once
    draw.rect()
        .color(tile_colour(tile))
        .xy(center)
        .h(size.y)
        .w(size.x * horizontal_tile_count as f32);
}

fn tile_colour(tile: TileType) -> Colour {
    match tile {
        TileType::Empty => LIGHT_GRAY,
        TileType::Dirt => BROWN,
        TileType::Sand => ORANGE,
        TileType::OutOfWorld => unreachable!("Tried to gpu outside the world."),
    }
}
