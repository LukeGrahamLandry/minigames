use std::mem;

use bracket_lib::prelude::*;
use rand::random;

#[derive(Clone)]
pub struct World {
    pub player: Snake,
    pub food: Vec<Food>,
    pub size: Point,
    pub computer_snakes: Vec<Snake>,
    enemy_size: usize,
}

#[derive(Clone)]
pub struct Snake {
    pub pos: PointF,
    pub dir: Direction,
    pub body: Vec<PointF>,
}

#[derive(Clone, Debug)]
pub struct Food {
    pub pos: PointF,
}

const PLAYER_SPEED: f32 = 0.75;
const FOOD_CHANCE: f32 = 0.01;
const COMPUTER_SNAKE_CHANCE: f32 = 0.01;
const SNAKE_DROP_FOOD_CHANCE: f32 = 0.3;

impl World {
    pub fn new(size: Point) -> World {
        let mut w = World {
            player: Snake {
                pos: PointF::new((size.x / 2) as f32, (size.y / 2) as f32 + 2.0),
                dir: Direction::Right,
                body: vec![],
            },
            food: vec![],
            computer_snakes: vec![],
            size,
            enemy_size: 0,
        };

        w.spawn_food();
        w
    }

    pub fn update(&mut self, dt: f32) -> GameResult {
        if random::<f32>() < (FOOD_CHANCE * dt) {
            self.spawn_food();
        }

        if random::<f32>() < (COMPUTER_SNAKE_CHANCE * dt) {
            let mut pos = self.food[random::<usize>() % self.food.len()].pos;
            pos.y = 0.0;
            let mut s = Snake {
                pos,
                dir: Direction::Down,
                body: vec![],
            };
            for _ in 0..self.enemy_size {
                s.grow();
            }
            self.computer_snakes.push(s);
        }

        self.update_computers(dt);
        self.player.movement(dt);
        for food in &mut self.food {
            if length_sqr(food.pos - self.player.pos) < 1.0 {
                food.pos = rand_pos(self.size);
                self.player.grow();
                self.enemy_size += 1;
            }
        }

        if self.should_die(&self.player, true) {
            GameResult::Lose
        } else {
            GameResult::Continue
        }
    }

    pub fn spawn_food(&mut self) {
        self.food.push(Food {
            pos: rand_pos(self.size),
        })
    }

    fn should_die(&self, snake: &Snake, die_on_bottom: bool) -> bool {
        // check player tail
        for part in &self.player.body {
            if length_sqr(*part - snake.pos) < 0.5 {
                return true;
            }
        }

        for other_snake in &self.computer_snakes {
            for part in &other_snake.body {
                if length_sqr(*part - snake.pos) < 0.5 {
                    return true;
                }
            }
        }

        // check screen edge
        snake.pos.x < -1.0
            || snake.pos.x > self.size.x as f32
            || snake.pos.y < -1.0
            || (die_on_bottom && snake.pos.y > self.size.y as f32)
    }

    fn update_computers(&mut self, dt: f32) {
        (0..self.computer_snakes.len()).rev().for_each(|i| {
            self.computer_snakes[i].movement(dt);
            for food in &mut self.food {
                if length_sqr(food.pos - self.computer_snakes[i].pos) < 1.0 {
                    food.pos = rand_pos(self.size);
                    self.computer_snakes[i].grow();
                    self.enemy_size += 1;
                }
            }
            if self.should_die(&self.computer_snakes[i], false) {
                for pos in &self.computer_snakes[i].body {
                    // Since new body parts are put at (-1, -1), we check that the pos is on screen before putting food there.
                    if random::<f32>() < SNAKE_DROP_FOOD_CHANCE && pos.x > 0.0 && pos.y > 0.0 {
                        self.food.push(Food { pos: *pos });
                    }
                }

                self.computer_snakes.remove(i);
            }
        });

        // Since they don't die on hitting the bottom, remove them if the whole body is off the screen.
        self.computer_snakes
            .retain(|snake| snake.pos.y < self.size.y as f32 + self.enemy_size as f32)
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
        self.body.push(PointF::new(-1.0, -1.0));
    }
}

#[derive(Eq, PartialEq)]
pub enum GameResult {
    Continue,
    Lose,
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
