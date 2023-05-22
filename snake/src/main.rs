use crate::render::render_world;
use crate::simulate::{Direction, World};
use bracket_lib::prelude::*;
use std::ops::DerefMut;

mod render;
mod simulate;

fn main() {
    let context = BTermBuilder::simple80x50()
        .with_title("Snake")
        .with_fancy_console(80, 50, "terminal8x8.png")
        .with_vsync(true)
        .build()
        .unwrap();

    main_loop(
        context,
        Game {
            world: World::new(Point::new(80, 50)),
        },
    )
    .unwrap()
}

struct Game {
    world: World,
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.input(ctx);
        self.world.update(1.0);
        self.render(ctx);
    }
}

impl Game {
    fn input(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            None => {}
            Some(VirtualKeyCode::D) => self.world.player.input(Direction::Right),
            Some(VirtualKeyCode::A) => self.world.player.input(Direction::Left),
            Some(VirtualKeyCode::W) => self.world.player.input(Direction::Up),
            Some(VirtualKeyCode::S) => self.world.player.input(Direction::Down),
            _ => {}
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(0, 0, format!("Score: {}", self.world.player.body.len()));
        let mut batch = DrawBatch::new();
        batch.target(1);
        render_world(batch.deref_mut(), &self.world);
        batch.submit(0).expect("Batch error");
        render_draw_buffer(ctx).expect("Render error");
    }
}
