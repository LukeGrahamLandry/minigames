pub const CROP_SIZE: f32 = 10.0;

pub struct CropType {
    pub name: &'static str,
    pub growth_time: f32,
    pub seed_price: usize,
    pub sale_price: usize,
}

pub const CROPS: [CropType; 4] = [
    CropType {
        name: "tomato",
        growth_time: 5.0,
        seed_price: 1,
        sale_price: 2,
    },
    CropType {
        name: "potato",
        growth_time: 15.0,
        seed_price: 3,
        sale_price: 5,
    },
    CropType {
        name: "pizza",
        growth_time: 30.0,
        seed_price: 15,
        sale_price: 20,
    },
    CropType {
        name: "acorn",
        growth_time: 60.0,
        seed_price: 30,
        sale_price: 50,
    },
];
