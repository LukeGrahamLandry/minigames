use crate::player::PlayerData;
use nannou::geom::Vec2;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Level {
    pub(crate) size: usize,
    pub tiles: Box<[TileType]>,
    // TODO: Rust's default hasher is chosen for resilience against malicious input, sacrificing performance on small keys.
    //       Since I know I'm just using sequential integers, they can just be their own hashcode and this becomes super efficient (other than iteration).
    //       Normal hashmaps already do the obvious power of two trick to make wrapping use a bitshift instead of division.
    pub entities: HashMap<EntityID, Entity>,
    wants_jump: bool,
}

impl Level {
    pub fn new(width: usize) -> Level {
        let tiles = vec![TileType::Dirt; width * width];
        Level {
            size: width,
            tiles: tiles.into_boxed_slice(),
            entities: HashMap::new(),
            wants_jump: false,
        }
    }

    pub fn update(&mut self, delta_t: f32) {
        const GRAVITY: f32 = 10.0;
        const JUMP: f32 = 15.0;
        let max_pos = Vec2::new(self.size as f32 - 1.0, self.size as f32 - 1.0);
        for entity in self.entities.values_mut() {
            entity.velocity.y += GRAVITY * delta_t;
            entity.pos += entity.velocity * delta_t;
            entity.pos = entity.pos.clamp(Vec2::ZERO, max_pos);

            if self.wants_jump {
                if let EntityType::Player(_) = entity.ty {
                    entity.velocity.y -= JUMP;
                    self.wants_jump = false;
                }
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> TileType {
        let index = x + (y * self.size);
        self.tiles[index]
    }

    pub fn set(&mut self, x: usize, y: usize, ty: TileType) {
        let index = x + (y * self.size);
        self.tiles[index] = ty;
    }

    pub fn add_entity(&mut self, pos: Vec2, ty: EntityType) -> EntityID {
        let id = EntityID::get_next();
        let entity = Entity {
            id,
            pos,
            velocity: Vec2::ZERO,
            ty,
        };
        self.entities.insert(id, entity);
        id
    }

    pub fn jump(&mut self) {
        // TODO: if on ground
        self.wants_jump = true;
    }
}

pub struct Entity {
    pub id: EntityID,
    pub pos: Vec2,
    pub velocity: Vec2,
    pub ty: EntityType,
}

pub enum EntityType {
    Player(PlayerData),
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Empty,
    Dirt,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct EntityID(usize);

impl EntityID {
    fn get_next() -> EntityID {
        // Same variable used in every invocation of the function, even across multiple threads.
        // Relaxed ordering is fine because we just care that values are unique (which the atomic provides).
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        EntityID(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[test]
fn entity_id_sanity() {
    assert_ne!(EntityID::get_next(), EntityID::get_next());
}
