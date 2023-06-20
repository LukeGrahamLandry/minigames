use crate::gen::starting_level;
use crate::level::{EntityID, Level};
use crate::render::view;
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

fn handle_event(app: &App, game: &mut Game, event: Event) {}
