use std::io::Read;

#[derive(PartialEq, Debug)]
enum Tile {
    Pipe,
    Dash,
    L,
    J,
    Seven,
    F,
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Pipe,
            '-' => Self::Dash,
            'L' => Self::L,
            'J' => Self::J,
            '7' => Self::Seven,
            'F' => Self::F,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Tile>,
    row_len: usize,
}

impl From<String> for Grid {
    fn from(string: String) -> Self {
        let mut row_len = 0;
        let tiles: Vec<_> = string
            .lines()
            .flat_map(|line| {
                row_len = line.len();
                line.chars().map(|c| Tile::from(c))
            })
            .collect();
        Grid { tiles, row_len }
    }
}

#[derive(Debug)]
struct GridIterator<'a> {
    grid: &'a Grid,
    curr: usize,
    next: usize,
}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Grid {
    fn idx_in_dir(&self, idx: usize, dir: &Direction) -> usize {
        match dir {
            Direction::North => idx - self.row_len,
            Direction::South => idx + self.row_len,
            Direction::East => idx + 1,
            Direction::West => idx - 1,
        }
    }

    fn dir_from_idx(&self, idx1: usize, idx2: usize) -> Direction {
        let diff = idx2 as isize - idx1 as isize;
        let len = self.row_len as isize;
        if diff == 1 {
            Direction::East
        } else if diff == -1 {
            Direction::West
        } else if diff == len {
            Direction::South
        } else if diff == -len {
            Direction::North
        } else {
            unreachable!()
        }
    }
}

impl<'a> GridIterator<'a> {
    pub fn next_from_dir(tile: &Tile, incoming: &Direction) -> Option<Direction> {
        match tile {
            Tile::Pipe => match incoming {
                Direction::North => Some(Direction::South),
                Direction::South => Some(Direction::North),
                _ => None,
            },
            Tile::Dash => match incoming {
                Direction::East => Some(Direction::West),
                Direction::West => Some(Direction::East),
                _ => None,
            },
            Tile::L => match incoming {
                Direction::North => Some(Direction::East),
                Direction::East => Some(Direction::North),
                _ => None,
            },
            Tile::J => match incoming {
                Direction::North => Some(Direction::West),
                Direction::West => Some(Direction::North),
                _ => None,
            },
            Tile::Seven => match incoming {
                Direction::South => Some(Direction::West),
                Direction::West => Some(Direction::South),
                _ => None,
            },
            Tile::F => match incoming {
                Direction::South => Some(Direction::East),
                Direction::East => Some(Direction::South),
                _ => None,
            },
            _ => None,
        }
    }
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = &'a Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = Some(&self.grid.tiles[self.curr]);
        let incoming = self.grid.dir_from_idx(self.next, self.curr);
        self.curr = self.next;
        let tile = &self.grid.tiles[self.next];
        let dir = GridIterator::next_from_dir(tile, &incoming).unwrap();
        self.next = self.grid.idx_in_dir(self.next, &dir);
        curr
    }
}

impl Grid {
    fn make_iter(&self, check_order: [Direction; 4]) -> GridIterator {
        let start = self
            .tiles
            .iter()
            .position(|tile| *tile == Tile::Start)
            .unwrap();

        let dir = check_order
            .iter()
            .filter(|&dir| {
                let idx = self.idx_in_dir(start, dir);
                let tile = &self.tiles[idx];
                let into_start = self.dir_from_idx(idx, start);
                check_order
                    .iter()
                    .filter_map(|dir| GridIterator::next_from_dir(&tile, dir))
                    .any(|dir: Direction| dir == into_start)
            })
            .next()
            .unwrap();

        GridIterator {
            grid: self,
            curr: start,
            next: self.idx_in_dir(start, dir),
        }
    }

    pub fn iter(&self) -> GridIterator {
        self.make_iter([
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ])
    }

    pub fn rev_iter(&self) -> GridIterator {
        self.make_iter([
            Direction::West,
            Direction::South,
            Direction::East,
            Direction::North,
        ])
    }
}

fn main() {
    let mut stdin = String::new();
    std::io::stdin().read_to_string(&mut stdin).unwrap();
    let grid: Grid = stdin.into();
    let mut count = 1;
    let mut forward = grid.iter();
    forward.next();
    let mut backward = grid.rev_iter();
    backward.next();

    for (tile1, tile2) in forward.zip(backward) {
        if std::ptr::eq(tile1, tile2) {
            break;
        }
        count += 1;
    }

    println!("{count}");
}
