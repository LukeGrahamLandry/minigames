use nannou::color::{GREEN, RED, WHITE, YELLOW};

pub const CROP_SIZE: f32 = 30.0;
pub const PLAYER_SPEED: f32 = 200.0;
pub const BASE_ROUND_TIMER: f32 = 60.0;
pub const MIN_ROUND_TIMER: f32 = 15.0;
pub const ROUND_TIME_DECREMENT: f32 = 5.0;
pub const REQUEST_COUNT: (usize, usize) = (4, 8);

#[derive(PartialEq)]
pub struct CropType {
    pub name: &'static str,
    pub growth_time: f32,
    pub seed_price: usize,
    pub sale_price: usize,
    pub colour: nannou::prelude::Srgb<u8>,
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
        growth_time: 30.0,
        seed_price: 15,
        sale_price: 20,
        colour: YELLOW,
    },
    CropType {
        name: "acorn",
        growth_time: 60.0,
        seed_price: 30,
        sale_price: 50,
        colour: GREEN,
    },
];
