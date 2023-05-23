use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;

use crate::render::render;
use crate::simulate::World;

mod config;
mod render;
mod simulate;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

enum Game {
    Playing(World),
}

fn model(_app: &App) -> Game {
    Game::Playing(World::new())
}

fn event(app: &App, game: &mut Game, event: Event) {
    match game {
        Game::Playing(world) => {
            if let Event::WindowEvent { simple, .. } = &event {
                if let Some(WindowEvent::KeyPressed(Key::Space)) = simple {
                    world.interact();
                }
            }
            if let Event::Update(delta) = &event {
                user_input(app, world);
                world.update(delta.since_last.as_secs_f32());
            }
        }
    }
}

fn user_input(app: &App, world: &mut World) {
    world.player.dir = Vec2::new(0.0, 0.0);
    [Key::W, Key::S, Key::A, Key::D]
        .into_iter()
        .filter(|key| app.keys.down.contains(key))
        .for_each(|key| {
            world.player.dir += dir(key).unwrap();
        });
    if world.player.dir.length_squared() != 0.0 {
        world.player.dir = world.player.dir.normalize();
    }
}

fn dir(key: VirtualKeyCode) -> Option<Vec2> {
    match key {
        Key::W => Some((0.0, 1.0)),
        Key::S => Some((0.0, -1.0)),
        Key::A => Some((-1.0, 0.0)),
        Key::D => Some((1.0, 0.0)),
        _ => None,
    }
    .map(|(x, y)| Vec2::new(x, y))
}

fn view(app: &App, game: &Game, frame: Frame) {
    match game {
        Game::Playing(world) => {
            let draw = app.draw();
            draw.background().color(GRAY);
            render(world, &draw);
            draw.to_frame(app, &frame).unwrap();
        }
    }
}
