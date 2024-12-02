use std::ops::ControlFlow;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    if input.is_empty() {
        return (0_u64.into(), 0_u64.into());
    }

    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;

    let reports: Vec<Vec<u64>> = input
        .split('\n')
        .map(|report| {
            report
                .split_whitespace()
                .filter_map(|level| level.trim().parse().ok())
                .collect()
        })
        .collect();

    for report in reports.iter() {
        match is_report_safe(report) {
            true => {
                sol1 += 1;
                sol2 += 1
            }
            false => {
                for (removed_idx, _) in report.iter().enumerate() {
                    let mut report_removed = report.clone();
                    report_removed.remove(removed_idx);

                    if is_report_safe(&report_removed) {
                        sol2 += 1;
                        break;
                    }
                }
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn is_report_safe(report: &[u64]) -> bool {
    let first = match report.first() {
        Some(n) => n,
        None => return false,
    };

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

    safe
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
        assert_eq!(p2, Solution::from(6_u64));
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
        assert_eq!(p2, Solution::from(6_u64));
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
        assert_eq!(p2, Solution::from(4_u64));
    }

    #[test]
    fn one_level() {
        let input = "2";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(1_u64));
        assert_eq!(p2, Solution::from(1_u64));
    }

    #[test]
    fn combination() {
        let input = "1 9 9 2 3 9 4";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(0_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }
}
