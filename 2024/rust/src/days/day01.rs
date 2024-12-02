use std::collections::HashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    //////////
    // Part 1
    //////////
    let res: Vec<&str> = input.split_whitespace().collect();

    let mut left_list: Vec<u64> = vec![];
    let mut right_list: Vec<u64> = vec![];

    res.windows(2).step_by(2).for_each(|w| {
        left_list.push(
            w[0].parse()
                .unwrap_or_else(|e| panic!("The input is not formed correctly: {e}")),
        );
        right_list.push(
            w[1].parse()
                .unwrap_or_else(|e| panic!("The input is not formed correctly: {e}")),
        );
    });

    left_list.sort();
    right_list.sort();

    let sol1: u64 = left_list
        .iter()
        .zip(right_list.iter())
        .fold(0_u64, |acc, (l, r)| acc + l.abs_diff(*r));

    //////////
    // Part 2
    //////////
    let mut left_map: HashMap<u64, usize> = HashMap::new();
    for num in left_list {
        let count = left_map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut right_map: HashMap<u64, usize> = HashMap::new();
    for num in right_list {
        let count = right_map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut sol2: u64 = 0;
    for (key,occurency) in left_map {
        sol2 += key * occurency as u64 * *right_map.get(&key).unwrap_or(&0) as u64;
    }


    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_input() {
        let (p1, p2) = solve("");
        assert_eq!(p1, Solution::from(0_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn all_zeros() {
        let input = "0  0
         1  1
         2  2
         3  3";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(0_u64));
        assert_eq!(p2, Solution::from(6_u64));
    }

    #[test]
    fn all_zeros_except_one() {
        let input = "0  0
         1  1
         2  2
         3  4";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(1_u64));
        assert_eq!(p2, Solution::from(3_u64));
    }

    #[test]
    fn unordered() {
        let input = "4  4
         3  3
         2  2
         1  1";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(0_u64));
        assert_eq!(p2, Solution::from(10_u64));
    }

    #[test]
    fn random() {
        let input = "3   4
         4   3
         2   5
         1   3
         3   9
         3   3";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(11_u64));
        assert_eq!(p2, Solution::from(31_u64));
    }
}
