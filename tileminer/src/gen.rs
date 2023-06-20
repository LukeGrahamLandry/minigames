use crate::level::{Entity, EntityID, EntityType, Level, TileType};
use crate::player::PlayerData;
use nannou::geom::Vec2;

pub fn starting_level() -> (Level, EntityID) {
    let mut level = Level::new(50);
    for i in 0..150 {
        level.tiles[i] = TileType::Empty;
    }
    for i in (level.tiles.len() - 100)..level.tiles.len() {
        level.tiles[i] = TileType::Empty;
    }
    let player_id = level.add_entity(Vec2::new(0.0, 0.0), EntityType::Player(PlayerData {}));
    (level, player_id)
}
