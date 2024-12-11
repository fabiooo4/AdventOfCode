use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    num::{IntErrorKind, ParseIntError},
    ops::{Div, RangeBounds, Rem},
    str::FromStr,
};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let stones: Vec<Stone> = input
        .trim()
        .split(" ")
        .map(|n| n.parse().unwrap_or_default())
        .collect();

    println!("Part 1:");
    let sol1: u64 = blink(&stones, 25);
    println!("\nPart 2:");
    let sol2: u64 = blink(&stones, 75);

    (Solution::from(sol1), Solution::from(sol2))
}

// type StoneCache<G> = HashMap<Stone, G>;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Stone(u64);

impl Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Stone {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Stone(s.parse()?))
    }
}

fn blink(stones: &[Stone], depth: usize) -> u64 {
    let mut count = 0;

    for (progress, stone) in stones.iter().enumerate() {
        println!("{}/{}", progress + 1, stones.len());
        count += stone.count_stones_after_blinks(depth) as u64;
    }

    count
}

impl Stone {
    fn apply_rule_one(&self) -> Option<Stone> {
        if *self == Stone(0) {
            return Some(Stone(1));
        }

        None
    }

    fn apply_rule_two(&self) -> Option<(Stone, Stone)> {
        self.split_middle()
    }

    fn apply_rule_three(&self) -> Stone {
        Stone(self.0 * 2024)
    }

    fn blink_once(&self) -> (Stone, Option<Stone>) {
        if let Some(stone_one) = self.apply_rule_one() {
            (stone_one, None)
        } else if let Some(stone_two) = self.apply_rule_two() {
            (stone_two.0, Some(stone_two.1))
        } else {
            (self.apply_rule_three(), None)
        }
    }

    fn count_stones_after_blinks(&self, depth: usize) -> usize {
        let (left_stone, right_stone) = self.blink_once();

        if depth == 1 {
            if right_stone.is_some() {
                return 2;
            } else {
                return 1;
            }
        }

        let mut count = left_stone.count_stones_after_blinks(depth - 1);
        if let Some(right_stone) = right_stone {
            count += right_stone.count_stones_after_blinks(depth - 1);
        }

        count
    }

    fn split_middle(&self) -> Option<(Stone, Stone)> {
        let mut digits = 0;

        let mut tmp = self.0;
        while tmp != 0 {
            tmp /= 10;
            digits += 1;
        }

        if digits == 0 || digits % 2 != 0 {
            return None;
        }

        let mut right = 0;
        let mut left = self.0;
        for idx in 0..(digits / 2) {
            right += 10_u64.pow(idx) * (left % 10);

            left /= 10;
        }

        Some((Stone(left), Stone(right)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn split_middle_test() {
        assert_eq!(Stone(1234).split_middle(), Some((Stone(12), Stone(34))));
        assert_eq!(Stone(123).split_middle(), None);
        assert_eq!(Stone(9999).split_middle(), Some((Stone(99), Stone(99))));
        assert_eq!(Stone(00).split_middle(), None);
    }

    #[test]
    fn first_rule() {
        assert_eq!(Stone(0).apply_rule_one(), Some(Stone(1)));
        assert_eq!(Stone(1).apply_rule_one(), None);
    }

    #[test]
    fn second_rule() {
        assert_eq!(Stone(0).apply_rule_two(), None);
        assert_eq!(Stone(1).apply_rule_two(), None);
        assert_eq!(Stone(22).apply_rule_two(), Some((Stone(2), Stone(2))));
        assert_eq!(Stone(10).apply_rule_two(), Some((Stone(1), Stone(0))));
        assert_eq!(Stone(1234).apply_rule_two(), Some((Stone(12), Stone(34))));
    }

    #[test]
    fn third_rule() {
        assert_eq!(Stone(0).apply_rule_three(), Stone(0));
        assert_eq!(Stone(1).apply_rule_three(), Stone(2024));
    }

    #[test]
    fn aoc_test() {
        let input = "125 17";

        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(55312_u64))
    }
}
