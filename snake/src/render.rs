use crate::simulate::{Direction, World};
use bracket_lib::prelude::*;
use std::ops::DerefMut;

const NO_ROT: Degrees = Degrees(0.0);

pub fn render(world: &mut World, ctx: &mut BTerm) {
    ctx.cls();
    ctx.print_centered(0, format!("Score: {}", world.player.body.len()));
    let mut batch = DrawBatch::new();
    batch.target(1);
    render_world(batch.deref_mut(), world);
    batch.submit(0).expect("Batch error");
    render_draw_buffer(ctx).expect("Render error");
}

fn render_world(frame: &mut DrawBatch, world: &World) {
    for part in &world.player.body {
        draw(frame, *part, '■', SKY_BLUE);
    }
    draw(frame, world.player.pos, arrow(world.player.dir), SKY_BLUE);

    for food in &world.food {
        draw(frame, food.pos, '*', GREEN);
    }
}

fn draw(frame: &mut DrawBatch, pos: PointF, glyph: char, colour: (u8, u8, u8)) {
    frame.set_fancy(
        pos + PointF::new(0.5, 0.5),
        0,
        NO_ROT,
        PointF::new(1.0, 1.0),
        ColorPair::new(colour, BLACK),
        to_cp437(glyph),
    );
}

fn arrow(dir: Direction) -> char {
    match dir {
        Direction::Left => '◄',
        Direction::Right => '►',
        Direction::Up => '▲',
        Direction::Down => '▼',
    }
}
