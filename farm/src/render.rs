use crate::config::CROP_SIZE;
use crate::simulate::{InvItem, World};
use nannou::prelude::*;

pub fn render(world: &World, draw: &Draw) {
    draw.rect()
        .color(DARKGRAY)
        .xy(world.farm.area.xy())
        .wh(world.farm.area.wh());

    for crop in &world.farm.crops {
        if crop.is_ripe() {
            draw_item(draw, InvItem::Crop(crop.kind), &crop.area);
        } else {
            draw_item(draw, InvItem::Seed(crop.kind), &crop.area);
        }
    }

    for shop in &world.shops {
        draw.rect()
            .color(LIME)
            .xy(shop.area.xy())
            .wh(shop.area.wh());

        draw_item(draw, shop.contents, &shop.area);
    }

    draw.rect()
        .color(BLACK)
        .xy(world.player.pos)
        .wh(Vec2::new(CROP_SIZE, CROP_SIZE));
    draw_item(draw, world.player.inventory, &world.player.area());
}

fn draw_item(draw: &Draw, item: InvItem, area: &Rect) {
    match item {
        InvItem::None => {}
        InvItem::Seed(crop) => {
            draw.tri()
                .color(crop.colour)
                .xy(area.xy() + Vec2::new(CROP_SIZE, 0.0))
                .wh(area.wh());
        }
        InvItem::Crop(crop) => {
            draw.ellipse()
                .color(crop.colour)
                .xy(area.xy())
                .wh(area.wh());
        }
    }
}
