use crate::level::{EntityInfo, layer, Texture};
use crate::math::{AABB, Shape, Vec2};
use crate::physics::{Material};
use crate::save::Template;

pub const TEMPLATES: &[Template] = &[
    Template {
        texture: Texture::Delete,
        shape: Shape::Rect(AABB { min: Vec2::ZERO, max: Vec2::new(50.0, 50.0) }),
        material: Material::NONE,
        my_layers: 0,
        watch_layers: 0,
        index: 0,
        info: EntityInfo::CantMove,
    },
    Template {
        texture: Texture::Cactus,
        shape: Shape::Rect(AABB { min: Vec2::ZERO, max: Vec2::new(50.0, 50.0) }),
        material: Material::PLAYER,
        my_layers: layer::CACTUS | layer::OBJECT | layer::DO_RESOLVE | layer::HURT_BALLOON,
        watch_layers: layer::WALL,
        index: 1,
        info: EntityInfo::Player {
            touching_wall: false,
            next_jump_time: 0.0,
        },
    },
    Template {
        texture: Texture::Balloon,
        shape: Shape::Rect(AABB { min: Vec2::ZERO, max: Vec2::new(50.0, 50.0) }),
        material: Material::BALLOON,
        my_layers: layer::BALLOON | layer::OBJECT | layer::DO_RESOLVE,
        watch_layers: layer::HURT_BALLOON,
        index: 2,
        info: EntityInfo::Balloon,
    },
    Template {
        texture: Texture::Wall,
        shape: Shape::Rect(AABB { min: Vec2::ZERO, max: Vec2::new(50.0, 50.0) }),
        material: Material::WALL,
        my_layers: layer::WALL | layer::DO_RESOLVE,
        watch_layers: 0,
        index: 3,
        info: EntityInfo::CantMove,
    },
    Template {
        texture: Texture::Box,
        shape: Shape::Rect(AABB { min: Vec2::ZERO, max: Vec2::new(50.0, 50.0) }),
        material: Material::CUBE,
        my_layers: layer::OBJECT | layer::DO_RESOLVE,
        watch_layers: 0,
        index: 4,
        info: EntityInfo::None,
    },
    Template {
        texture: Texture::ButtonUp,
        shape: Shape::Rect(AABB { min: Vec2::new(10.0, 15.0), max: Vec2::new(30.0, 20.0) }),
        material: Material::NONE,
        my_layers: 0,
        watch_layers: layer::OBJECT,
        index: 5,
        info: EntityInfo::CantMove,
    },
    Template {
        texture: Texture::Wind,
        shape: Shape::Rect(AABB { min: Vec2::ZERO, max: Vec2::new(50.0, 50.0) }),
        material: Material::NONE,
        my_layers: 0,
        watch_layers: layer::BALLOON,
        index: 6,
        info: EntityInfo::Wind {
            force: Vec2::ZERO,
        },
    },
    Template {
        texture: Texture::Fan,
        shape: Shape::Rect(AABB { min: Vec2::ZERO, max: Vec2::new(50.0, 50.0) }),
        material: Material::NONE,
        my_layers: 0,
        watch_layers: 0,
        index: 7,
        info: EntityInfo::CantMove,
    },
    Template {
        texture: Texture::Portal,
        shape: Shape::Rect(AABB { min: Vec2::ZERO, max: Vec2::new(50.0, 50.0) }),
        material: Material::NONE,
        my_layers: 0,
        watch_layers: layer::BALLOON,
        index: 8,
        info: EntityInfo::CantMove,
    },
];
