use crate::render::render;
use crate::simulate::{Direction, GameResult, World};
use bracket_lib::prelude::*;

mod render;
mod simulate;

const SCREEN_SIZE: Point = Point { x: 80, y: 50 };

fn main() {
    let context = BTermBuilder::simple(SCREEN_SIZE.x, SCREEN_SIZE.y)
        .unwrap()
        .with_title("Snake")
        .with_fancy_console(SCREEN_SIZE.x, SCREEN_SIZE.y, "terminal8x8.png")
        .with_fps_cap(30.0)
        .with_vsync(true)
        .build()
        .unwrap();

    main_loop(context, State::Initial).unwrap()
}

enum State {
    Initial,
    Lose,
    Playing(World),
    Paused(World),
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self {
            State::Playing(world) => {
                if let Some(VirtualKeyCode::Space) = ctx.key {
                    *self = State::Paused(world.clone());
                    return;
                }

                input(world, ctx);
                if world.update(1.0) == GameResult::Lose {
                    *self = State::Lose;
                } else {
                    render(world, ctx);
                }
            }
            State::Initial => {
                ctx.print_color_centered(SCREEN_SIZE.y / 2 + 2, SKY_BLUE, BLACK, "►");
                if should_unpause(ctx, "start") {
                    *self = State::Playing(World::new(SCREEN_SIZE));
                }
            }
            State::Paused(world) => {
                if should_unpause(ctx, "resume") {
                    *self = State::Playing(world.clone());
                }
            }
            State::Lose => {
                ctx.print_color_centered(SCREEN_SIZE.y / 2 - 2, RED, BLACK, "YOU LOSE.");
                ctx.print_color_centered(SCREEN_SIZE.y / 2 + 2, SKY_BLUE, BLACK, "►");
                if should_unpause(ctx, "try again") {
                    *self = State::Playing(World::new(SCREEN_SIZE));
                }
            }
        }
    }
}

const INSTRUCTIONS: [&str; 4] = [
    "Use the WASD keys to change direction.",
    "Collect food to grow longer.",
    "Avoid hitting your tail or going off the screen.",
    "Press SPACE to pause.",
];

fn should_unpause(ctx: &mut BTerm, action: &str) -> bool {
    INSTRUCTIONS
        .into_iter()
        .enumerate()
        .for_each(|(i, msg)| ctx.print_centered(4 + (i * 2), msg));

    ctx.print_centered(SCREEN_SIZE.y / 2, format!("Press SPACE to {}.", action));
    matches!(ctx.key, Some(VirtualKeyCode::Space))
}

fn input(world: &mut World, ctx: &mut BTerm) {
    match ctx.key {
        Some(VirtualKeyCode::D) => world.player.input(Direction::Right),
        Some(VirtualKeyCode::A) => world.player.input(Direction::Left),
        Some(VirtualKeyCode::W) => world.player.input(Direction::Up),
        Some(VirtualKeyCode::S) => world.player.input(Direction::Down),
        _ => {}
    }
}
