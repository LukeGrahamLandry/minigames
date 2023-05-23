use crate::config::*;
use nannou::prelude::*;

#[derive(Clone)]
pub struct World {
    pub player: Player,
    pub farm: Farm,
    pub shops: Vec<Shop>,
    pub cash: usize,
    pub truck: Truck,
    pub round_timer: f32,
    pub round_number: usize,
}

#[derive(Clone)]
pub struct Farm {
    pub crops: Vec<Crop>,
    pub area: Rect,
}

#[derive(Clone)]
pub struct Crop {
    pub kind: &'static CropType,
    pub timer: f32,
    pub area: Rect,
}

#[derive(Clone)]
pub struct Shop {
    pub contents: InvItem,
    pub area: Rect,
    pub price: usize,
}

#[derive(Clone)]
pub struct Player {
    pub pos: Vec2,
    pub dir: Vec2,
    pub inventory: InvItem,
}

#[derive(Clone)]
pub struct Truck {
    pub area: Rect,
    pub wants: Vec<&'static CropType>,
}

#[derive(Copy, Clone)]
pub enum InvItem {
    None,
    Seed(&'static CropType),
    Crop(&'static CropType),
}

impl World {
    pub fn new() -> World {
        let mut w = World {
            player: Player {
                pos: Vec2::new(-200.0, 200.0),
                dir: Default::default(),
                inventory: InvItem::None,
            },
            farm: Farm {
                crops: vec![],
                area: Rect::from_xy_wh(Point2::new(-150.0, 0.0), Vec2::new(400.0, 400.0)),
            },
            shops: vec![],
            cash: 5,
            truck: Truck {
                area: Rect::from_xy_wh(Point2::new(-200.0, 300.0), Vec2::new(200.0, 100.0)),
                wants: vec![],
            },
            round_number: 0,
            round_timer: 0.0,
        };

        for (i, crop) in CROPS.iter().enumerate() {
            w.shops.push(Shop {
                contents: InvItem::Seed(crop),
                area: Rect::from_xy_wh(
                    Point2::new(-400.0, (i as f32 * CROP_SIZE * -2.0) + 180.0),
                    Vec2::new(CROP_SIZE, CROP_SIZE),
                ),
                price: crop.seed_price,
            });
        }
        w.start_round();

        w
    }

    pub fn update(&mut self, dt: f32) {
        self.farm.update(dt);
        self.player.pos += self.player.dir * dt * PLAYER_SPEED;
        self.round_timer -= dt;
    }

    pub fn game_over(&self) -> bool {
        self.round_timer < 0.0
    }

    pub fn interact(&mut self) {
        match self.player.inventory {
            InvItem::None => {
                for shop in &self.shops {
                    if self.player.collides(&shop.area) && self.cash >= shop.price {
                        self.cash -= shop.price;
                        self.player.inventory = shop.contents;
                        return;
                    }
                }

                self.farm.try_harvest(&mut self.player);
            }
            InvItem::Seed(kind) => {
                let on_farm = self.player.collides(&self.farm.area);
                if on_farm && self.farm.available_space(self.player.pos) {
                    self.farm.crops.push(Crop::new(kind, self.player.pos));
                    self.player.inventory = InvItem::None;
                }
            }
            InvItem::Crop(kind) => {
                if self.player.collides(&self.truck.area) {
                    let remove_index = self
                        .truck
                        .wants
                        .iter()
                        .enumerate()
                        .find(|(_, crop)| **crop == kind)
                        .map(|(i, _)| i);

                    if let Some(i) = remove_index {
                        self.truck.wants.remove(i);
                        self.cash += kind.sale_price;
                        if self.truck.wants.is_empty() {
                            self.start_round();
                        }
                    }

                    self.player.inventory = InvItem::None;
                }
            }
        }
    }

    fn start_round(&mut self) {
        // Extra cash for finishing quickly.
        self.cash += (self.round_timer / 10.0) as usize + 1;

        let round_time = BASE_ROUND_TIMER - (self.round_number as f32 * ROUND_TIME_DECREMENT);
        self.round_timer = round_time.max(MIN_ROUND_TIMER);
        self.round_number += 1;

        // TODO: less unfair level design
        for _ in 0..random_range(REQUEST_COUNT.0, REQUEST_COUNT.1) {
            let index = random_range(0, 3);
            if self.cash >= CROPS[index].seed_price {
                self.truck.wants.push(&CROPS[index]);
            }
        }
    }
}

impl Farm {
    fn update(&mut self, dt: f32) {
        for crop in &mut self.crops {
            crop.timer += dt;
        }

        self.crops.retain(|crop| !crop.is_overripe());
    }

    /// If the player is over a ripe crop, remove it and put in in their inventory.
    fn try_harvest(&mut self, player: &mut Player) {
        assert!(matches!(player.inventory, InvItem::None)); // TODO: change when i have watering can, etc.
        if !player.collides(&self.area) {
            return;
        }
        let pickup_index = self
            .crops
            .iter()
            .enumerate()
            .find(|(_, crop)| crop.is_ripe() && player.collides(&crop.area))
            .map(|(i, crop)| {
                player.inventory = InvItem::Crop(crop.kind);
                i
            });

        if let Some(i) = pickup_index {
            self.crops.swap_remove(i);
        }
    }

    fn available_space(&self, pos: Vec2) -> bool {
        for crop in &self.crops {
            if crop.area.contains(pos) {
                return false;
            }
        }
        true
    }
}

impl Crop {
    fn new(kind: &'static CropType, pos: Vec2) -> Crop {
        Crop {
            kind,
            timer: 0.0,
            area: Crop::rect(pos),
        }
    }

    pub fn is_ripe(&self) -> bool {
        self.timer > self.kind.growth_time
    }

    fn is_overripe(&self) -> bool {
        self.timer > (self.kind.growth_time * 2.0)
    }

    pub fn rect(pos: Vec2) -> Rect {
        Rect::from_xy_wh(pos, Vec2::new(CROP_SIZE, CROP_SIZE))
    }
}

impl Player {
    fn collides(&self, area: &Rect) -> bool {
        area.contains(self.pos)
    }

    pub fn area(&self) -> Rect {
        Rect::from_xy_wh(self.pos, Vec2::new(CROP_SIZE, CROP_SIZE))
    }
}
