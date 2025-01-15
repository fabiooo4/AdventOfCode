use owo_colors::OwoColorize;
use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, AddAssign, Mul},
};

#[cfg(all(not(test), feature = "visualize"))]
use std::{thread::sleep, time::Duration};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let (mut small_grid, directions) = parse_input(input).unwrap_or_else(|e| panic!("{e}"));
    let mut wide_grid = small_grid.to_wide();

    small_grid.run_robot(&directions);
    let sol1: u64 = small_grid.calculate_gps();

    wide_grid.run_robot(&directions);
    let sol2: u64 = wide_grid.calculate_gps();

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_input(input: &str) -> Result<(Grid<SmallBox>, Vec<Direction>), String> {
    let (grid, moves) = match input.trim().split_once("\n\n") {
        Some(val) => val,
        None => return Err(String::from("Wrong input")),
    };

    let mut robot: Option<Coordinate<i64>> = None;
    let mut boxes: Vec<SmallBox> = Vec::new();
    let mut walls: Vec<Coordinate<i64>> = Vec::new();
    let height = grid.trim().lines().count();
    let width = grid.trim().lines().next().unwrap().trim().chars().count();

    for (y, line) in grid.trim().lines().enumerate() {
        for (x, cell) in line.trim().chars().enumerate() {
            let (x, y) = (x as i64, y as i64);
            match cell {
                '@' => robot = Some(Coordinate::new(x, y)),
                'O' => boxes.push(SmallBox::new(x, y)),
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

//
// Grid
//
#[derive(Clone)]
struct Grid<T> {
    robot: Coordinate<i64>,
    boxes: Vec<T>,
    walls: Vec<Coordinate<i64>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: FoodBox + Copy + Add<Coordinate<i64>, Output = T> + AddAssign<Coordinate<i64>>,
{
    fn new(
        robot: Coordinate<i64>,
        boxes: Vec<T>,
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
        if let Some(next_box) = self.boxes.iter().find(|b| b.contains(&next_pos)) {
            if let Some(amount) = self.move_boxes(*next_box, direction) {
                // Move the robot by the amount that the boxes have moved
                self.robot += amount
            }
        } else {
            // If there is no box next move the robot
            self.robot += direction.delta().into()
        }
    }

    fn move_boxes(&mut self, current_box: T, direction: &Direction) -> Option<Coordinate<i64>> {
        let next_box = current_box + direction.delta().into();

        // Out of bounds check
        if next_box.position().x < 0
            || next_box.position().y < 0
            || next_box.position().x > self.width as i64
            || next_box.position().y > self.height as i64
        {
            return None;
        }

        let boxes_in_dir = current_box.boxes_in_direction(direction, self);

        for &idx in &boxes_in_dir {
            if let Some(b) = self.boxes.get_mut(idx) {
                if self
                    .walls
                    .iter()
                    .any(|w| (*b + direction.delta().into()).contains(w))
                {
                    // If there is a wall next, don't move
                    return None;
                }
            }
        }

        // Move all the boxes that are touching in the same direction
        for idx in boxes_in_dir {
            if let Some(b) = self.boxes.get_mut(idx) {
                b.move_by(direction.delta().into())
            }
        }

        Some(direction.delta().into())
    }

    fn run_robot(&mut self, directions: &[Direction]) {
        for direction in directions {
            self.move_robot(direction);
            #[cfg(all(not(test), feature = "visualize"))]
            {
                println!("{}[2J", 27 as char);
                println!("{self}");
                sleep(Duration::from_millis(4));
            }
        }
    }

    fn calculate_gps(&self) -> u64 {
        self.boxes.iter().fold(0, |acc, b| {
            acc + 100 * b.position().y as u64 + b.position().x as u64
        })
    }

    fn to_wide(&self) -> Grid<WideBox> {
        // Double x position and add one more wall to the right
        let mut new_walls = Vec::with_capacity(self.walls.len());
        for &wall in &self.walls {
            new_walls.push(wall * (2, 1));
            new_walls.push((wall * (2, 1)) + Direction::Right.delta().into());
        }

        let mut new_boxes = Vec::with_capacity(self.boxes.len());
        for &small_box in &self.boxes {
            new_boxes.push(WideBox::new(
                small_box.position() * (2, 1),
                (small_box.position() * (2, 1)) + Direction::Right.delta().into(),
            ));
        }

        Grid::new(
            self.robot * (2, 1),
            new_boxes,
            new_walls,
            self.width * 2,
            self.height,
        )
    }
}

impl<T: FoodBox> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid: Vec<Vec<char>> = vec![vec![' '; self.width]; self.height];

        let wall_char = '#';
        let box_char = T::display_char();
        let robot_char = '@';

        for b in &self.boxes {
            let cell = grid
                .get_mut(b.position().y as usize)
                .unwrap()
                .get_mut(b.position().x as usize)
                .unwrap();
            *cell = box_char;

            if box_char == WideBox::display_char() && b.position().x as usize + 1 < self.width {
                let right_cell = grid
                    .get_mut(b.position().y as usize)
                    .unwrap()
                    .get_mut(b.position().x as usize + 1)
                    .unwrap();
                *right_cell = ']';
            }
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
                } else if cell == box_char || cell == ']' {
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

//
// FoodBox
//
trait FoodBox {
    fn position(&self) -> Coordinate<i64>;

    fn display_char() -> char
    where
        Self: Sized;

    fn move_by(&mut self, amount: Coordinate<i64>)
    where
        Self: AddAssign<Coordinate<i64>>,
    {
        *self += amount
    }

    fn contains(&self, position: &Coordinate<i64>) -> bool;

    /// Returns the indices of all the boxes that are touching in the same direction
    fn boxes_in_direction(&self, direction: &Direction, grid: &Grid<Self>) -> HashSet<usize>
    where
        Self: Sized;
}

//
// SmallBox
//
#[derive(Clone, Copy, PartialEq, Debug, Default, Eq, Hash)]
struct SmallBox {
    position: Coordinate<i64>,
}

impl SmallBox {
    fn new(x: i64, y: i64) -> Self {
        Self {
            position: Coordinate::new(x, y),
        }
    }
}

impl FoodBox for SmallBox {
    fn position(&self) -> Coordinate<i64> {
        self.position
    }

    fn display_char() -> char {
        'O'
    }

    fn contains(&self, position: &Coordinate<i64>) -> bool {
        self.position == *position
    }

    fn boxes_in_direction(&self, direction: &Direction, grid: &Grid<SmallBox>) -> HashSet<usize> {
        let next_pos = self.position + direction.delta().into();
        let mut idxs: HashSet<usize> =
            HashSet::from([grid.boxes.iter().position(|b| b == self).unwrap()]);
        if let Some(b) = grid.boxes.iter().find(|b| b.contains(&next_pos)) {
            let i = b.boxes_in_direction(direction, grid);
            idxs.extend(i);
        }

        idxs
    }
}

impl Add<Coordinate<i64>> for SmallBox {
    type Output = Self;

    fn add(self, rhs: Coordinate<i64>) -> Self::Output {
        Self {
            position: self.position + rhs,
        }
    }
}

impl AddAssign<Coordinate<i64>> for SmallBox {
    fn add_assign(&mut self, rhs: Coordinate<i64>) {
        self.position += rhs
    }
}

//
// WideBox
//
#[derive(Clone, Copy, PartialEq, Debug, Default, Eq, Hash)]
struct WideBox {
    left: Coordinate<i64>,
    right: Coordinate<i64>,
}

impl WideBox {
    fn new(left: Coordinate<i64>, right: Coordinate<i64>) -> Self {
        Self { left, right }
    }
}

impl FoodBox for WideBox {
    fn position(&self) -> Coordinate<i64> {
        self.left
    }

    fn display_char() -> char {
        '['
    }

    fn contains(&self, position: &Coordinate<i64>) -> bool {
        let left_pos = *position + Direction::Left.delta().into();
        let right_pos = *position + Direction::Right.delta().into();

        // The box is wide 2 positions, so for each position there can be 2 possible WideBoxes
        (self.right == *position && self.left == left_pos)
            || (self.left == *position && self.right == right_pos)
    }

    fn boxes_in_direction(&self, direction: &Direction, grid: &Grid<Self>) -> HashSet<usize> {
        let next_positions = [
            self.left + direction.delta().into(),
            self.right + direction.delta().into(),
        ];
        let mut idxs: HashSet<usize> =
            HashSet::from([grid.boxes.iter().position(|b| b == self).unwrap()]);

        for next_pos in next_positions {
            if let Some(b) = grid
                .boxes
                .iter()
                .find(|&b| b.contains(&next_pos) && b != self)
            {
                let i = b.boxes_in_direction(direction, grid);
                idxs.extend(i);
            }
        }

        idxs
    }
}

impl Add<Coordinate<i64>> for WideBox {
    type Output = Self;

    fn add(self, rhs: Coordinate<i64>) -> Self::Output {
        Self::new(self.left + rhs, self.right + rhs)
    }
}

impl AddAssign<Coordinate<i64>> for WideBox {
    fn add_assign(&mut self, rhs: Coordinate<i64>) {
        self.left += rhs;
        self.right += rhs;
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
// Direction
//
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

    #[test]
    fn move_by_test() {
        let mut sb = SmallBox::new(0, 0);
        sb.move_by(Direction::Right.delta().into());
        assert_eq!(SmallBox::new(1, 0), sb);
    }

    #[test]
    fn small_box_contains() {
        let sb = SmallBox::new(0, 0);
        assert!(sb.contains(&Coordinate::new(0, 0)));

        let wb = WideBox::new((0, 0).into(), (1, 0).into());
        assert!(wb.contains(&Coordinate::new(1, 0)));
    }

    #[test]
    fn find_wide_box() {
        let boxes = [
            WideBox::new((0, 0).into(), (0, 1).into()),
            WideBox::new((2, 2).into(), (3, 2).into()),
        ];
        let position = Coordinate::new(2, 2);
        assert!(!boxes[0].contains(&position));
        assert!(boxes[1].contains(&position));

        let b = boxes.iter().find(|b| b.contains(&position));
        assert_eq!(b, Some(&WideBox::new((2, 2).into(), (3, 2).into())));
    }

    #[test]
    fn smallboxes_in_direction() {
        let input = "
            ###
            #O#
            #.#
            #O#
            #O#
            #O#
            #@#
            ###

            ^
            ";

        let (grid, _) = parse_input(input).unwrap();
        let b = grid
            .boxes
            .iter()
            .find(|&&b| b == SmallBox::new(1, 5))
            .unwrap();
        let boxes_above = b.boxes_in_direction(&Direction::Up, &grid);

        assert_eq!(
            boxes_above,
            HashSet::from([1, 2, 3]) /* HashSet::from([
                                         &SmallBox::new(1, 5),
                                         &SmallBox::new(1, 4),
                                         &SmallBox::new(1, 3),
                                     ]) */
        );
    }

    #[test]
    fn wideboxes_in_direction() {
        let input = "
            ###
            #@#
            #O#
            #O#
            #O#
            #.#
            #O#
            ###

            ^
            ";

        let (grid, _) = parse_input(input).unwrap();
        let grid = grid.to_wide();
        let b = grid
            .boxes
            .iter()
            .find(|&&b| b == WideBox::new((2, 2).into(), (3, 2).into()))
            .unwrap();
        let boxes_below = b.boxes_in_direction(&Direction::Down, &grid);
        assert_eq!(
            boxes_below,
            HashSet::from([
                0, 1,
                2 /* &WideBox::new((2, 2).into(), (3, 2).into()),
                  &WideBox::new((2, 3).into(), (3, 3).into()),
                  &WideBox::new((2, 4).into(), (3, 4).into()) */
            ])
        );

        let input = "
            #####
            #...#
            #@O.#
            #.OO#
            #...#
            #####

            ^
            ";

        let (grid, _) = parse_input(input).unwrap();

        let mut grid = grid.to_wide();
        let b = grid
            .boxes
            .iter()
            .find(|&&b| b == WideBox::new((4, 3).into(), (5, 3).into()))
            .unwrap();
        let boxes_below = b.boxes_in_direction(&Direction::Down, &grid);

        assert_eq!(
            boxes_below,
            HashSet::from([1 /* &WideBox::new((4, 3).into(), (5, 3).into()), */])
        );

        let boxes_right = b.boxes_in_direction(&Direction::Right, &grid);
        assert_eq!(
            boxes_right,
            HashSet::from([
                1,
                2 /* &WideBox::new((4, 3).into(), (5, 3).into()),
                  &WideBox::new((6, 3).into(), (7, 3).into()) */
            ])
        );

        grid.move_robot(&Direction::Right);
        grid.move_robot(&Direction::Right);
        /*
        ##########
        ##......##
        ##..@[].##
        ##..[][]##
        ##......##
        ##########
        */
        let b = grid
            .boxes
            .iter()
            .find(|&&b| b == WideBox::new((5, 2).into(), (6, 2).into()))
            .unwrap();
        let boxes_below = b.boxes_in_direction(&Direction::Down, &grid);

        assert_eq!(
            boxes_below,
            HashSet::from([
                0, 1,
                2 /* &WideBox::new((5, 2).into(), (6, 2).into()),
                  &WideBox::new((4, 3).into(), (5, 3).into()),
                  &WideBox::new((6, 3).into(), (7, 3).into()) */
            ])
        );
    }
}
