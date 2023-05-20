use crate::board::{Board, Display, Pos};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::button::{Appearance, StyleSheet};
use iced::widget::mouse_area::MouseArea;
use iced::widget::{column, row, text, Button, Row};
use iced::Settings;
use iced::{theme, Color, Element, Length, Sandbox, Theme};

mod board;

fn main() {
    Game::run(Settings::default()).unwrap();
}

enum Game {
    Board(Board),
    Menu,
}

#[derive(Debug, Copy, Clone)]
enum Msg {
    RevealSquare(Pos),
    FlagSquare(Pos),
    StartGame(Difficulty),
    EndGame,
}

#[derive(Debug, Copy, Clone)]
enum Difficulty {
    Beginner,
    Intermediate,
    Expert,
}

impl Sandbox for Game {
    type Message = Msg;

    fn new() -> Self {
        Game::Menu
    }

    fn title(&self) -> String {
        String::from("Minesweeper")
    }

    fn update(&mut self, message: Msg) {
        match self {
            Game::Board(board) => match message {
                Msg::RevealSquare(pos) => board.reveal_square(pos),
                Msg::FlagSquare(pos) => board.toggle_flag(pos),
                Msg::EndGame => *self = Game::Menu,
                _ => unreachable!(),
            },
            Game::Menu => match message {
                Msg::StartGame(difficulty) => *self = Game::Board(difficulty.start()),
                _ => unreachable!(),
            },
        }
    }

    fn view(&self) -> Element<Msg> {
        match self {
            Game::Board(board) => view_board(board),
            Game::Menu => view_menu(),
        }
    }
}

fn view_board(board: &Board) -> Element<Msg> {
    let mut rows: Vec<Element<Msg>> = Vec::with_capacity(board.width);
    for x in 0..board.width {
        let mut cells: Vec<Element<Msg>> = Vec::with_capacity(board.height);
        for y in 0..board.height {
            let pos = Pos { x, y };

            let t = match board[pos].state {
                Display::Closed => text("?"),
                Display::Flagged => text("!"),
                Display::OpenNearMines(mines) => text(mines),
                Display::OpenNoMines => text(""),
                Display::ShowMine => text("M"),
            }
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center);

            let colour = match board[pos].state {
                Display::Closed => Color::from_rgb8(200, 200, 200),
                Display::Flagged => Color::from_rgb8(150, 0, 0),
                Display::OpenNearMines { .. } => Color::from_rgb8(100, 100, 150),
                Display::OpenNoMines => Color::from_rgb8(75, 75, 75),
                Display::ShowMine => Color::from_rgb8(255, 0, 0),
            };

            let button = Button::new(t)
                .style(theme::Button::Custom(Box::new(ButtonColor { colour })))
                .width(Length::Fill)
                .height(Length::Fill);

            let e = if matches!(board[pos].state, Display::Flagged | Display::Closed) {
                MouseArea::new(button.on_press(Msg::RevealSquare(pos)))
                    .on_right_press(Msg::FlagSquare(pos))
                    .into()
            } else {
                button.into()
            };

            cells.push(e);
        }
        rows.push(Row::with_children(cells).height(Length::Fill).into());
    }

    rows.push(view_footer(board));
    column(rows).into()
}

fn view_footer(board: &Board) -> Element<Msg> {
    let remaining = board.total_mines - board.flags_placed - board.exploded_mines;
    let message = if board.hidden_squares == board.total_mines {
        String::from("You Win!")
    } else if board.hidden_squares < board.total_mines && remaining == 0 {
        String::from("All mines found, but at what cost")
    } else {
        format!("{} Mines Remaining", remaining)
    };
    row!(
        text(format!("{}x{}", board.width, board.height))
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center),
        Button::new(text("End Game").horizontal_alignment(Horizontal::Center))
            .style(theme::Button::Destructive)
            .on_press(Msg::EndGame)
            .width(Length::Fill),
        text(message)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center)
    )
    .into()
}

fn view_menu() -> Element<'static, Msg> {
    column!(
        start_button("Beginner", Difficulty::Beginner),
        start_button("Intermediate", Difficulty::Intermediate),
        start_button("Expert", Difficulty::Expert)
    )
    .into()
}

fn start_button(name: &str, value: Difficulty) -> Button<'_, Msg> {
    Button::new(
        text(name)
            .size(32.0)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .style(theme::Button::Positive)
    .on_press(Msg::StartGame(value))
    .width(Length::Fill)
    .height(Length::Fill)
}

impl Difficulty {
    fn start(self) -> Board {
        match self {
            Difficulty::Beginner => Board::new(9, 9, 10),
            Difficulty::Intermediate => Board::new(16, 16, 40),
            Difficulty::Expert => Board::new(30, 16, 99),
        }
    }
}

struct ButtonColor {
    colour: Color,
}

impl StyleSheet for ButtonColor {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(self.colour)),
            border_color: Color::from_rgb8(100, 100, 100),
            border_width: 2.0,
            ..Default::default()
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Some(iced::Background::Color(self.colour)),
            border_color: Color::WHITE,
            border_width: 2.0,
            ..Default::default()
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }
}
