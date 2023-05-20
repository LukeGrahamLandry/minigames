use rand::random;
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

pub struct Board {
    squares: Vec<Square>,
    pub width: usize,
    pub height: usize,
    pub total_mines: isize,
    pub flags_placed: isize,
    pub exploded_mines: isize,
    pub hidden_squares: isize,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Square {
    pub state: Display,
    pub is_mine: bool,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum Display {
    #[default]
    Closed,
    Flagged,
    OpenNearMines(usize),
    OpenNoMines,
    ShowMine,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, total_mines: usize) -> Board {
        assert!(width > 0 && height > 0 && total_mines < width * height);
        let mut board = Board {
            squares: vec![Square::default(); width * height],
            width,
            height,
            total_mines: total_mines as isize,
            flags_placed: 0,
            exploded_mines: 0,
            hidden_squares: (width * height) as isize,
        };
        board.place_mines(total_mines);
        board
    }

    pub fn reveal_square(&mut self, start: Pos) {
        // Don't accidentally blow up a flagged mine
        if let Display::Flagged = self[start].state {
            return;
        }

        // Game over
        if self[start].is_mine {
            self[start].state = Display::ShowMine;
            self.exploded_mines += 1;
            self.hidden_squares -= 1;
            return;
        }

        self.chain_reveal(start);
    }

    fn chain_reveal(&mut self, start: Pos) {
        let mut seen = HashSet::new();
        let mut next = vec![start];
        while let Some(pos) = next.pop() {
            seen.insert(pos);
            if let Display::Closed = self[pos].state {
                assert!(!self[pos].is_mine, "Chain reveal hit a mine.");
                self.hidden_squares -= 1;
                let adjacent_mines = self.count_adjacent_mines(pos);
                if adjacent_mines == 0 {
                    self[pos].state = Display::OpenNoMines;
                    next.extend(self.iter_adjacent(pos).filter(|p| !seen.contains(p)));
                } else {
                    self[pos].state = Display::OpenNearMines(adjacent_mines);
                }
            }
        }
    }

    pub fn toggle_flag(&mut self, pos: Pos) {
        match self[pos].state {
            Display::Closed => {
                self[pos].state = Display::Flagged;
                self.flags_placed += 1;
            }
            Display::Flagged => {
                self[pos].state = Display::Closed;
                self.flags_placed -= 1;
            }
            Display::OpenNearMines { .. } | Display::ShowMine | Display::OpenNoMines => {
                unreachable!()
            }
        }
    }

    fn count_adjacent_mines(&self, pos: Pos) -> usize {
        self.iter_adjacent(pos)
            .filter(|pos| self[*pos].is_mine)
            .count()
    }

    fn place_mines(&mut self, mut count: usize) {
        while count > 0 {
            let pos = self.rand_pos();
            if self[pos].is_mine {
                continue;
            }
            self[pos].is_mine = true;
            count -= 1;
        }
    }

    fn rand_pos(&self) -> Pos {
        Pos {
            x: random::<usize>() % self.width,
            y: random::<usize>() % self.height,
        }
    }

    fn iter_adjacent(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        [
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, 1),
            (0, 1),
            (0, -1),
            (-1, -1),
            (1, -1),
        ]
        .into_iter()
        .filter_map(move |(x, y)| self.offset(pos, x, y))
    }

    fn offset(&self, pos: Pos, x: isize, y: isize) -> Option<Pos> {
        let xx = pos.x as isize + x;
        let yy = pos.y as isize + y;
        if (0..self.width as isize).contains(&xx) && (0..self.height as isize).contains(&yy) {
            Some(Pos {
                x: xx as usize,
                y: yy as usize,
            })
        } else {
            None
        }
    }
}

impl Index<Pos> for Board {
    type Output = Square;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.squares[index.y * self.width + index.x]
    }
}

impl IndexMut<Pos> for Board {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.squares[index.y * self.width + index.x]
    }
}
