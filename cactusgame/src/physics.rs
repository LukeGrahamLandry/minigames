use crate::level::layer;
use crate::math::{AABB, Shape, Vec2};

#[derive(Debug)]
pub struct World {
    pub things: Vec<Thing>,
    pub pairs: Vec<Pair>,
    pub unique_pairs: Vec<Pair>,
    pub wants_callback: Vec<Pair>
}

impl World {
    pub const GRAVITY: f32 = -150.0;

    pub fn tick(&mut self, dt: f32) {
        for t in &mut self.things {
            let mut accel = t.force.scale(t.mass_inv);
            accel.y += World::GRAVITY * t.material.gravity_scale;
            t.velocity += accel.scale(dt);
            t.velocity.x = t.velocity.x.clamp(-t.material.max_vel, t.material.max_vel);
            t.velocity.y = t.velocity.y.clamp(-t.material.max_vel, t.material.max_vel);
            t.offset(t.velocity.scale(dt));
            t.force = Vec2::ZERO;

        }

        self.wants_callback.clear();
        self.broad_phase();
        if !self.pairs.is_empty() {
            self.deduplicate_collisions();
            // Each collision is here twice so the check for wants_callback will be done in both directions.
            for pair in &self.unique_pairs {
                let a = self.things[pair.a].shape.get_aabb();
                let b = self.things[pair.b].shape.get_aabb();
                if let Some(hit) = aabb_vs_aabb(&a, &b) {
                    assert_ne!(pair.a, pair.b, "alias safety");
                    let b_t = unsafe { &mut *((&mut self.things[pair.b]) as *mut Thing) };
                    let a_t = &mut self.things[pair.a];

                    if (a_t.my_layers & b_t.my_layers & layer::DO_RESOLVE) != 0 {
                        resolve_collision(a_t, b_t, hit, dt);
                        positional_correction(a_t, b_t, hit);
                    }
                    if a_t.watch_layers & b_t.my_layers != 0 {
                        self.wants_callback.push(*pair);
                    }
                }
            }
        }
    }

    fn broad_phase(&mut self) {
        self.pairs.clear();
        for i in 0..self.things.len() {
            for j in 0..self.things.len() {
                if i == j {
                    continue;
                }

                if self.things[i].mass_inv == 0.0 && self.things[j].mass_inv == 0.0 {
                    continue;  // Both have infinite mass.
                }

                let a = self.things[i].shape.get_aabb();
                let b = self.things[j].shape.get_aabb();
                if a.collides_aabb(&b) {
                    self.pairs.push(Pair { a: i, b: j});
                }
            }
        }
    }

    fn deduplicate_collisions(&mut self) {
        assert!(!self.pairs.is_empty());
        self.unique_pairs.clear();
        self.pairs.sort_unstable();
        let mut iter = self.pairs.iter();
        self.unique_pairs.push(*iter.next().unwrap());
        for pair in iter {
            if pair != self.unique_pairs.last().unwrap() {
                self.unique_pairs.push(*pair);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Thing {
    pub shape: Shape,
    pub material: Material,
    pub velocity: Vec2,
    pub force: Vec2,  // TODO
    pub mass_inv: f32,
    pub my_layers: u32,
    pub watch_layers: u32,
}

impl Thing {
    pub fn new(shape: Shape, material: Material) -> Thing {
        Thing {
            shape,
            material,
            velocity: Vec2::ZERO,
            force: Vec2::ZERO,
            mass_inv: material.mass_inv(&shape),
            my_layers: 0,
            watch_layers: 0,
        }
    }

    pub fn offset(&mut self, delta: Vec2) {
        match &mut self.shape {
            Shape::Rect(aabb) => {
                aabb.min += delta;
                aabb.max += delta;
            },
            Shape::NoCollide(pos) => {
                *pos += delta;
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pair {
    pub a: usize,
    pub b: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub density: f32,
    pub restitution: f32,
    pub gravity_scale: f32,
    pub max_vel: f32
}

impl Material {
    pub fn mass_inv(&self, shape: &Shape) -> f32 {
        if self.density == 0.0 {
            0.0  // Infinite mass.
        } else {
            1.0 / (shape.volume() * self.density)
        }
    }
}

impl Material {
    pub const PLAYER: Material = Material { density: 0.5, restitution: 0.8, gravity_scale: 4.0, max_vel: 1000.0 };
    pub const WALL: Material = Material { density: 0.0, restitution: 0.2, gravity_scale: 0.0, max_vel: 0.0 };
    pub const CUBE: Material = Material { density: 0.3, restitution: 1.25, gravity_scale: 1.0, max_vel: 1000.0 };
    pub const BALLOON: Material = Material { density: 0.1, restitution: 1.5, gravity_scale: 0.1, max_vel: 100.0 };
    pub const NONE: Material = Material { density: 0.0, restitution: 0.0, gravity_scale: 0.0, max_vel: 0.0 };
}

#[derive(Copy, Clone, Debug)]
struct Hit {
    overlap: f32,
    normal: Vec2
}

// TODO: is this supposed to be multiplying by dt?
fn resolve_collision(a: &mut Thing, b: &mut Thing, hit: Hit, _dt: f32) {
    let rel_vel = b.velocity - a.velocity;
    let vel_n = rel_vel.dot(hit.normal);
    if vel_n > 0.0 { return; }

    let e = a.material.restitution.min(b.material.restitution);

    let magnitude = (-(1.0 + e) * vel_n) / (a.mass_inv + b.mass_inv);
    let impulse = hit.normal.scale(magnitude);
    // log!("impulse={:?} magnitude={magnitude} vel_n={vel_n} \n hit.normal={:?}\n rel_vel={:?}", impulse, hit.normal, rel_vel);
    a.velocity -= impulse.scale(a.mass_inv);
    b.velocity += impulse.scale(b.mass_inv);
}

// Note this isn't multiplying by dt. It just fixes error proportional to the overlap amount.
fn positional_correction(a: &mut Thing, b: &mut Thing, hit: Hit) {
    let percent = 0.5;
    let threshold = 0.1;
    let magnitude = (hit.overlap - threshold).max(0.0) / (a.mass_inv + b.mass_inv) * percent;
    a.offset(hit.normal.scale(-1.0 * magnitude * a.mass_inv));
    b.offset(hit.normal.scale(magnitude * b.mass_inv));
}

fn aabb_vs_aabb(a: &AABB, b: &AABB) -> Option<Hit> {
    let n = b.min - a.min;
    let a_extent = (a.max.x - a.min.x) / 2.0;
    let b_extent = (b.max.x - b.min.x) / 2.0;
    let x_overlap = a_extent + b_extent - n.x.abs();

    // SAT test on x axis
    if x_overlap <= 0.0 {
        return None;
    }

    let a_extent = (a.max.y - a.min.y) / 2.0;
    let b_extent = (b.max.y - b.min.y) / 2.0;
    let y_overlap = a_extent + b_extent - n.y.abs();

    // SAT test on y axis
    if y_overlap <= 0.0 {
        return None;
    }

    Some(if x_overlap < y_overlap {  // TODO: > was flipped?
        // Point towards B knowing that n points from A to B
        let normal = if n.x < 0.0 {
            Vec2::new(-1.0, 0.0)
        } else {
            // (0, 0) must have been a typo
            Vec2::new(1.0, 0.0)
        };
        Hit {
            overlap: x_overlap,
            normal
        }
    } else {
        // Point toward B knowing that n points from A to B
        let normal = if n.y < 0.0 {
            Vec2::new(0.0, -1.0)
        } else {
            Vec2::new(0.0, 1.0)
        };
        Hit {
            overlap: y_overlap,
            normal
        }
    })
}

/* I learned how to physics from https://github.com/RandyGaul/ImpulseEngine
 * (The license also had a link to their website but I've omitted it since the domain expired and now it's just porn)
 *  Copyright (c) 2013 Randy Gaul
 *
 *  This software is provided 'as-is', without any express or implied
 *  warranty. In no event will the authors be held liable for any damages
 *  arising from the use of this software.
 *
 *  Permission is granted to anyone to use this software for any purpose,
 *  including commercial applications, and to alter it and redistribute it
 *  freely, subject to the following restrictions:
 *    1. The origin of this software must not be misrepresented; you must not
 *       claim that you wrote the original software. If you use this software
 *       in a product, an acknowledgment in the product documentation would be
 *       appreciated but is not required.
 *    2. Altered source versions must be plainly marked as such, and must not be
 *       misrepresented as being the original software.
 *    3. This notice may not be removed or altered from any source distribution.
 */
