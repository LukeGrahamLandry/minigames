use nannou::color;
use nannou::prelude::*;

use crate::config::CROP_SIZE;
use crate::simulate::{Crop, InvItem, World};

pub fn render_simple(world: &World, draw: &Draw) {
    draw.rect()
        .color(DARKGRAY)
        .xy(world.farm.area.xy())
        .wh(world.farm.area.wh());

    draw.rect()
        .color(ORANGE)
        .xy(world.truck.area.xy())
        .wh(world.truck.area.wh());

    let mut tx = -90.0;
    for crop in &world.truck.wants {
        draw_item(
            draw,
            InvItem::Crop(crop),
            &Crop::rect(world.truck.area.xy() + Vec2::new(tx, 60.0)),
            0.5,
        );
        tx += CROP_SIZE
    }

    for crop in &world.farm.crops {
        if crop.is_ripe() {
            draw_item(draw, InvItem::Crop(crop.kind), &crop.area, 1.0);
        } else {
            draw_item(draw, InvItem::Seed(crop.kind), &crop.area, 1.0);
        }
        if crop.needs_water {
            draw.rect()
                .color(BLUE)
                .xy(crop.area.xy() + Vec2::new(CROP_SIZE / 2.0, CROP_SIZE / 2.0))
                .wh(crop.area.wh() * 0.2);
        }
    }

    for shop in &world.shops {
        draw.rect()
            .color(if world.cash >= shop.price {
                LIME
            } else {
                DARKCYAN
            })
            .xy(shop.area.xy())
            .wh(shop.area.wh());

        draw_item(draw, shop.contents, &shop.area, 1.0);
        draw.text(&format!("{}", shop.price))
            .color(BLACK)
            .xy(shop.area.xy())
            .font_size(15);
    }

    for shelf in &world.shelves {
        draw.rect()
            .color(LIGHTBLUE)
            .xy(shelf.area.xy())
            .wh(shelf.area.wh());

        draw_item(draw, shelf.contents, &shelf.area, 1.0);

        if shelf.crow_hp > 0 {
            draw.rect()
                .color(ORANGE)
                .xy(shelf.area.xy() + Vec2::new(CROP_SIZE / 2.0, CROP_SIZE / 2.0))
                .wh(shelf.area.wh() * 0.2 * (shelf.crow_hp as f32 / 2.0));
        }
    }

    draw.rect()
        .color(srgba8(0, 0, 0, 230))
        .xy(world.player.pos)
        .wh(Vec2::new(CROP_SIZE, CROP_SIZE));

    draw_item(draw, world.player.inventory, &world.player.area(), 1.0);

    draw.ellipse()
        .color(LIGHTCORAL)
        .xy(world.player.pos)
        .wh(Vec2::new(5.0, 5.0));

    draw.text(&format!("Money: {}", world.cash))
        .color(BLACK)
        .xy(Point2::new(0.0, 300.0))
        .font_size(15);

    draw.text(&format!("Timer: {:.0}", world.round_timer))
        .color(BLACK)
        .xy(Point2::new(0.0, 280.0))
        .font_size(15);

    draw.text(&format!("Round: {}", world.round_number))
        .color(BLACK)
        .xy(Point2::new(0.0, 260.0))
        .font_size(15);
}

fn draw_item(draw: &Draw, item: InvItem, area: &Rect, scale: f32) {
    match item {
        InvItem::None => {}
        InvItem::Seed(crop) => {
            draw.tri()
                .color(crop.colour)
                .xy(area.xy() + Vec2::new(CROP_SIZE * 0.4, 0.0))
                .wh(area.wh() * scale);
        }
        InvItem::Crop(crop) => {
            draw.ellipse()
                .color(crop.colour)
                .xy(area.xy())
                .wh(area.wh() * scale);
        }
        InvItem::WateringCan => {
            draw.rect()
                .color(BLUE)
                .xy(area.xy())
                .wh(area.wh() * scale * 0.8);
        }
        InvItem::CrowBaton => {
            draw.rect()
                .color(ORANGE)
                .xy(area.xy())
                .wh(area.wh() * scale * 0.8);
        }
    }
}
