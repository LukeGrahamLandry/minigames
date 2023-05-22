use crate::simulate::{Direction, Snake, World};
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
    draw_snake(frame, &world.player, SKY_BLUE);
    for snake in &world.computer_snakes {
        draw_snake(frame, snake, RED);
    }
    for food in &world.food {
        draw(frame, food.pos, '*', GREEN);
    }
}

fn draw_snake(frame: &mut DrawBatch, snake: &Snake, colour: (u8, u8, u8)) {
    for part in &snake.body {
        draw(frame, *part, '■', colour);
    }
    draw(frame, snake.pos, arrow(snake.dir), colour);
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
