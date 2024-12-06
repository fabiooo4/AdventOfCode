use std::{str::FromStr, thread::sleep, time::Duration};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let mut input_matrix: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();

    let mut guard = Guard::default();

    for (y, line) in input_matrix.iter().enumerate() {
        match line
            .iter()
            .position(|&c| c == '^' || c == '<' || c == 'v' || c == '>')
        {
            Some(x) => {
                guard.position.0 = x;
                guard.position.1 = y;
                guard.direction = Direction::try_from(input_matrix[y][x]).unwrap_or_default();
                break;
            }
            None => continue,
        }
    }

    let mut sol1: u64 = 0;
    while let Some(visits) = guard.forward(&mut input_matrix) {
        sol1 += visits as u64;

        #[cfg(all(not(test), feature = "visualize"))]
        _print_map(&input_matrix, &(sol1 as usize));
    }

    if input_matrix.len() <= 1 {
        sol1 = 1;
    }

    let sol2: u64 = 0;

    for (y, line) in input_matrix.iter().enumerate() {
        match line
            .iter()
            .position(|&c| c == '^' || c == '<' || c == 'v' || c == '>')
        {
            Some(x) => {
                guard.position.0 = x;
                guard.position.1 = y;
                guard.direction = Direction::try_from(input_matrix[y][x]).unwrap_or_default();
                break;
            }
            None => continue,
        }
    }

    for (y, _) in input_matrix.iter().enumerate() {
        // println!("{}", input_matrix[y][guard.position.0]);
        if input_matrix[y][guard.position.0] == '#' {
            // println!("wall_y:{y}");
            if let Some(line) = input_matrix.get(y + 1) {
                if let Some(x) = line.iter().position(|&c| c == '#') {
                    // println!("wall_x:{x}");
                }
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn _print_map(input_matrix: &Vec<Vec<char>>, visits: &usize) {
    print!("{}[2J", 27 as char);
    for line in input_matrix {
        for char in line {
            if *char == '.' {
                print!(" ");
            } else {
                print!("{char}");
            }
        }
        println!();
    }
    println!("Visits: {}", visits);
    sleep(Duration::from_millis(300));
}

#[derive(Default)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(String::from(
                "The string must be either of: '^', 'v', '<', '>'",
            )),
        }
    }
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn turn_right(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
            Direction::Right => *self = Direction::Down,
        }
    }
}

#[derive(Default)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    /// Moves the guard forward in the direction it is facing and returns the unique visits and the
    /// next
    fn forward(&mut self, grid: &mut [Vec<char>]) -> Option<usize> {
        let speed = 1;
        let mut visits = 0;

        let mut next_cell = get_next_cell(self, grid, speed);

        let mut intersection = false;
        if next_cell != Some(&mut 'X')
            && next_cell != Some(&mut '-')
            && next_cell != Some(&mut '|')
            && next_cell != Some(&mut '+')
        {
            visits += 1;
        }

        while next_cell == Some(&mut '#') {
            self.direction.turn_right();
            intersection = true;

            next_cell = get_next_cell(self, grid, speed);
            if next_cell == Some(&mut 'X')
                || next_cell == Some(&mut '-')
                || next_cell == Some(&mut '|')
                || next_cell == Some(&mut '+')
            {
                visits -= 1;
            }
        }

        let mut next_value = '-';
        if let Some(next_cell) = next_cell {
            next_value = *next_cell;
            *next_cell = self.direction.to_char();
        }

        let current_cell: &mut char = grid.get_mut(self.position.1)?.get_mut(self.position.0)?;
        match self.direction.to_char() {
            '>' | '<' => {
                *current_cell = '-';
            }
            '^' | 'v' => *current_cell = '|',
            _ => *current_cell = 'X',
        }

        match self.direction {
            Direction::Up => {
                if let Some(res) = self.position.1.checked_sub(speed) {
                    self.position.1 = res;
                } else {
                    self.position.1 = grid.len();
                }
            }
            Direction::Down => self.position.1 += speed,
            Direction::Left => {
                if let Some(res) = self.position.0.checked_sub(speed) {
                    self.position.0 = res;
                } else {
                    self.position.0 = grid.len();
                }
            }
            Direction::Right => self.position.0 += speed,
        }

        if intersection || (next_value == '-' && next_value == '|') {
            let previous_cell: &mut char = match self.direction {
                Direction::Up => grid
                    .get_mut(self.position.1 + speed)?
                    .get_mut(self.position.0)?,
                Direction::Down => grid
                    .get_mut(self.position.1.checked_sub(speed)?)?
                    .get_mut(self.position.0)?,
                Direction::Left => grid
                    .get_mut(self.position.1)?
                    .get_mut(self.position.0 + speed)?,
                Direction::Right => grid
                    .get_mut(self.position.1)?
                    .get_mut(self.position.0.checked_sub(speed)?)?,
            };

            *previous_cell = '+';
        }

        Some(visits)
    }
}

fn get_next_cell<'a>(
    guard: &Guard,
    grid: &'a mut [Vec<char>],
    speed: usize,
) -> Option<&'a mut char> {
    Some(match guard.direction {
        Direction::Up => grid
            .get_mut(guard.position.1.checked_sub(speed)?)?
            .get_mut(guard.position.0)?,
        Direction::Down => grid
            .get_mut(guard.position.1 + speed)?
            .get_mut(guard.position.0)?,
        Direction::Left => grid
            .get_mut(guard.position.1)?
            .get_mut(guard.position.0.checked_sub(speed)?)?,
        Direction::Right => grid
            .get_mut(guard.position.1)?
            .get_mut(guard.position.0 + speed)?,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc_test() {
        let input = "
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(41_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn one_cell() {
        let input = "^";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(1_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn edge_wall() {
        let input = "
            #
            .
            ^
            ";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(2_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn consecutive_walls() {
        let input = "
            .#.
            #<.
            ...
            ";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(2_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn test1() {
        let input = "
            .....
            ..#..
            ..^.#
            ...#.
            .....
            ";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(4_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn test2() {
        let input = "
            ....#
            ...#.
            ..^..
            .....
            ..#..
            ";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(3_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }
}
