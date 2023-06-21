use crate::level::{Entity, EntityID, EntityType, Level, TileType};
use crate::player::PlayerData;
use glam::Vec2;

pub fn starting_level() -> (Level, EntityID) {
    let mut level = Level::new(1000);
    for i in 0..4000 {
        level.tiles.tiles[i] = TileType::Empty;
    }
    for i in 0..6 {
        level.tiles.raw_set(20, i, TileType::Dirt);
    }

    for _ in 0..1000 {
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
    let player_id = level.add_entity(Vec2::new(3.0, 0.0), EntityType::Player(PlayerData {}));
    level.add_entity(
        Vec2::new(10.0, 1.0),
        EntityType::FallingTile(TileType::Sand),
    );
    (level, player_id)
}

fn random_range(min: i32, max_exclusive: i32) -> i32 {
    debug_assert!(max_exclusive > min);
    (rand::random::<i32>() % (max_exclusive - min)) + min
}
