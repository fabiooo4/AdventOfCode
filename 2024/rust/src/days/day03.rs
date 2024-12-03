use crate::{Solution, SolutionPair};
use regex::Regex;

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let mul_regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    for (_, [fac1, fac2]) in mul_regex.captures_iter(input).map(|c| c.extract()) {
        sol1 += fac1.parse().unwrap_or(1) * fac2.parse().unwrap_or(1);
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
    fn single_mul() {
        let input = "do()mul(2,2)don't()mul(1,1)gibebrishmul(3,4)";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(17_u64));
        assert_eq!(p2, Solution::from(4_u64));
    }

    #[test]
    fn wrong_muls() {
        let input = "mal(2,2)mul(2, 2)mul( 2,2)mul( 2 , 2 )mul(2 2)";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(0_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn do_dont() {
        let input = "don't()mul(1,1)mul(1,1)mul(1,1)do()mul(1,1)";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(4_u64));
        assert_eq!(p2, Solution::from(1_u64));
    }

    #[test]
    fn aoc_test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(161_u64));

        let input = "mul(2,4)don't()_mul(5,5)mul(11,8)undo()?mul(8,5)";
        let (_, p2) = solve(input);
        assert_eq!(p2, Solution::from(48_u64));
    }
}
