use iced::{Application, Settings};
use minesweeper::Game;

fn main() {
    Game::run(Settings::default()).unwrap();
}
