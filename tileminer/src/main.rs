use crate::gen::starting_level;
use crate::level::{EntityID, Level};
use crate::render::view;
use nannou::event::{Key, MouseScrollDelta, WindowEvent};
use nannou::prelude::KeyPressed;
use nannou::text::font::default;
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

fn handle_event(app: &App, game: &mut Game, event: Event) {
    match game {
        Game::Playing { level, player_id } => {
            if let Event::Update(update) = event {
                level.player_input(&app.keys, &app.mouse, player_id, app.main_window().rect());
                level.update(update.since_last.as_secs_f32())
            }
            if let Event::WindowEvent {
                id,
                simple: Some(WindowEvent::MouseWheel(delta, _)),
            } = event
            {
                if let MouseScrollDelta::PixelDelta(pos) = delta {
                    // TODO: It deeply offends me that I picked the max zoom out by when the fps started dropping.
                    //       It's drawing like a hundred rectangles for fuck sake. How can that possibly be slow. Fix it.
                    level.view_scale =
                        (level.view_scale + (pos.y * 0.01) as f32).clamp(0.25, 100.0);
                } else {
                    todo!();
                }
            }
        }
    }
}
