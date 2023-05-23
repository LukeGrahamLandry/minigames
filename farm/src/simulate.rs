use crate::config::*;
use nannou::prelude::*;
use std::mem;

#[derive(Clone)]
pub struct World {
    pub player: Player,
    pub farm: Farm,
    pub shops: Vec<Shop>,
    pub shelves: Vec<Shelf>,
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
    pub needs_water: bool,
    water_timer: f32,
}

#[derive(Clone)]
pub struct Shop {
    pub contents: InvItem,
    pub area: Rect,
    pub price: usize,
}

#[derive(Clone)]
pub struct Shelf {
    pub contents: InvItem,
    pub area: Rect,
    pub crow_hp: usize,
    crow_timer: f32,
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
    WateringCan,
    CrowBaton,
}

// TODO: minigame for gold where you click targets with the mouse if you dont use the interact button for a few seconds

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
            shelves: vec![],
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
        for (i, item) in STARTING_SHELVES.iter().enumerate() {
            w.shelves.push(Shelf {
                contents: *item,
                area: Rect::from_xy_wh(
                    Point2::new((i as f32 * CROP_SIZE * 2.0) - 325.0, 225.0),
                    Vec2::new(CROP_SIZE, CROP_SIZE),
                ),
                crow_hp: 0,
                crow_timer: 0.0,
            });
        }

        w.start_round();

        w
    }

    pub fn update(&mut self, dt: f32) {
        self.farm.update(dt, self.round_number);
        self.update_shelves(dt);
        self.player.pos += self.player.dir * dt * PLAYER_SPEED;
        self.round_timer -= dt;
    }

    fn update_shelves(&mut self, dt: f32) {
        let crow_chance = CROW_CHANCE + (self.round_number as f32 * CROW_CHANCE_SCALING);
        for shelf in &mut self.shelves {
            if shelf.crow_hp > 0 {
                shelf.crow_timer -= dt;
                if shelf.crow_timer < 0.0 {
                    shelf.contents = InvItem::None;
                    shelf.crow_hp = 0;
                }
            } else if !shelf.is_empty() && random_f32() < crow_chance * dt {
                shelf.crow_hp = CROW_MAX_HP;
                shelf.crow_timer = CROW_TIME;
            }
        }
    }

    pub fn game_over(&self) -> bool {
        self.round_timer < 0.0
    }

    pub fn interact(&mut self) {
        if self.try_interact_shelf() {
            return;
        }

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
                    let remove_index = self.truck.wants.iter().position(|crop| *crop == kind);

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
            InvItem::WateringCan => {
                self.farm.try_water(self.player.pos);
            }
            InvItem::CrowBaton => {} // handled by try_interact_shelf
        }
    }

    fn try_interact_shelf(&mut self) -> bool {
        for shelf in &mut self.shelves {
            if self.player.collides(&shelf.area) {
                if shelf.crow_hp > 0 {
                    let damage = if matches!(self.player.inventory, InvItem::CrowBaton) {
                        3
                    } else {
                        1
                    }
                    .min(shelf.crow_hp);
                    shelf.crow_hp -= damage;
                } else {
                    mem::swap(&mut self.player.inventory, &mut shelf.contents);
                }

                return true;
            }
        }
        false
    }

    fn start_round(&mut self) {
        // Extra cash for finishing quickly.
        self.cash += (self.round_timer / 5.0) as usize;

        let round_time = BASE_ROUND_TIMER - (self.round_number as f32 * ROUND_TIME_DECREMENT);
        self.round_timer = round_time.max(MIN_ROUND_TIMER);
        self.round_number += 1;

        for _ in 0..random_range(REQUEST_COUNT.0, REQUEST_COUNT.1) {
            let index = random_range(0, if self.round_number >= 5 { 3 } else { 2 });
            self.truck.wants.push(&CROPS[index]);
        }

        if self.round_number >= 10 && random() {
            self.truck.wants.push(&CROPS[3]);
        }
    }
}

impl Farm {
    fn update(&mut self, dt: f32, round_number: usize) {
        let drought_chance = DROUGHT_CHANCE + (round_number as f32 * DROUGHT_CHANCE_SCALING);
        let drought = random_f32() < (drought_chance * dt);
        for crop in &mut self.crops {
            crop.timer += dt;
            if crop.needs_water {
                crop.water_timer -= dt;
            } else if drought {
                crop.needs_water = true;
                crop.water_timer = WATER_TIME;
            }
        }

        self.crops.retain(|crop| !crop.should_die());
    }

    /// If the player is over a ripe crop, remove it and put in in their inventory.
    fn try_harvest(&mut self, player: &mut Player) {
        assert!(matches!(player.inventory, InvItem::None));
        if !player.collides(&self.area) {
            return;
        }
        let pickup_index = self
            .crops
            .iter()
            .position(|crop| crop.is_ripe() && player.collides(&crop.area));

        if let Some(i) = pickup_index {
            let crop = self.crops.swap_remove(i);
            player.inventory = InvItem::Crop(crop.kind);
        }
    }

    fn try_water(&mut self, pos: Vec2) {
        if !self.area.contains(pos) {
            return;
        }

        for crop in &mut self.crops {
            if crop.needs_water && crop.area.contains(pos) {
                crop.needs_water = false;
                return;
            }
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
            needs_water: false,
            water_timer: 0.0,
        }
    }

    pub fn is_ripe(&self) -> bool {
        self.timer > self.kind.growth_time
    }

    fn should_die(&self) -> bool {
        self.timer > (self.kind.growth_time * 2.0) || (self.needs_water && self.water_timer < 0.0)
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

impl Shelf {
    fn is_empty(&self) -> bool {
        matches!(self.contents, InvItem::None)
    }
}
