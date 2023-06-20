use crate::level::{Entity, EntityID, EntityType, Level, TileType};
use crate::player::PlayerData;
use nannou::geom::Vec2;

pub fn starting_level() -> (Level, EntityID) {
    let mut level = Level::new(100);
    for i in 0..400 {
        level.tiles[i] = TileType::Empty;
    }
    for i in 0..6 {
        level.set(20, i, TileType::Dirt);
    }
    let player_id = level.add_entity(Vec2::new(3.0, 0.0), EntityType::Player(PlayerData {}));
    level.add_entity(Vec2::new(10.0, 1.0), EntityType::Box);
    (level, player_id)
}
