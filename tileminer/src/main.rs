use crate::gen::starting_level;
use crate::level::{EntityID, Level};
use crate::render::view;
use nannou::event::Key;
use nannou::prelude::KeyPressed;
use nannou::{App, Event, Frame};

mod gen;
mod level;
mod player;
mod render;

fn main() {
    nannou::app(init)
        .event(handle_event)
        .simple_window(view)
        .run();
}

pub enum Game {
    Playing { level: Level, player_id: EntityID },
}

fn init(_app: &App) -> Game {
    let (level, player_id) = starting_level();
    Game::Playing { level, player_id }
}

fn handle_event(_app: &App, game: &mut Game, event: Event) {
    match game {
        Game::Playing { level, .. } => match event {
            Event::Update(update) => level.update(update.since_last.as_secs_f32()),
            Event::WindowEvent {
                simple: Some(KeyPressed(Key::Space)),
                ..
            } => {
                level.jump();
            }
            _ => {}
        },
    }
}
