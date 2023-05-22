use std::mem;

use bracket_lib::prelude::*;
use rand::random;

pub struct World {
    pub player: Snake,
    pub food: Vec<Food>,
    pub size: Point,
}

pub struct Snake {
    pub pos: PointF,
    pub dir: Direction,
    pub body: Vec<PointF>,
}

pub struct Food {
    pub pos: PointF,
}

const PLAYER_SPEED: f32 = 0.75;
const FOOD_CHANCE: f32 = 0.01;

impl World {
    pub fn new(size: Point) -> World {
        let mut w = World {
            player: Snake {
                pos: PointF::new(0.0, 10.0),
                dir: Direction::Right,
                body: vec![],
            },
            food: vec![],
            size,
        };
        w.spawn_food();
        w
    }

    pub fn update(&mut self, dt: f32) {
        if random::<f32>() < (FOOD_CHANCE * dt) {
            self.spawn_food();
        }

        self.player.movement(dt);

        for food in &mut self.food {
            if length_sqr(food.pos - self.player.pos) < 1.0 {
                food.pos = rand_pos(self.size);
                self.player.grow();
            }
        }
    }

    pub fn spawn_food(&mut self) {
        self.food.push(Food {
            pos: rand_pos(self.size),
        })
    }
}

impl Snake {
    pub fn input(&mut self, dir: Direction) {
        if self.dir != dir.reverse() {
            self.dir = dir;
        }
    }

    fn movement(&mut self, dt: f32) {
        let mut last = self.pos;
        for part in &mut self.body {
            mem::swap(part, &mut last);
        }
        self.pos += self.dir.vec() * PLAYER_SPEED * dt;
    }

    fn grow(&mut self) {
        let last = self.body.last().unwrap_or(&self.pos);
        self.body.push(*last);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn vec(self) -> PointF {
        match self {
            Direction::Left => PointF::new(-1.0, 0.0),
            Direction::Right => PointF::new(1.0, 0.0),
            Direction::Up => PointF::new(0.0, -1.0),
            Direction::Down => PointF::new(0.0, 1.0),
        }
    }

    fn reverse(self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

/// Doesn't include the edges of the bounds.
fn rand_pos(bounds: Point) -> PointF {
    PointF::new(
        (random::<i32>() % (bounds.x - 2)).abs() as f32 + 1.0,
        (random::<i32>() % (bounds.y - 3)).abs() as f32 + 2.0,
    )
}

fn length_sqr(v: PointF) -> f32 {
    v.dot(v)
}
