use std::collections::HashSet;

#[cfg(all(not(test), any(feature = "visualize", feature = "debug")))]
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

#[cfg(all(not(test), any(feature = "visualize", feature = "debug")))]
use owo_colors::OwoColorize;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let garden = parse_input(input);

    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();

    let mut price = 0;
    let mut discount_price = 0;
    for (y, line) in garden.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if visited_positions.contains(&(x, y)) {
                continue;
            }
            let mut plot_visited_positions: HashSet<(usize, usize)> = HashSet::new();

            let (plot_area, plot_perimeter) =
                get_plot_area_and_perimeter(&garden, (x, y), cell, &mut plot_visited_positions);

            let sides = get_sides(
                #[cfg(all(not(test), any(feature = "visualize", feature = "debug")))]
                &garden,
                &plot_visited_positions,
            );
            visited_positions.extend(plot_visited_positions);

            price += plot_area * plot_perimeter;
            discount_price += plot_area * sides;
        }
    }

    let sol1: u64 = price as u64;
    let sol2: u64 = discount_price as u64;

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let garden: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();
    garden
}

fn get_plot_area_and_perimeter(
    garden: &[Vec<char>],
    position: (usize, usize),
    plot_type: &char,
    visited_positions: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut area = 1;
    let mut perimeter = 4;

    // Check if position is already visited
    if visited_positions.contains(&position) {
        return (0, 0);
    } else {
        visited_positions.insert(position);

        #[cfg(all(not(test), any(feature = "visualize", feature = "debug")))]
        print_garden(garden, visited_positions);
    }

    // Search for more area vertically
    for y in [0, 2] {
        let next_y = (position.1 + y).saturating_sub(1);
        if next_y == position.1 {
            continue;
        }

        if let Some(row) = garden.get(next_y) {
            if let Some(vertical_cell) = row.get(position.0) {
                if vertical_cell == plot_type {
                    // If the current cell has one vertical neighbour remove one from perimeter
                    perimeter -= 1;
                    let (next_area, next_perimeter) = get_plot_area_and_perimeter(
                        garden,
                        (position.0, next_y),
                        plot_type,
                        visited_positions,
                    );
                    area += next_area;
                    perimeter += next_perimeter;
                }
            }
        }
    }

    // Search for more area horizontally
    for x in [0, 2] {
        let next_x = (position.0 + x).saturating_sub(1);
        if next_x == position.0 {
            continue;
        }

        if let Some(row) = garden.get(position.1) {
            if let Some(horizontal_cell) = row.get(next_x) {
                if horizontal_cell == plot_type {
                    // If the current cell has one vertical neighbour remove one from perimeter
                    perimeter -= 1;
                    let (next_area, next_perimeter) = get_plot_area_and_perimeter(
                        garden,
                        (next_x, position.1),
                        plot_type,
                        visited_positions,
                    );
                    area += next_area;
                    perimeter += next_perimeter;
                }
            }
        }
    }

    (area, perimeter)
}

fn get_sides(
    #[cfg(all(not(test), any(feature = "visualize", feature = "debug")))] garden: &[Vec<char>],
    visited_positions: &HashSet<(usize, usize)>,
) -> usize {
    let mut edges: HashSet<(usize, usize)> = HashSet::new();
    let mut sides = 0;

    for position in visited_positions.clone() {
        for y in [-1, 0, 1] {
            for x in [-1, 0, 1] {
                let next_x = position.0 as i32 + x;
                let next_y = position.1 as i32 + y;

                if (next_x < 0
                    || next_y < 0
                    || !visited_positions.contains(&(next_x as usize, next_y as usize)))
                    && !edges.contains(&position)
                {
                    edges.insert(position);
                }
            }
        }
    }

    #[cfg(all(not(test), any(feature = "visualize", feature = "debug")))]
    print_edges(garden, visited_positions, &edges);

    for edge in edges {
        let neighbours = get_neighbours_direction(edge, visited_positions);

        match neighbours.len() {
            0 => {
                sides += 4;
            }
            1 => {
                sides += 2;
            }
            2 => {
                let diagonal_neighbours =
                    get_diagonal_neighbours_direction(edge, visited_positions);

                if neighbours.contains(&Direction::North) && neighbours.contains(&Direction::East) {
                    if !diagonal_neighbours.contains(&Direction::NorthEast) {
                        sides += 2;
                    } else {
                        sides += 1;
                    }
                }

                if neighbours.contains(&Direction::North) && neighbours.contains(&Direction::West) {
                    if !diagonal_neighbours.contains(&Direction::NorthWest) {
                        sides += 2;
                    } else {
                        sides += 1;
                    }
                }

                if neighbours.contains(&Direction::South) && neighbours.contains(&Direction::East) {
                    if !diagonal_neighbours.contains(&Direction::SouthEast) {
                        sides += 2;
                    } else {
                        sides += 1;
                    }
                }

                if neighbours.contains(&Direction::South) && neighbours.contains(&Direction::West) {
                    if !diagonal_neighbours.contains(&Direction::SouthWest) {
                        sides += 2;
                    } else {
                        sides += 1;
                    }
                }
            }
            3 => {
                let diagonal_neighbours =
                    get_diagonal_neighbours_direction(edge, visited_positions);

                if !neighbours.contains(&Direction::North) {
                    if !diagonal_neighbours.contains(&Direction::SouthEast) {
                        sides += 1;
                    }

                    if !diagonal_neighbours.contains(&Direction::SouthWest) {
                        sides += 1;
                    }
                }

                if !neighbours.contains(&Direction::South) {
                    if !diagonal_neighbours.contains(&Direction::NorthEast) {
                        sides += 1;
                    }

                    if !diagonal_neighbours.contains(&Direction::NorthWest) {
                        sides += 1;
                    }
                }

                if !neighbours.contains(&Direction::East) {
                    if !diagonal_neighbours.contains(&Direction::NorthWest) {
                        sides += 1;
                    }

                    if !diagonal_neighbours.contains(&Direction::SouthWest) {
                        sides += 1;
                    }
                }

                if !neighbours.contains(&Direction::West) {
                    if !diagonal_neighbours.contains(&Direction::NorthEast) {
                        sides += 1;
                    }

                    if !diagonal_neighbours.contains(&Direction::SouthEast) {
                        sides += 1;
                    }
                }
            }
            4 => {
                let diagonal_neighbours =
                    get_diagonal_neighbours_direction(edge, visited_positions);

                sides += 4 - diagonal_neighbours.len();
            }
            _ => {}
        }
    }

    sides
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    West,
    South,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

fn get_neighbours_direction(
    position: (usize, usize),
    visited_positions: &HashSet<(usize, usize)>,
) -> Vec<Direction> {
    let mut neighbours: Vec<Direction> = Vec::new();

    for y in [-1, 1] {
        let next_y = position.1 as i32 + y;

        if next_y >= 0 && visited_positions.contains(&(position.0, next_y as usize)) {
            if y == -1 {
                neighbours.push(Direction::North);
            } else {
                neighbours.push(Direction::South);
            }
        }
    }

    for x in [-1, 1] {
        let next_x = position.0 as i32 + x;

        if next_x >= 0 && visited_positions.contains(&(next_x as usize, position.1)) {
            if x == -1 {
                neighbours.push(Direction::West);
            } else {
                neighbours.push(Direction::East);
            }
        }
    }

    neighbours
}

fn get_diagonal_neighbours_direction(
    position: (usize, usize),
    visited_positions: &HashSet<(usize, usize)>,
) -> Vec<Direction> {
    let mut neighbours: Vec<Direction> = Vec::new();

    for y in [-1, 1] {
        for x in [-1, 1] {
            let next_x = position.0 as i32 + x;
            let next_y = position.1 as i32 + y;

            if next_x >= 0
                && next_y >= 0
                && visited_positions.contains(&(next_x as usize, next_y as usize))
            {
                if x == -1 && y == -1 {
                    neighbours.push(Direction::NorthWest);
                } else if x == 1 && y == -1 {
                    neighbours.push(Direction::NorthEast);
                } else if x == -1 && y == 1 {
                    neighbours.push(Direction::SouthWest);
                } else if x == 1 && y == 1 {
                    neighbours.push(Direction::SouthEast);
                }
            }
        }
    }

    neighbours
}

#[cfg(all(not(test), any(feature = "visualize", feature = "debug")))]
fn print_garden(garden: &[Vec<char>], visited_positions: &HashSet<(usize, usize)>) {
    let mut lock = stdout().lock();
    println!("{}[2J", 27 as char);
    for (y, line) in garden.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            let offset = (cell.to_ascii_uppercase() as u8 % 65 + 1) * 4;
            let mut visited = 0;
            if visited_positions.contains(&(x, y)) {
                visited += 100;
            }
            #[cfg(feature = "visualize")]
            write!(
                lock,
                "{}",
                "██".truecolor(visited, offset + visited, 50 + visited)
            )
            .unwrap();
            #[cfg(feature = "debug")]
            write!(
                lock,
                "{}",
                cell.truecolor(visited, offset + visited, 50 + visited)
            )
            .unwrap();
        }
        writeln!(lock).unwrap();
    }
    stdout().flush().unwrap();
    sleep(Duration::from_millis(3));
}

#[cfg(all(not(test), any(feature = "visualize", feature = "debug")))]
fn print_edges(
    garden: &[Vec<char>],
    visited_positions: &HashSet<(usize, usize)>,
    edges: &HashSet<(usize, usize)>,
) {
    let mut lock = stdout().lock();
    println!("{}[2J", 27 as char);
    for (y, line) in garden.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            let offset = (cell.to_ascii_uppercase() as u8 % 65 + 1) * 4;
            let mut color: (u8, u8, u8) = (0, offset, 50);
            if edges.contains(&(x, y)) {
                color = (255, 255, 255);
            } else if visited_positions.contains(&(x, y)) {
                let highlight = 100;
                color.0 += highlight;
                color.1 += highlight;
                color.2 += highlight;
            } else {
                color = (0, offset, 50);
            }
            #[cfg(feature = "visualize")]
            write!(lock, "{}", "██".truecolor(color.0, color.1, color.2),).unwrap();
            #[cfg(feature = "debug")]
            write!(lock, "{}", cell.truecolor(color.0, color.1, color.2)).unwrap();
        }
        writeln!(lock).unwrap();
    }
    stdout().flush().unwrap();
    sleep(Duration::from_millis(100));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn perimeter_area() {
        let input = parse_input(
            "
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO",
        );

        let (o_area, o_perimeter) =
            get_plot_area_and_perimeter(&input, (0, 0), &'O', &mut HashSet::new());

        assert_eq!(o_area, 21);
        assert_eq!(o_perimeter, 36);

        let (x_area, x_perimeter) =
            get_plot_area_and_perimeter(&input, (1, 1), &'X', &mut HashSet::new());

        assert_eq!(x_area, 1);
        assert_eq!(x_perimeter, 4);

        assert_eq!(o_area + x_area * 4, 25);
    }

    #[test]
    fn small_sides() {
        let input = "
            AAAA
            BBCD
            BBCC
            EEEC";

        let (_, p2) = solve(input);
        assert_eq!(p2, Solution::from(80_u64));
    }

    #[test]
    fn small_sides2() {
        let input = "
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO";

        let (_, p2) = solve(input);
        assert_eq!(p2, Solution::from(436_u64));
    }

    #[test]
    fn small_sides3() {
        let input = "
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA";

        let (_, p2) = solve(input);
        assert_eq!(p2, Solution::from(368_u64));
    }

    #[test]
    fn e_sides() {
        let input = "
            EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE";

        let (_, p2) = solve(input);
        assert_eq!(p2, Solution::from(236_u64))
    }

    #[test]
    fn aoc_test() {
        let input = "
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE";

        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(1930_u64));
    }
}
