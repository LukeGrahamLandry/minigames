use crate::simulate::InvItem;
use nannou::color::{GREEN, RED, WHITE, YELLOW};
use nannou::prelude::Srgb;

pub const CROP_SIZE: f32 = 30.0;
pub const PLAYER_SPEED: f32 = 200.0;
pub const BASE_ROUND_TIMER: f32 = 60.0;
pub const MIN_ROUND_TIMER: f32 = 20.0;
pub const ROUND_TIME_DECREMENT: f32 = 3.0;
pub const REQUEST_COUNT: (usize, usize) = (4, 7);
pub const DROUGHT_CHANCE: f32 = 0.03;
pub const DROUGHT_CHANCE_SCALING: f32 = 0.003;
pub const WATER_TIME: f32 = 5.0;
pub const CROW_CHANCE: f32 = 0.05;
pub const CROW_CHANCE_SCALING: f32 = 0.001;
pub const CROW_TIME: f32 = 5.0;
pub const CROW_MAX_HP: usize = 3;
pub const SPRINKLER_RANGE: f32 = CROP_SIZE * 2.0;
pub const STARTING_CASH: usize = 5;

#[derive(PartialEq)]
pub struct CropType {
    pub name: &'static str,
    pub growth_time: f32,
    pub seed_price: usize,
    pub sale_price: usize,
    pub colour: Srgb<u8>,
}

pub const CROPS: [CropType; 4] = [
    CropType {
        name: "tomato",
        growth_time: 5.0,
        seed_price: 1,
        sale_price: 2,
        colour: RED,
    },
    CropType {
        name: "potato",
        growth_time: 15.0,
        seed_price: 3,
        sale_price: 5,
        colour: WHITE,
    },
    CropType {
        name: "pizza",
        growth_time: 25.0,
        seed_price: 15,
        sale_price: 20,
        colour: YELLOW,
    },
    CropType {
        name: "acorn",
        growth_time: 60.0,
        seed_price: 30,
        sale_price: 60,
        colour: GREEN,
    },
];

pub const STARTING_SHELVES: [InvItem; 5] = [
    InvItem::WateringCan,
    InvItem::CrowBaton,
    InvItem::None,
    InvItem::None,
    InvItem::None,
];

pub const SHOPS: [(InvItem, usize); 6] = [
    (InvItem::Fertalizer, 10),
    (InvItem::WateringCan, 50),
    (InvItem::CrowBaton, 50),
    (InvItem::Sprinkler, 100),
    (InvItem::ScareCrow, 100),
    (InvItem::Shelf, 50),
];
