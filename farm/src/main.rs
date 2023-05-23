use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;

use crate::render::render_simple;
use crate::simulate::World;

mod config;
mod render;
mod simulate;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

enum Game {
    Initial,
    Playing(World),
    Pause(World),
    GameOver { round: usize },
}

fn model(_app: &App) -> Game {
    Game::Initial
}

fn event(app: &App, game: &mut Game, event: Event) {
    match game {
        Game::Playing(world) => {
            if let Event::WindowEvent { simple, .. } = &event {
                if let Some(WindowEvent::KeyPressed(Key::G)) = simple {
                    *game = Game::Pause(world.clone());
                    return;
                }

                if let Some(WindowEvent::KeyPressed(Key::Space)) = simple {
                    world.interact();
                }
            }
            if let Event::Update(delta) = &event {
                user_input(app, world);
                world.update(delta.since_last.as_secs_f32());
                if world.game_over() {
                    *game = Game::GameOver {
                        round: world.round_number,
                    }
                }
            }
        }
        Game::Initial | Game::GameOver { .. } => {
            if let Event::WindowEvent { simple, .. } = &event {
                if let Some(WindowEvent::KeyPressed(Key::G)) = simple {
                    *game = Game::Playing(World::new());
                }
            }
        }
        Game::Pause(world) => {
            if let Event::WindowEvent { simple, .. } = &event {
                if let Some(WindowEvent::KeyPressed(Key::G)) = simple {
                    *game = Game::Playing(world.clone());
                }
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

// Feels clever to not rerender the world when paused but still show it behind the text
// by just not drawing the blank background but that makes the text look terrible for some reason.
fn view(app: &App, game: &Game, frame: Frame) {
    let draw = app.draw();
    match game {
        Game::Playing(world) => {
            draw.background().color(GRAY);
            render_simple(world, &draw);
        }
        Game::Initial => {
            draw.text("Press G to start a new game.");
            draw.text("You can press G again to pause the game if you need a break!")
                .y(-20.0);
        }
        Game::Pause(_) => {
            draw.text("Press G to continue.");
            draw.text("Game paused!").y(20.0);
        }
        Game::GameOver { round } => {
            draw.text("Press G to try again.");
            draw.text(&format!("You survived until round {}.", round))
                .color(RED)
                .y(20.0);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
