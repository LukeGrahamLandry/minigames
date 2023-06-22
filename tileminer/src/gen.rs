use crate::level::{EntityID, EntityType, Level, TileMap, TileType};
use crate::player::PlayerData;
use glam::Vec2;

pub fn starting_level() -> (Level, EntityID) {
    let w = 1000;
    let mut level = Level::new(w);
    for i in 0..(w * 4) {
        level.tiles.tiles[i] = TileType::Empty;
    }
    for i in 0..4 {
        level.tiles.raw_set(20, i, TileType::Dirt);
    }

    for _ in 0..w {
        let mut width = 20;
        let height = 10;
        let xx = random_range(0, level.tiles.size);
        let yy = random_range(0, level.tiles.size);

        for y_offset in 0..height {
            for x_offset in 0..width {
                level.tiles.set_silently_fail(
                    xx + x_offset - (width / 2),
                    yy + y_offset,
                    TileType::Sand,
                );
            }
            width += random_range(-7, 7);
        }
    }

    for _ in 0..100000 {
        let x = random_range(0, level.tiles.size);
        let y = random_range(0, level.tiles.size);
        level
            .tiles
            .set_silently_fail(x, y, TileType::ExplodingBarrel);
    }

    for _ in 0..500 {
        let x1 = random_range(10, level.tiles.size - 10);
        let y1 = random_range(10, level.tiles.size - 10);
        let x2 = random_range(10, level.tiles.size - 10);
        let y2 = random_range(10, level.tiles.size - 10);
        place_portal(&mut level, x1, y1, x2, y2);
        place_portal(&mut level, x2, y2, x1, y1);
    }

    let player_id = level.add_entity(Vec2::new(3.0, 0.0), EntityType::Player(PlayerData {}));
    level.add_entity(
        Vec2::new(10.0, 1.0),
        EntityType::FallingTile(TileType::Sand),
    );
    (level, player_id)
}

fn place_portal(level: &mut Level, x: i32, y: i32, target_x: i32, target_y: i32) {
    for y_offset in -1..=1 {
        for x_offset in -2..=2 {
            level
                .tiles
                .set_silently_fail(x + x_offset, y + y_offset, TileType::Empty);
        }
    }
    assert!(level.tiles.in_bounds(target_x, target_y + 1));
    level.add_entity(
        Vec2::new(x as f32, y as f32),
        EntityType::Portal {
            target: Vec2::new(target_x as f32 + 2.0, target_y as f32),
        },
    );
}

fn random_range(min: i32, max_exclusive: i32) -> i32 {
    debug_assert!(max_exclusive > min);
    (rand::random::<i32>().abs() % (max_exclusive - min)) + min
}
