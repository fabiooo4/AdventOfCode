use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    //////////
    // Part 1
    //////////
    if input.is_empty() {
        return (0_u64.into(), 0_u64.into());
    }

    let mut safe_count = 0;

    let reports: Vec<Vec<u64>> = input
        .split('\n')
        .map(|report| {
            report
                .split_whitespace()
                .filter_map(|level| level.parse().ok())
                .collect()
        })
        .collect();

    for report in reports {
        let first = report.first().unwrap_or(&0);
        let increasing: bool = (*first as i64 - *report.get(1).unwrap_or(&0) as i64).is_negative();

        let mut safe = true;

        for couple in report.windows(2) {
            let diff = couple[0].abs_diff(couple[1]);

            // Unsafe report (not differing by 1,2 or 3)
            if !(1..=3).contains(&diff) {
                safe = false;
                break;
            }

            // Unsafe report (not always increasing or decreasing)
            if (increasing && couple[0] > couple[1]) || (!increasing && couple[0] < couple[1]) {
                safe = false;
                break;
            }
        }

        if safe {
            safe_count += 1;
        }
    }

    let sol1: u64 = safe_count;

    //////////
    // Part 2
    //////////
    let sol2: u64 = 0;

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
    fn increasing() {
        let input = "1 3 6 7 9
         0 1 2
         0 2 4 6 8
         0 3 6 9 12 13
         1 2 1
         1 2 7 8 9
         1 3 2 4 5";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(4_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn decreasing() {
        let input = "7 6 4 2 1
         4 3 2 1 0
         8 6 4 2 0
         12 9 6 3 0
         4 3 4 2 1
         9 7 6 2 1
         8 6 4 4 1";
        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(4_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn all_zeros() {
        let input = "0 0 0 0";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(0_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn aoc_test() {
        let input = "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(2_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn one_level() {
        let input = "2";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(1_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }

    #[test]
    fn combination() {
        let input = "0 1 2 safe
            0 2 4 6 8      safe
            0 3 6 9 12 13  safe
            1 2 1          unsafe
            4 3 2 1 0      safe
            8 6 4 2 0      safe
            12 9 6 3 0     safe
            4 3 4 2 1      unsafe
            0 0 0 0        safe
            7 6 4 2 1      safe
            1 2 7 8 9      unsafe
            9 7 6 2 1      unsafe
            1 3 2 4 5      unsafe
            8 6 4 4 1      unsafe
            1 3 6 7 9      unsafe
            2              safe";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(9_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }
}
