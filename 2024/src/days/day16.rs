use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul},
    thread::sleep,
    time::Duration,
};

use owo_colors::OwoColorize;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let grid = parse_input(input);
    let shortest_path = grid.a_star(grid.start, grid.end);

    let mut reindeer = Reindeer::new(grid.start);

    let sol1: u64 = reindeer.run(&shortest_path, &grid);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_input(input: &str) -> Grid {
    Grid::new(
        input
            .trim()
            .lines()
            .map(|l| l.trim().chars().map(|c| c.into()).collect())
            .collect(),
    )
}

//
// Tiles
//
#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Space,
    Wall,
    Start,
    End,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Tile::Start,
            'E' => Tile::End,
            '#' => Tile::Wall,
            '.' => Tile::Space,
            _ => Tile::Space,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Space => write!(f, "{}", "  ".on_default_color()),
            Tile::Wall => write!(f, "{}", "  ".on_white()),
            Tile::Start => write!(f, "{}", "  ".on_green()),
            Tile::End => write!(f, "{}", "  ".on_red()),
        }
    }
}

//
// Coordinate
//
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, Default)]
struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coordinate<T> {
    fn new(x: T, y: T) -> Self {
        Coordinate { x, y }
    }
}

impl<T> From<(T, T)> for Coordinate<T> {
    fn from(tuple: (T, T)) -> Self {
        Coordinate::new(tuple.0, tuple.1)
    }
}

impl<T: Add<Output = T>> Add<Coordinate<T>> for Coordinate<T> {
    type Output = Self;

    fn add(self, rhs: Coordinate<T>) -> Self::Output {
        Coordinate::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Mul<Output = T>> Mul<(T, T)> for Coordinate<T> {
    type Output = Self;

    fn mul(self, rhs: (T, T)) -> Self::Output {
        Coordinate::new(self.x * rhs.0, self.y * rhs.1)
    }
}

impl<T> AddAssign<Coordinate<T>> for Coordinate<T>
where
    T: AddAssign + Add<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Coordinate<T>) {
        *self = *self + rhs
    }
}

//
// Grid
//
struct Grid {
    cells: Vec<Vec<Tile>>,
    start: Coordinate<i64>,
    end: Coordinate<i64>,
}

impl Grid {
    fn new(cells: Vec<Vec<Tile>>) -> Self {
        let mut start = None;
        let mut end = None;

        for (y, line) in cells.iter().cloned().enumerate() {
            if start.is_some() && end.is_some() {
                break;
            }

            for (x, cell) in line.iter().enumerate() {
                match cell {
                    Tile::Start => start = Some((x as i64, y as i64)),
                    Tile::End => end = Some((x as i64, y as i64)),
                    _ => {}
                }
            }
        }

        Self {
            cells,
            start: start.expect("No start found").into(),
            end: end.expect("No end found").into(),
        }
    }

    fn print_a_star(&self, open: &[Path], close: &[Path], path: Option<&[Coordinate<i64>]>) {
        print!("{}[2J", 27 as char);
        for (y, line) in self.cells.iter().enumerate() {
            'next_cell: for (x, cell) in line.iter().enumerate() {
                if let Some(path) = path {
                    for p in path {
                        if Coordinate::new(x as i64, y as i64) == *p
                            && *cell != Tile::Start
                            && *cell != Tile::End
                        {
                            print!("{}", "  ".on_blue());
                            continue 'next_cell;
                        }
                    }
                }

                for o in open {
                    if Coordinate::new(x as i64, y as i64) == o.position
                        && *cell != Tile::Start
                        && *cell != Tile::End
                    {
                        print!("{}", "  ".on_bright_magenta());
                        continue 'next_cell;
                    }
                }

                for c in close {
                    if Coordinate::new(x as i64, y as i64) == c.position
                        && *cell != Tile::Start
                        && *cell != Tile::End
                    {
                        print!("{}", "  ".on_purple());
                        continue 'next_cell;
                    }
                }

                print!("{cell}");
            }
            println!();
        }
        sleep(Duration::from_millis(10));
    }

    fn a_star(&self, start_pos: Coordinate<i64>, end_pos: Coordinate<i64>) -> Vec<Coordinate<i64>> {
        let mut open = vec![Path::new(start_pos)]; // Nodes to evaluate
        let mut close = vec![]; // Nodes already evaluated

        loop {
            self.print_a_star(&open, &close, None);
            let min_idx = open
                .iter()
                .position(|c| {
                    self.f_cost(&c.position)
                        == open
                            .iter()
                            .map(|c| self.f_cost(&c.position))
                            .min()
                            .unwrap_or_default()
                })
                .unwrap();

            let current = open.remove(min_idx);
            close.push(current.clone());

            if current.position == end_pos {
                let path = self.retrace_path(close.first().unwrap(), close.last().unwrap());
                self.print_a_star(&open, &close, Some(&path));
                sleep(Duration::from_secs(1));
                return path;
            }

            for mut neighbour in self.neighbours(current.position) {
                if close.iter().any(|c| c.position == neighbour.position) {
                    continue;
                }

                let f_cost = self.f_cost(&neighbour.position);
                neighbour.parent = Some(Box::new(current.clone()));

                if open
                    .iter()
                    .find(|&c| c.position == neighbour.position)
                    .is_some_and(|c| self.f_cost(&c.position) < f_cost)
                {
                    continue;
                }

                if close
                    .iter()
                    .find(|&c| c.position == neighbour.position)
                    .is_some_and(|c| self.f_cost(&c.position) < f_cost)
                {
                    continue;
                }

                open.push(neighbour);
            }
        }
    }

    /// Trace the path from the end to the start following the parent to determine the best path
    fn retrace_path(&self, start_path: &Path, end_path: &Path) -> Vec<Coordinate<i64>> {
        let mut path = Vec::new();
        let mut current = end_path.clone();

        while current != *start_path {
            path.push(current.clone());
            current = *current.parent.unwrap();
        }

        path.iter().map(|p| p.position).collect()
    }

    /// Calculate the f_cost where:
    /// - `h_cost` is the distance from the end
    /// - `g_cost` is the distance from the start
    /// - `f_cost` is the sum of h_cost + g_cost
    ///
    /// The distance between 2 neighbouring cells is 1
    fn f_cost(&self, current_cell: &Coordinate<i64>) -> usize {
        // TODO: Count the number of steps and turns in the cost
        let g_cost = (current_cell.x - self.start.x).unsigned_abs()
            + (current_cell.y - self.start.y).unsigned_abs();

        let h_cost = (current_cell.x - self.end.x).unsigned_abs()
            + (current_cell.y - self.end.y).unsigned_abs();

        (g_cost + h_cost) as usize
    }

    /// Returns a list of all the traversable neighbours of a cell
    fn neighbours(&self, current_cell: Coordinate<i64>) -> Vec<Path> {
        let mut neighbours = Vec::new();

        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        for direction in directions.map(|d| d.delta()) {
            // Check bounds
            if current_cell.x + direction.x < 0
                || current_cell.x + direction.y < 0
                || current_cell.x + direction.x > self.width() as i64
                || current_cell.y + direction.y > self.height() as i64
            {
                continue;
            }

            let next_pos = current_cell + direction;

            let next_cell = self
                .cells
                .get(next_pos.y as usize)
                .unwrap()
                .get(next_pos.x as usize)
                .unwrap();
            if *next_cell != Tile::Wall {
                neighbours.push(Path::new(next_pos));
            }
        }

        neighbours
    }

    fn width(&self) -> usize {
        self.cells.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn print_reindeer(&self, path: &[Coordinate<i64>], reindeer: &Reindeer) {
        print!("{}[2J", 27 as char);
        for (y, line) in self.cells.iter().enumerate() {
            'next_cell: for (x, cell) in line.iter().enumerate() {
                if reindeer.position == Coordinate::new(x as i64, y as i64) {
                    print!("{}{}", " ".on_yellow(), reindeer.direction.on_yellow());
                    continue 'next_cell;
                }

                for p in path {
                    if Coordinate::new(x as i64, y as i64) == *p
                        && *cell != Tile::Start
                        && *cell != Tile::End
                    {
                        print!("{}", "  ".on_blue());
                        continue 'next_cell;
                    }
                }

                print!("{cell}");
            }
            println!();
        }
        sleep(Duration::from_millis(100));
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.cells {
            for cell in line {
                write!(f, "{cell}")?
            }
            writeln!(f)?
        }

        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Path {
    position: Coordinate<i64>,
    parent: Option<Box<Path>>,
}

impl Path {
    fn new(pos: Coordinate<i64>) -> Self {
        Self {
            position: pos,
            parent: None,
        }
    }
}

//
// Direction
//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn delta(&self) -> Coordinate<i64> {
        match self {
            Direction::Up => (0, -1).into(),
            Direction::Down => (0, 1).into(),
            Direction::Left => (-1, 0).into(),
            Direction::Right => (1, 0).into(),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

struct Reindeer {
    position: Coordinate<i64>,
    direction: Direction,
}

impl Reindeer {
    fn new(start_pos: Coordinate<i64>) -> Self {
        Reindeer {
            position: start_pos,
            direction: Direction::Right,
        }
    }

    fn rotate(&mut self, direction: Direction) {
        self.direction = match direction {
            Direction::Left => match self.direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            Direction::Right => match self.direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            _ => self.direction,
        };
    }

    fn move_in(&mut self, direction: Direction) {
        self.position += direction.delta();
    }

    fn run(&mut self, path: &[Coordinate<i64>], grid: &Grid) -> u64 {
        let mut turns = 0;
        let mut moves = 0;

        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        for p in path.iter().rev() {
            grid.print_reindeer(path, self);
            for direction in directions {
                if *p == self.position + direction.delta() {
                    while self.direction != direction {
                        match (self.direction, direction) {
                            (Direction::Up, Direction::Right) => self.rotate(Direction::Right),
                            (Direction::Up, Direction::Left) => self.rotate(Direction::Left),
                            (Direction::Down, Direction::Right) => self.rotate(Direction::Left),
                            (Direction::Down, Direction::Left) => self.rotate(Direction::Right),
                            (Direction::Left, Direction::Up) => self.rotate(Direction::Right),
                            (Direction::Left, Direction::Down) => self.rotate(Direction::Left),
                            (Direction::Right, Direction::Up) => self.rotate(Direction::Left),
                            (Direction::Right, Direction::Down) => self.rotate(Direction::Right),
                            _ => {}
                        }

                        turns += 1000;
                    }

                    self.move_in(direction);
                    moves += 1;
                }
            }
        }

        turns + moves
    }
}
