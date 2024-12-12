use std::collections::HashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let mut stones: HashMap<u64, u64> = HashMap::new();

    input.trim().split(" ").for_each(|n| {
        let num = n.parse().unwrap();
        *stones.entry(num).or_default() += 1;
    });

    let mut stones_p1 = stones.clone();
    for _ in 0..25 {
        stones_p1 = blink(stones_p1);
    }

    let sol1: u64 = stones_p1.values().sum();

    for _ in 0..75 {
        stones = blink(stones);
    }

    let sol2: u64 = stones.values().sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next_blink: HashMap<u64, u64> = HashMap::with_capacity(stones.len());

    for (stone, amount) in stones {
        match stone {
            0 => *next_blink.entry(1).or_default() += amount,
            _ => {
                if let Some((left_stone, right_stone)) = split_middle(stone) {
                    *next_blink.entry(left_stone).or_default() += amount;
                    *next_blink.entry(right_stone).or_default() += amount;
                } else {
                    *next_blink.entry(stone * 2024).or_default() += amount;
                }
            }
        }
    }

    next_blink
}

fn split_middle(num: u64) -> Option<(u64, u64)> {
    let mut digits = 0;

    let mut tmp = num;
    while tmp != 0 {
        tmp /= 10;
        digits += 1;
    }

    if digits == 0 || digits % 2 != 0 {
        return None;
    }

    let mut right = 0;
    let mut left = num;
    for idx in 0..(digits / 2) {
        right += 10_u64.pow(idx) * (left % 10);

        left /= 10;
    }

    Some((left, right))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn split_middle_test() {
        assert_eq!(split_middle(1234), Some((12, 34)));
        assert_eq!(split_middle(123), None);
        assert_eq!(split_middle(9999), Some((99, 99)));
        assert_eq!(split_middle(00), None);
    }

    #[test]
    fn aoc_test() {
        let input = "125 17";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(55312_u64));
        assert_eq!(p2, Solution::from(65601038650482_u64));
    }
}
