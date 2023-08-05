use core::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    NoCollide(Vec2),
    Rect(AABB),
}

impl Shape {
    pub fn volume(self) -> f32 {
        match self {
            Shape::Rect(aabb) => {
                let width = aabb.max.x - aabb.min.x;
                let height = aabb.max.y - aabb.min.y;
                width * height
            },
            Shape::NoCollide(_) => 0.0,
        }
    }

    pub fn get_aabb(&self) -> AABB {
        match self {
            Shape::Rect(aabb) => *aabb,
            Shape::NoCollide(_) => AABB::square(Vec2::ZERO, 0.0),
        }
    }

    pub fn position(&self) -> Vec2 {
        match self {
            Shape::Rect(aabb) => aabb.min,
            Shape::NoCollide(pos) => *pos,
        }
    }

    pub fn overlaps_point(&self, pos: Vec2) -> bool {
        let aabb = AABB::square(pos, 1.0);
        self.get_aabb().collides_aabb(&aabb)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn square(pos: Vec2, size: f32) -> AABB {
        AABB { min: pos, max: Vec2::new(pos.x + size, pos.y + size) }
    }

    pub fn collides_aabb(&self, other: &AABB) -> bool {
        !(self.max.x < other.min.x || self.min.x > other.max.x || self.max.y < other.min.y || self.min.y > other.max.y)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn dot(self, other: Vec2) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn len_sq(self) -> f32 {
        self.dot(self)
    }

    pub fn normalize(self) -> Vec2 {
        self.scale(1.0 / self.len_sq().sqrt())
    }

    pub fn scale(self, w: f32) -> Vec2 {
        Vec2::new(self.x * w, self.y * w)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
