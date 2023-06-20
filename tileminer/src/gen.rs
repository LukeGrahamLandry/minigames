use crate::level::{Entity, EntityID, EntityType, Level, TileType};
use crate::player::PlayerData;
use nannou::geom::Vec2;

pub fn starting_level() -> (Level, EntityID) {
    let mut level = Level::new(100);
    for i in 0..300 {
        level.tiles[i] = TileType::Empty;
    }
    let player_id = EntityID::new();
    level.entities.insert(
        player_id,
        Entity {
            pos: Vec2::new(0.0, 0.0),
            ty: EntityType::Player(PlayerData {}),
        },
    );
    (level, player_id)
}
