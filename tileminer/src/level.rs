use crate::player::PlayerData;
use nannou::geom::Vec2;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Level {
    width: usize,
    pub tiles: Box<[TileType]>,
    pub entities: HashMap<EntityID, Entity>,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Empty,
    Dirt,
}

impl Level {
    pub fn new(width: usize) -> Level {
        let tiles = vec![TileType::Dirt; width * width];
        Level {
            width,
            tiles: tiles.into_boxed_slice(),
            entities: HashMap::new(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> TileType {
        let index = x + (y * self.width);
        self.tiles[index]
    }

    pub fn set(&mut self, x: usize, y: usize, ty: TileType) {
        let index = x + (y * self.width);
        self.tiles[index] = ty;
    }
}

pub struct Entity {
    pub(crate) pos: Vec2,
    pub(crate) ty: EntityType,
}

pub enum EntityType {
    Player(PlayerData),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EntityID(usize);

impl EntityID {
    pub fn new() -> EntityID {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        EntityID(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}
