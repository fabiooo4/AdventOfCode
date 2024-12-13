use std::collections::HashMap;

#[cfg(all(not(test), feature = "visualize"))]
use std::{thread::sleep, time::Duration};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let map: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_string().parse().unwrap_or(11))
                .collect()
        })
        .collect();

    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 0 {
                sol1 += calculate_score(&map, (x, y)) as u64;
                sol2 += calculate_rating(&map, (x, y)) as u64;
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn calculate_score(map: &[Vec<usize>], position: (usize, usize)) -> usize {
    let mut tops = HashMap::new();
    search_tops(map, position, &mut tops);

    tops.len()
}

fn calculate_rating(map: &[Vec<usize>], position: (usize, usize)) -> usize {
    let mut tops = HashMap::new();
    search_tops(map, position, &mut tops);

    tops.values().sum()
}

fn search_tops(
    map: &[Vec<usize>],
    position: (usize, usize),
    tops: &mut HashMap<(usize, usize), usize>,
) {
    let current_cell = map.get(position.1).unwrap().get(position.0).unwrap();
    if current_cell == &9 {
        let top = tops.entry(position).or_insert(0);
        *top += 1;
    }

    #[cfg(all(not(test), feature = "visualize"))]
    print_map(map, position);

    for y in [0, 2] {
        if let Some(row) = map.get((position.1 + y).saturating_sub(1)) {
            if let Some(vertical_cell) = row.get(position.0) {
                if *vertical_cell == *current_cell + 1 {
                    search_tops(map, (position.0, (position.1 + y) - 1), tops);
                }
            }
        }
    }

    for x in [0, 2] {
        if let Some(row) = map.get(position.1) {
            if let Some(horizontal_cell) = row.get((position.0 + x).saturating_sub(1)) {
                if *horizontal_cell == *current_cell + 1 {
                    search_tops(map, ((position.0 + x) - 1, position.1), tops);
                }
            }
        }
    }
}

#[cfg(all(not(test), feature = "visualize"))]
fn print_map(map: &[Vec<usize>], position: (usize, usize)) {
    for row in 0..map.len() {
        'next_cell: for col in 0..map.first().unwrap().len() {
            if col == position.0 && row == position.1 {
                print!("█");
            } else {
                if map[row][col] == 11 {
                    print!(".");
                } else {
                    print!("{}", map[row][col]);
                }
            }
        }
        println!();
    }
    println!();
    sleep(Duration::from_millis(3));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn small() {
        let input = "
            0123
            1234
            8765
            9876";

        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(1_u64));
    }

    #[test]
    fn split_trail() {
        let input = "
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9";

        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(2_u64));
    }

    #[test]
    fn multiple_tops() {
        let input = "
            ..90..9
            ...1.98
            ...2..7
            6543456
            765.987
            876....
            987....";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(4_u64));
        assert_eq!(p2, Solution::from(13_u64));
    }

    #[test]
    fn multiple_trailheads() {
        let input = "
            10..9..
            2...8..
            3...7..
            4567654
            ...8..3
            ...9..2
            .....01";

        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(3_u64));
    }

    #[test]
    fn single_top_rating() {
        let input = "
            .....0.
            ..4321.
            ..5..2.
            ..6543.
            ..7..4.
            ..8765.
            ..9....";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(1_u64));
        assert_eq!(p2, Solution::from(3_u64));
    }

    #[test]
    fn multiple_top_rating() {
        let input = "
            012345
            123456
            234567
            345678
            4.6789
            56789.";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(2_u64));
        assert_eq!(p2, Solution::from(227_u64));
    }

    #[test]
    fn aoc_example() {
        let input = "
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(36_u64));
        assert_eq!(p2, Solution::from(81_u64));
    }
}
