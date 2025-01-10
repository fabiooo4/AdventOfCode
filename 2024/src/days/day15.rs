use owo_colors::OwoColorize;
use std::{
    fmt::Display,
    ops::{Add, AddAssign},
    thread::sleep,
    time::Duration,
};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let (mut grid, directions) = parse_input(input).unwrap_or_else(|e| panic!("{e}"));

    grid.run_robot(&directions);


    let sol1: u64 = grid.calculate_gps();
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_input(input: &str) -> Result<(Grid, Vec<Direction>), String> {
    let (grid, moves) = match input.trim().split_once("\n\n") {
        Some(val) => val,
        None => return Err(String::from("Wrong input")),
    };

    let mut robot: Option<Coordinate<i64>> = None;
    let mut boxes: Vec<Coordinate<i64>> = Vec::new();
    let mut walls: Vec<Coordinate<i64>> = Vec::new();
    let height = grid.trim().lines().count();
    let width = grid.trim().lines().next().unwrap().trim().chars().count();

    for (y, line) in grid.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let (x, y) = (x as i64, y as i64);
            match cell {
                '@' => robot = Some(Coordinate::new(x, y)),
                'O' => boxes.push(Coordinate::new(x, y)),
                '#' => walls.push(Coordinate::new(x, y)),
                _ => {}
            }
        }
    }

    if robot.is_none() {
        return Err(String::from("No robot in input"));
    }

    let grid = Grid::new(robot.unwrap(), boxes, walls, width, height);

    let moves: Vec<Direction> = moves
        .trim()
        .chars()
        .filter_map(|c| c.try_into().ok())
        .collect();

    Ok((grid, moves))
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(String::from(
                "Cannot parse chars different from: '^' 'v' '<' '>'",
            )),
        }
    }
}

impl Direction {
    fn delta(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
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

impl<T> AddAssign<Coordinate<T>> for Coordinate<T>
where
    T: AddAssign + Add<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Coordinate<T>) {
        *self = *self + rhs
    }
}

struct Grid {
    robot: Coordinate<i64>,
    boxes: Vec<Coordinate<i64>>,
    walls: Vec<Coordinate<i64>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(
        robot: Coordinate<i64>,
        boxes: Vec<Coordinate<i64>>,
        walls: Vec<Coordinate<i64>>,
        width: usize,
        height: usize,
    ) -> Self {
        Grid {
            robot,
            boxes,
            walls,
            width,
            height,
        }
    }

    fn move_robot(&mut self, direction: &Direction) {
        let next_pos = self.robot + direction.delta().into();

        // Out of bounds check
        if next_pos.x < 0
            || next_pos.y < 0
            || next_pos.x > self.width as i64
            || next_pos.y > self.height as i64
        {
            return;
        }

        // If the next position is a wall don't move
        if self.walls.contains(&next_pos) {
            return;
        }

        // Move all the boxes touching in the same direction
        if self.boxes.contains(&next_pos) {
            if let Some(amount) = self.move_boxes(next_pos, direction) {
                self.robot += amount
            }
        } else {
            self.robot += direction.delta().into()
        }
    }

    fn move_boxes(
        &mut self,
        position: Coordinate<i64>,
        direction: &Direction,
    ) -> Option<Coordinate<i64>> {
        let next_pos = position + direction.delta().into();

        // Out of bounds check
        if next_pos.x < 0
            || next_pos.y < 0
            || next_pos.x > self.width as i64
            || next_pos.y > self.height as i64
        {
            return None;
        }

        if self.walls.contains(&next_pos) {
            // If there is a wall next, don't move
            return None;
        }

        if self.boxes.contains(&next_pos) {
            // If there is a box next move that box
            if let Some(amount) = self.move_boxes(next_pos, direction) {
                *self.boxes.iter_mut().find(|&&mut b| b == position).unwrap() += amount;
                return Some(amount);
            } else {
                return None;
            }
        } else {
            *self.boxes.iter_mut().find(|&&mut b| b == position).unwrap() +=
                direction.delta().into();
        }

        Some(direction.delta().into())
    }

    fn run_robot(&mut self, directions: &[Direction]) {
        for direction in directions {
            println!("{}[2J", 27 as char);
            println!("{self}");
            self.move_robot(direction);
            sleep(Duration::from_millis(3));
        }
    }

    fn calculate_gps(&self) -> u64 {
        self.boxes
            .iter()
            .fold(0, |acc, b| acc + 100 * b.y as u64 + b.x as u64)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid: Vec<Vec<char>> = vec![vec![' '; self.width]; self.height];

        let wall_char = '#';
        let box_char = 'O';
        let robot_char = '@';

        for b in &self.boxes {
            let cell = grid
                .get_mut(b.y as usize)
                .unwrap()
                .get_mut(b.x as usize)
                .unwrap();
            *cell = box_char;
        }

        for w in &self.walls {
            let cell = grid
                .get_mut(w.y as usize)
                .unwrap()
                .get_mut(w.x as usize)
                .unwrap();
            *cell = wall_char;
        }

        let cell = grid
            .get_mut(self.robot.y as usize)
            .unwrap()
            .get_mut(self.robot.x as usize)
            .unwrap();
        *cell = robot_char;

        for line in grid {
            for cell in line {
                if cell == robot_char {
                    write!(f, "{}", cell.red())?
                } else if cell == box_char {
                    write!(f, "{}", cell.yellow())?
                } else if cell == wall_char {
                    write!(f, "{}", cell.white())?
                } else {
                    write!(f, "{}", cell)?
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn coordinate_sum() {
        assert_eq!(
            Coordinate::new(1, 1) + Coordinate::new(1, 1),
            Coordinate::new(2, 2)
        )
    }
}
