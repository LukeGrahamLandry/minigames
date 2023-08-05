use crate::level::{Entity, EntityInfo, Level, Texture};
use crate::log;
use crate::math::{Shape, Vec2};
use crate::physics::{Material, Thing};
use crate::templates::TEMPLATES;

const VERSION: u16 = 4;
const MIN_VERSION: u16 = 4;
const MAGIC: u8 = 123;

pub struct Template {
    pub texture: Texture,
    pub shape: Shape,
    pub material: Material,
    pub my_layers: u32,
    pub watch_layers: u32,
    pub index: usize,
    pub info: EntityInfo,
}

impl Template {
    pub fn entity(&self, id: usize) -> Entity {
        Entity { id, texture: self.texture, info: self.info.clone(), template_index: self.index }
    }

    pub fn thing(&self, pos: Vec2) -> Thing {
        let mut t = Thing {
            shape: self.shape,
            material: self.material,
            velocity: Vec2::ZERO,
            force: Vec2::ZERO,
            mass_inv: self.material.mass_inv(&self.shape),
            my_layers: self.my_layers,
            watch_layers: self.watch_layers,
        };
        t.offset(pos);
        t
    }
}

pub fn write_level(level: &Level) -> Box<[u8]> {
    let mut data = Writer::new();

    data.u16(VERSION);
    data.u16(level.entities.len() as u16);
    for entity in &level.entities {
        data.u8(MAGIC);

        data.u16(entity.template_index as u16);
        let thing = &level.physics.things[entity.id];
        data.vec2(thing.shape.position());

        if has_physics(&entity.info) {
            data.vec2(thing.velocity);
            data.vec2(thing.force);
        }

        if let EntityInfo::Wind { force } = &entity.info {
            data.vec2(*force);
        }
    }

    data.u8(MAGIC);
    data.data.into_boxed_slice()
}

fn has_physics(info: &EntityInfo) -> bool {
    !matches!(info, EntityInfo::CantMove | EntityInfo::Wind { .. } )
}

#[derive(Debug)]
pub enum LevelError {
    InvalidVersion(u16),
    MissingMagic,
    TooManyBytes
}

pub fn read_level(bytes: &[u8]) -> Result<Level, LevelError> {
    let mut data = Reader { data: bytes, i: 0 };
    let mut level = Level::empty();
    let version = data.u16();
    if version > VERSION || version < MIN_VERSION {
        return Err(LevelError::InvalidVersion(version));
    }

    let count = data.u16();
    for _ in 0..count {
        if data.u8() != MAGIC {
            return Err(LevelError::MissingMagic);
        }

        let template = data.u16() as usize;
        let pos = data.vec2();
        let id = level.add(&TEMPLATES[template], pos);

        if has_physics(&level.entities[id].info) {
            level.physics.things[id].velocity = data.vec2();
            level.physics.things[id].force = data.vec2();
        }

        if let EntityInfo::Wind { force } = &mut level.entities[id].info {
            *force = data.vec2();
        }
    }

    if data.u8() != MAGIC {
        return Err(LevelError::MissingMagic);
    }

    if data.i != data.data.len() {
        return Err(LevelError::TooManyBytes);
    }

    Ok(level)
}

pub struct Reader<'a> {
    data: &'a [u8],
    i: usize
}

impl<'a> Reader<'a> {
    fn u8(&mut self) -> u8 {
        self.i += 1;
        self.data[self.i - 1]
    }

    fn u16(&mut self) -> u16 {
        self.i += 2;
        u16::from_be_bytes([self.data[self.i - 2], self.data[self.i - 1]])
    }

    fn i16(&mut self) -> i16 {
        self.i += 2;
        i16::from_be_bytes([self.data[self.i - 2], self.data[self.i - 1]])
    }

    fn f32(&mut self) -> f32 {
        (self.i16() as f32) / 10.0
    }

    fn vec2(&mut self) ->Vec2 {
        Vec2::new(self.f32(), self.f32())
    }
}

pub struct Writer {
    data: Vec<u8>,
}

impl Writer {
    fn new() -> Writer {
        Writer { data: Vec::new() }
    }

    fn u8(&mut self, x: u8) {
        self.data.push(x);
    }

    fn u16(&mut self, x: u16) {
        self.data.extend(x.to_be_bytes().iter());
    }

    fn i16(&mut self, x: i16) {
        self.data.extend(x.to_be_bytes().iter());
    }

    fn f32(&mut self, x: f32) {
        let v = x.clamp((i16::MIN / 10) as f32, (i16::MAX / 10) as f32);
        self.i16((v * 10.0) as i16);
    }

    fn vec2(&mut self, x: Vec2) {
        self.f32(x.x);
        self.f32(x.y);
    }
}

// TODO: this might fail now since floats are lossy i16/10
#[test]
fn level_serialization(){
    let level_1 = crate::level::default_level();
    let data_1 = write_level(&level_1);
    let level_2 = read_level(&data_1).unwrap();
    let data_2 = write_level(&level_2);
    assert_eq!(data_1, data_2);
}
