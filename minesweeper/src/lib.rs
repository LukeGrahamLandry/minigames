mod board;

use crate::board::{Board, Display, Pos};
use iced::widget::canvas::{Cursor, Geometry, Program};
use iced::widget::{canvas, column, row, text, Button, Canvas, Row, Text};
use iced::{Element, Length, Rectangle, Sandbox, Theme};

pub struct Game {
    board: Board,
}

#[derive(Debug, Copy, Clone)]
pub enum Msg {
    RevealSquare(Pos),
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
                println!("Reveal {:?}", pos);
                self.board.chain_reveal(pos);
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
                    Display::Open { adjacent_mines } => text(adjacent_mines),
                    Display::Mine => text("M"),
                };

                let button = Button::new(t)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(Msg::RevealSquare(Pos { x, y }));
                rows.push(button.into());
            }
            cols.push(Row::with_children(rows).height(Length::Fill).into());
        }

        column(cols).into()
    }
}
