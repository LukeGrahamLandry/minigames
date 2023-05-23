use crate::config::{CropType, CROP_SIZE};
use nannou::prelude::*;

pub struct World {
    player: Player,
    farm: Farm,
    shops: Vec<Shop>,
    cash: usize,
}

pub struct Farm {
    crops: Vec<Crop>,
    area: Rect,
}

pub struct Crop {
    kind: &'static CropType,
    timer: f32,
    area: Rect,
}

pub struct Shop {
    contents: InvItem,
    area: Rect,
    price: usize,
}

pub struct Player {
    pos: Vec2,
    dir: Vec2,
    inventory: InvItem,
}

#[derive(Copy, Clone)]
pub enum InvItem {
    None,
    Seed(&'static CropType),
    Crop(&'static CropType),
}

impl World {
    pub fn update(&mut self, dt: f32) {
        self.farm.update(dt);
        self.player.pos += self.player.dir;
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

                self.farm.interact(&mut self.player);
            }
            InvItem::Seed(kind) => {
                let on_farm = self.player.collides(&self.farm.area);
                if on_farm && self.farm.available_space(self.player.pos) {
                    self.farm.crops.push(Crop::new(kind, self.player.pos));
                    self.player.inventory = InvItem::None;
                }
            }
            InvItem::Crop(_) => todo!(),
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
    fn interact(&mut self, player: &mut Player) {
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
            area: Rect::from_xy_wh(pos, Vec2::new(CROP_SIZE, CROP_SIZE)),
        }
    }

    fn is_ripe(&self) -> bool {
        self.timer > self.kind.growth_time
    }

    fn is_overripe(&self) -> bool {
        self.timer > (self.kind.growth_time * 2.0)
    }
}

impl Player {
    fn collides(&self, area: &Rect) -> bool {
        area.contains(self.pos)
    }
}
