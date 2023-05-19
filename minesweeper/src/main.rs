use crate::board::{Board, Display, Pos};
use iced::widget::mouse_area::MouseArea;
use iced::widget::{button, column, text, Button, Row, Text};
use iced::Settings;
use iced::{theme, Color, Element, Length, Sandbox, Theme};
mod board;

fn main() {
    Game::run(Settings::default()).unwrap();
}

pub struct Game {
    board: Board,
}

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    RevealSquare(Pos),
    FlagSquare(Pos),
}

impl Sandbox for Game {
    type Message = Msg;

    fn new() -> Self {
        Game {
            board: Board::new(15, 15, 10),
        }
    }

    fn title(&self) -> String {
        String::from("Minesweeper")
    }

    fn update(&mut self, message: Msg) {
        match message {
            Msg::RevealSquare(pos) => {
                self.board.chain_reveal(pos);
            }
            Msg::FlagSquare(pos) => {
                self.board.toggle_flag(pos);
            }
        }
    }

    fn view(&self) -> Element<Msg> {
        let mut cols: Vec<Element<Msg>> = Vec::with_capacity(self.board.width);
        for x in 0..self.board.width {
            let mut rows: Vec<Element<Msg>> = Vec::with_capacity(self.board.height);
            for y in 0..self.board.height {
                let pos = Pos { x, y };

                let t: Text = match self.board[pos].state {
                    Display::Closed => text("?"),
                    Display::Flagged => text("!"),
                    Display::Open { adjacent_mines } => {
                        if adjacent_mines == 0 {
                            text("")
                        } else {
                            text(adjacent_mines)
                        }
                    }
                    Display::ShowMine => text("M"),
                };

                let colour = match self.board[pos].state {
                    Display::Closed => Color::from_rgb8(200, 200, 200),
                    Display::Flagged => Color::from_rgb8(200, 0, 0),
                    Display::Open { adjacent_mines } => {
                        if adjacent_mines == 0 {
                            Color::from_rgb8(75, 75, 75)
                        } else {
                            Color::from_rgb8(100, 100, 150)
                        }
                    }
                    Display::ShowMine => Color::from_rgb8(255, 0, 0),
                };

                let button = Button::new(t)
                    .style(theme::Button::Custom(Box::new(ButtonColor { colour })))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(Msg::RevealSquare(pos));

                rows.push(
                    MouseArea::new(button)
                        .on_right_press(Msg::FlagSquare(pos))
                        .into(),
                );
            }
            cols.push(Row::with_children(rows).height(Length::Fill).into());
        }

        column(cols).into()
    }
}

struct ButtonColor {
    colour: Color,
}

impl button::StyleSheet for ButtonColor {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.colour)),
            ..Default::default()
        }
    }
}
