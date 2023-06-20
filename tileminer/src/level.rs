use crate::player::PlayerData;
use crate::render::ScreenTransform;
use nannou::geom::{Rect, Vec2};
use nannou::state::{Keys, Mouse};
use nannou::winit::event::VirtualKeyCode;
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
    cringe_scratch_buffer: Vec<(EntityID, Vec2, bool, Vec2)>,
    pub mouse_pos: (usize, usize),
}

impl Level {
    pub fn new(width: usize) -> Level {
        let tiles = vec![TileType::Dirt; width * width];
        Level {
            size: width,
            tiles: tiles.into_boxed_slice(),
            entities: HashMap::new(),
            wants_jump: false,
            cringe_scratch_buffer: vec![],
            mouse_pos: (0, 0),
        }
    }

    pub fn player_input(&mut self, keys: &Keys, mouse: &Mouse, id: &EntityID, window: Rect) {
        let player = self.entities.get_mut(id).unwrap();
        const JUMP: f32 = 10.0;
        const SPEED: f32 = 10.0;
        if keys.down.contains(&VirtualKeyCode::W) && player.is_on_ground {
            player.velocity.y -= JUMP;
        }
        if keys.down.contains(&VirtualKeyCode::A) {
            player.velocity.x = -SPEED;
        }
        if keys.down.contains(&VirtualKeyCode::D) {
            player.velocity.x = SPEED;
        }

        let mouse_pos_screen = mouse.position();
        let cam = ScreenTransform::new(self, *id, window);
        let mouse_world_pos = cam.pixel_to_tile(mouse_pos_screen);
        self.mouse_pos = (
            mouse_world_pos.0.clamp(0, self.size as isize) as usize,
            mouse_world_pos.1.clamp(0, self.size as isize) as usize,
        );
    }

    pub fn update(&mut self, delta_t: f32) {
        const GRAVITY: f32 = 20.0;
        const FRICTION: f32 = 4.0;
        for entity in self.entities.values_mut() {
            entity.velocity.y += GRAVITY * delta_t;
            entity.pos += entity.velocity * delta_t;
            if entity.is_on_ground {
                entity.velocity.x *= 1.0 - (FRICTION * delta_t);
                entity.is_on_ground = false;
            }
        }

        self.handle_collisions();
    }

    // This probably doesn't work if you're moving faster than a tile per frame but that seems like a non-issue.
    fn handle_collisions(&mut self) {
        for entity in self.entities.values() {
            let mut pos = entity.pos;
            let mut vel = entity.velocity;
            let mut dirty = false;
            let mut ground = false;
            for x in -1..=1 {
                for y in -1..=1 {
                    let check_x = pos.x as isize + x;
                    let check_y = pos.y as isize + y;
                    let tile = self.checked_get(check_x, check_y);
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
                            if pos.x + 1.0 > check_x as f32 && pos.x + 1.0 < check_x as f32 + 1.0 {
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
                            }
                        }
                    }
                }
            }
            if dirty {
                self.cringe_scratch_buffer
                    .push((entity.id, pos, ground, vel));
            }
        }

        for (id, pos, ground, vel) in self.cringe_scratch_buffer.drain(0..) {
            let entity = self.entities.get_mut(&id).unwrap();
            entity.pos = pos;
            entity.is_on_ground = ground;
            entity.velocity = vel;
        }
    }

    pub fn checked_get(&self, x: isize, y: isize) -> TileType {
        if x < 0 || y < 0 || x >= self.size as isize || y >= self.size as isize {
            TileType::Void
        } else {
            self.get(x as usize, y as usize)
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
            is_on_ground: false,
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
    is_on_ground: bool,
}

pub enum EntityType {
    Player(PlayerData),
    Box,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Void,
    Empty,
    Dirt,
}

impl TileType {
    pub fn is_solid(&self) -> bool {
        !matches!(self, TileType::Empty)
    }
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
