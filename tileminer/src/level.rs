use crate::player::PlayerData;
use crate::gpu::ScreenTransform;
use glam::Vec2;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Level {
    // TODO: Rust's default hasher is chosen for resilience against malicious input, sacrificing performance on small keys.
    //       Since I know I'm just using sequential integers, they can just be their own hashcode and this becomes super efficient (other than iteration).
    //       Normal hashmaps already do the obvious power of two trick to make wrapping use a bitshift instead of division.
    pub entities: HashMap<EntityID, Entity>,
    pub mouse_pos: (i32, i32),
    pub tiles: TileMap,
    pub view_scale: f32,
}

pub struct TileMap {
    pub size: i32,
    pub tiles: Box<[TileType]>,
}

impl Level {
    pub fn new(width: usize) -> Level {
        let tiles = vec![TileType::Dirt; width * width];
        Level {
            tiles: TileMap {
                size: width as i32,
                tiles: tiles.into_boxed_slice(),
            },
            entities: HashMap::new(),
            mouse_pos: (0, 0),
            view_scale: 1.0,
        }
    }

    pub fn update(&mut self, delta_t: f32) {
        const GRAVITY: f32 = 20.0;
        const FRICTION: f32 = 4.0;
        for entity in self.entities.values_mut() {
            entity.last_good_pos = entity.pos;
            entity.velocity.y += GRAVITY * delta_t;
            entity.pos += entity.velocity * delta_t;
            if entity.is_on_ground {
                entity.velocity.x *= 1.0 - (FRICTION * delta_t);
                entity.is_on_ground = false;
            }

            if entity.pos.y > (self.tiles.size as f32 + 50.0) {
                entity.pos.y = -50.0;
                entity.velocity = entity.velocity.normalize();
            }
        }

        self.handle_collisions();
    }

    // This probably doesn't work if you're moving faster than a tile per frame but that seems like a non-issue.
    fn handle_collisions(&mut self) {
        let mut falling_done = vec![];
        for entity in self.entities.values_mut() {
            let mut pos = entity.pos;
            let mut vel = entity.velocity;
            let mut dirty = false;
            let mut ground = false;
            for x in -1..=1 {
                for y in -1..=1 {
                    let check_x = pos.x as i32 + x;
                    let check_y = pos.y as i32 + y;
                    let tile = self.tiles.get(check_x, check_y);
                    if tile.is_solid() {
                        // In the same row as the tile.
                        if pos.y >= check_y as f32 && pos.y < check_y as f32 + 1.0 {
                            // Hit left
                            if pos.x > check_x as f32 && pos.x < check_x as f32 + 1.0 {
                                vel.x = vel.x.max(0.0);
                                pos.x = pos.x.ceil();
                                dirty = true;
                            }

                            // Hit right.
                            if pos.x + 1.0 > check_x as f32 && pos.x < check_x as f32 + 1.0 {
                                vel.x = vel.x.min(0.0);
                                pos.x = pos.x.floor();
                                dirty = true;
                            }
                        }

                        // In the same column as the tile.
                        if pos.x >= check_x as f32 && pos.x < check_x as f32 + 1.0 {
                            // Hitting the ceiling => snap down (+y).
                            if pos.y > check_y as f32 && pos.y < check_y as f32 + 1.0 {
                                vel.y = vel.y.max(0.0);
                                pos.y = pos.y.ceil();
                                dirty = true;
                            }

                            // Hitting the floor => snap up (-y).
                            if pos.y + 1.0 > check_y as f32 && pos.y < check_y as f32 {
                                vel.y = vel.y.min(0.0);
                                pos.y = pos.y.floor();
                                dirty = true;
                                ground = true;
                                if let EntityType::FallingTile(_) = entity.ty {
                                    falling_done.push(entity.id);
                                }
                            }
                        }
                    }
                }
            }
            if dirty {
                entity.pos = pos;
                entity.is_on_ground = ground;
                entity.velocity = vel;
                if self.tiles.is_in_wall(entity) {
                    entity.pos = entity.last_good_pos;
                    entity.velocity = Vec2::ZERO;
                }
            }
        }

        for id in falling_done {
            let entity = self.entities.remove(&id).unwrap();
            match entity.ty {
                EntityType::FallingTile(tile) => {
                    let x = entity.pos.x as i32;
                    let y = entity.pos.y as i32;
                    if self.tiles.get(x, y).is_solid() {
                        // Multiple landed in the same place on the same frame, just let the collision logic resolve it and try again next frame.
                        self.add_entity(entity.pos, entity.ty);
                    } else {
                        self.set(x, y, tile);
                        // Let stuff get snapped away from the new block. IDK if this feels right.
                        self.handle_collisions();
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    pub fn tile_update(&mut self, center_x: i32, center_y: i32) {
        for x in (center_x - 1)..=(center_x + 1) {
            for y in (center_y - 1)..=(center_y + 1) {
                let tile = self.tiles.get(x, y);
                if tile == TileType::Sand && !self.tiles.get(x, y + 1).is_solid() {
                    self.add_entity(Vec2::new(x as f32, y as f32), EntityType::FallingTile(tile));
                    self.set(x, y, TileType::Empty);
                }
            }
        }
    }

    pub fn set(&mut self, x: i32, y: i32, tile: TileType) {
        self.tiles.raw_set(x, y, tile);
        self.tile_update(x, y);
    }

    pub fn add_entity(&mut self, pos: Vec2, ty: EntityType) -> EntityID {
        let id = EntityID::get_next();
        let entity = Entity {
            id,
            pos,
            velocity: Vec2::ZERO,
            ty,
            is_on_ground: false,
            last_good_pos: pos,
        };
        self.entities.insert(id, entity);
        id
    }
}

impl TileMap {
    pub fn get(&self, x: i32, y: i32) -> TileType {
        if x < 0 || y < 0 || x >= self.size || y >= self.size {
            TileType::OutOfWorld
        } else {
            let index = x + (y * self.size);
            self.tiles[index as usize]
        }
    }

    pub fn raw_set(&mut self, x: i32, y: i32, tile: TileType) {
        let index = x + (y * self.size);
        self.tiles[index as usize] = tile;
    }

    pub fn set_silently_fail(&mut self, x: i32, y: i32, tile: TileType) {
        if x < 0 || y < 0 || x >= self.size || y >= self.size {
            return;
        }
        self.raw_set(x, y, tile);
    }

    fn is_in_wall(&self, entity: &Entity) -> bool {
        let pos = entity.pos;
        for x in -1..=1 {
            for y in -1..=1 {
                let check_x = pos.x as i32 + x;
                let check_y = pos.y as i32 + y;
                let tile = self.get(check_x, check_y);
                if tile.is_solid() {
                    // In the same row as the tile.
                    if pos.y >= check_y as f32 && pos.y < check_y as f32 + 1.0 {
                        // Hit left
                        if pos.x > check_x as f32 && pos.x < check_x as f32 + 1.0 {
                            return true;
                        }

                        // Hit right.
                        if pos.x + 1.0 > check_x as f32 && pos.x < check_x as f32 + 1.0 {
                            return true;
                        }
                    }

                    // In the same column as the tile.
                    if pos.x >= check_x as f32 && pos.x < check_x as f32 + 1.0 {
                        // Hitting the ceiling => snap down (+y).
                        if pos.y > check_y as f32 && pos.y < check_y as f32 + 1.0 {
                            return true;
                        }

                        // Hitting the floor => snap up (-y).
                        if pos.y + 1.0 > check_y as f32 && pos.y < check_y as f32 {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

pub struct Entity {
    pub id: EntityID,
    pub pos: Vec2,
    pub velocity: Vec2,
    pub ty: EntityType,
    pub(crate) is_on_ground: bool,
    last_good_pos: Vec2,
}

pub enum EntityType {
    Player(PlayerData),
    FallingTile(TileType),
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    OutOfWorld,
    Empty,
    Dirt,
    Sand,
}

impl TileType {
    pub fn is_solid(&self) -> bool {
        !matches!(self, TileType::Empty)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EntityID(u32);

impl EntityID {
    fn get_next() -> EntityID {
        // Same variable used in every invocation of the function, even across multiple threads.
        // Relaxed ordering is fine because we just care that values are unique (which the atomic provides).
        static NEXT_ID: AtomicU32 = AtomicU32::new(0);
        EntityID(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[test]
fn entity_id_sanity() {
    assert_ne!(EntityID::get_next(), EntityID::get_next());
}
