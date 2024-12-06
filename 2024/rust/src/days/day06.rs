use std::{collections::HashMap, fmt::Display, thread::sleep, time::Duration};

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

    let mut guard_p2 = guard.clone();

    let mut sol1: u64 = 0;
    loop {
        sol1 += guard.toggle_cell(&mut input_matrix) as u64;

        #[cfg(all(not(test), feature = "visualize"))]
        _print_map(&input_matrix, &guard, &(sol1 as usize));

        if get_next_cell(&guard, &mut input_matrix, 1).is_none() {
            break;
        }

        guard.forward(&mut input_matrix);
    }

    if input_matrix.len() <= 1 {
        sol1 = 1;
    }

    let mut sol2: u64 = 0;

    for (y, line) in input_matrix.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let mut input_matrix = input_matrix.clone();
            let cell: &mut char = input_matrix
                .get_mut(y)
                .and_then(|line| line.get_mut(x))
                .unwrap();
            if *cell == '-' || *cell == '|' || *cell == '+' {
                *cell = '#';
                if is_guard_looping(&mut guard_p2.clone(), &mut input_matrix) {
                    sol2 += 1;
                }
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn is_guard_looping(guard: &mut Guard, input_matrix: &mut [Vec<char>]) -> bool {
    let mut visited_positions: HashMap<((usize, usize), Direction), u64> = HashMap::new();
    loop {
        let visits = visited_positions
            .entry((guard.position, guard.direction))
            .or_insert(0);
        #[cfg(all(not(test), feature = "visualize"))]
        _print_map(&input_matrix, &guard, &(*visits as usize));
        *visits += 1;

        if *visits > 1 {
            return true;
        } else if get_next_cell(guard, input_matrix, 1).is_none() {
            return false;
        }

        guard.forward(input_matrix);
    }
}

fn _print_map(input_matrix: &[Vec<char>], guard: &Guard, visits: &usize) {
    print!("{}[2J", 27 as char);
    for (y, line) in input_matrix.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if guard.position == (x, y) {
                print!("{}", guard.direction);
            } else if *char == '.' {
                print!(" ");
            } else {
                print!("{char}");
            }
        }
        println!();
    }
    println!("Visits: {}", visits);
    sleep(Duration::from_millis(100));
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        write!(f, "{}", c)
    }
}

impl Direction {
    fn turn_right(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
            Direction::Right => *self = Direction::Down,
        }
    }
}

#[derive(Default, Clone, Copy)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    /// Moves the guard forward in the direction it is facing and returns the unique visits and the
    /// next
    fn forward(&mut self, grid: &mut [Vec<char>]) {
        let speed = 1;

        // While the next cell is a wall, turn right
        while let Some(cell) = get_next_cell(self, grid, speed) {
            if *cell == '#' {
                self.direction.turn_right();
            } else {
                break;
            }
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
    }

    fn toggle_cell(&self, grid: &mut [Vec<char>]) -> usize {
        let mut visited = 0;
        let wall_collision: bool = match get_next_cell(self, grid, 1) {
            Some('#') => true,
            Some(_) => false,
            None => false,
        };

        let cell = grid
            .get_mut(self.position.1)
            .and_then(|line| line.get_mut(self.position.0));

        // If the cell is not already visited, mark it as visited
        if let Some(cell) = cell {
            if *cell != 'X' && *cell != '-' && *cell != '|' && *cell != '+' {
                if wall_collision {
                    *cell = '+';
                } else {
                    match self.direction {
                        Direction::Up | Direction::Down => *cell = '|',
                        Direction::Left | Direction::Right => *cell = '-',
                    }
                }
                visited += 1;
            } else {
                *cell = '+';
            }
        }

        visited
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
        assert_eq!(p2, Solution::from(6_u64));
    }

    #[test]
    fn one_cell() {
        let input = "^";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(1_u64));
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
    }
}
