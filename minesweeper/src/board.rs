use rand::random;
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

pub struct Board {
    squares: Vec<Square>,
    pub width: usize,
    pub height: usize,
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
    Open {
        adjacent_mines: usize,
    },
    ShowMine,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Board {
        assert!(width > 0 && height > 0);
        let mut board = Board {
            squares: vec![Square::default(); width * height],
            width,
            height,
        };
        board.place_mines(mine_count);
        board
    }

    pub fn chain_reveal(&mut self, start: Pos) {
        if self[start].is_mine {
            self[start].state = Display::ShowMine;
            return;
        }

        let mut seen = HashSet::new();
        let mut next = vec![start];
        while let Some(pos) = next.pop() {
            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos);
            if let Display::Closed = self[pos].state {
                let adjacent_mines = self.count_adjacent_mines(pos);
                self[pos].state = Display::Open { adjacent_mines };
                if adjacent_mines == 0 {
                    next.extend(self.iter_adjacent(pos));
                }
            }
        }
    }

    pub fn toggle_flag(&mut self, pos: Pos) {
        match self[pos].state {
            Display::Closed => self[pos].state = Display::Flagged,
            Display::Flagged => self[pos].state = Display::Closed,
            Display::Open { .. } | Display::ShowMine => {}
        }
    }

    fn count_adjacent_mines(&self, pos: Pos) -> usize {
        self.iter_adjacent(pos)
            .filter(|pos| self[*pos].is_mine)
            .count()
    }

    fn place_mines(&mut self, count: usize) {
        for _ in 0..count {
            let pos = self.rand_pos();
            self[pos].is_mine = true;
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
