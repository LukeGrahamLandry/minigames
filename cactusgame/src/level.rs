use crate::{get_time, log };
use crate::math::{Shape, Vec2};
use crate::physics::{Thing, World};
use crate::save::Template;
use crate::templates::TEMPLATES;

#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub texture: Texture,
    pub info: EntityInfo,
    pub template_index: usize
}

#[derive(Debug, Clone)]
pub enum EntityInfo {
    None,
    CantMove,
    Deleted,
    Balloon,
    Player {
        touching_wall: bool,
        next_jump_time: f32
    },
    Wind {
        force: Vec2
    },
}

#[derive(Debug)]
pub struct Level {
    pub entities: Vec<Entity>,
    pub physics: World,
    pub last_time: f32,
    pub time_accumulator: f32,
    pub input: Keys,
    pub player: usize,  // TODO: include in save file
}

const TICK_RATE: f32 = 20.0 / 1000.0;

impl Level {
    pub const fn empty() -> Level {
        Level {
            entities: vec![],
            physics: World { things: vec![], pairs: vec![], unique_pairs: vec![], wants_callback: vec![] },
            last_time: 0.0,
            time_accumulator: 0.0,
            input: Keys {
                up: false,
                left: false,
                right: false,
            },
            player: 0,
        }
    }

    pub fn frame_update(&mut self) {
        let now = get_time();
        let dt = now - self.last_time;
        self.last_time = now;
        if dt > 1.0 {  // Pausing when unfocused should be preventing this.
            // log!("dt={dt} WTF!");  // I'd log this but it's 30 KB somehow.
            return;
        }
        self.time_accumulator = (self.time_accumulator + dt).min(0.2);

        while self.time_accumulator > TICK_RATE {
            self.time_accumulator -= TICK_RATE;
            self.physics.tick(TICK_RATE);
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        for p in 0..self.physics.wants_callback.len() {
            let pair = self.physics.wants_callback[p];
            let layers = self.physics.things[pair.b].my_layers;
            match &mut self.entities[pair.a].info {
                EntityInfo::Balloon => {
                    if layers & layer::HURT_BALLOON != 0 {
                        self.kill(pair.a);
                    }
                }
                EntityInfo::Player { touching_wall, next_jump_time } => {
                    if layers & layer::WALL != 0  && !*touching_wall && *next_jump_time < get_time() {
                        *touching_wall = true;
                    }
                },
                EntityInfo::Wind { force } => {
                    self.physics.things[pair.b].force += force.scale(1.0 / TICK_RATE);
                }
                _ => {}
            }
        }

        const WALK_SPEED: f32 = 15000.0 / TICK_RATE;
        const FLY_SPEED: f32 = 500000.0 / TICK_RATE;
        let player = &mut self.physics.things[self.player];
        if self.input.left {
            player.force.x -= WALK_SPEED;
        }
        if self.input.right {
            player.force.x += WALK_SPEED;
        }
        match &mut self.entities[self.player].info {
            EntityInfo::Player { touching_wall, next_jump_time } => {
                if *touching_wall && self.input.up && *next_jump_time < get_time() {
                    player.force.y += FLY_SPEED;
                    self.input.up = false;
                    *touching_wall = false;
                    *next_jump_time = get_time() + 0.4;
                }
            }
            _ => unreachable!()
        };
        player.velocity.x = player.velocity.x.clamp(-100.0, 100.0);
    }

    pub fn add(&mut self, template: &Template, pos: Vec2) -> usize {
        let id = self.entities.len();
        self.entities.push(template.entity(id));
        self.physics.things.push(template.thing(pos));
        id
    }

    pub fn kill(&mut self, i: usize) {
        self.entities[i].info = EntityInfo::Deleted;
        self.physics.things[i].shape = Shape::NoCollide(Vec2::ZERO);
        self.physics.things[i].watch_layers = 0;
        self.physics.things[i].my_layers = 0;
    }
}

pub fn default_level() -> Level {
    let mut level = Level::empty();
    level.add(&TEMPLATES[1], Vec2::new(50.0, 200.0));
    level.player = 0;
    for i in 0..10 {
        level.add(&TEMPLATES[3], Vec2::new(i as f32 * 50.0, 100.0 + (i as f32 * 10.0)));
    }

    for i in 0..10 {
        let id = level.add(&TEMPLATES[6], Vec2::new(i as f32 * 50.0, 200.0));
        if let EntityInfo::Wind { force }  = &mut level.entities[id].info {
            *force = Vec2::new(1000.0, 0.0);
        } else {
            unreachable!()
        }
    }

    level
}

pub struct Scene {
    pub entities: Vec<usize>,
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Texture {
    Cactus = 0,
    Wall = 1,
    Box = 2,
    Balloon = 3,
    ButtonUp = 4,
    ButtonDown = 5,
    Delete = 6,
    Wind = 7,
    Portal = 8,
    Fan = 9,
}

#[derive(Debug)]
pub struct Keys {
    pub up: bool,
    pub left: bool,
    pub right: bool
}

pub mod layer {
    // Specific objects
    pub const CACTUS: u32 = 1 << 1;
    pub const BALLOON: u32 = 1 << 2;
    pub const WALL: u32 = 1 << 5;

    // Things with this are pointy and will kill the balloon.
    pub const HURT_BALLOON: u32 = 1 << 3;

    // Things with this can press buttons.
    pub const OBJECT: u32 = 1 << 4;

    // If both thing have this, they will repel on collision to avoid overlapping.
    pub const DO_RESOLVE: u32 = 1 << 6;
}
