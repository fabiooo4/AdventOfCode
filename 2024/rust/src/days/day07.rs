use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let equations: Vec<(u64, Vec<u64>)> = input
        .trim()
        .lines()
        .map(|l| {
            let (result, numbers) = l.trim().split_once(":").unwrap_or_default();
            (
                result.trim().parse().unwrap_or_default(),
                numbers
                    .trim()
                    .split(" ")
                    .map(|n| n.trim().parse().unwrap_or_default())
                    .collect(),
            )
        })
        .collect();

    let mut sol1: u64 = 0;

    for equation in &equations {
        if is_equation_valid(equation) {
            sol1 += equation.0;
        }
    }

    // Your solution here...
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn is_equation_valid(equation: &(u64, Vec<u64>)) -> bool {
    // Operators is a bitmask where 1 represents multiplication and 0 represents addition
    let mut operators: usize = 2_u32.pow(equation.1.len() as u32 - 1) as usize - 1;
    loop {
        // If the nth bit of operators is 1 multiply, otherwise sum
        let result = &equation.1[1..].iter().enumerate().fold(
            *equation
                .1
                .first()
                .unwrap_or(&(get_nth_bit(operators, 0) as u64)) as u64,
            |acc: u64, (idx, n)| match get_nth_bit(operators, idx) {
                1 => acc * n,
                0 => acc + n,
                _ => acc,
            },
        );

        if equation.0 == *result {
            return true;
        }

        if operators != 0 {
            operators -= 1;
        } else {
            return false;
        }
    }
}

fn get_nth_bit(n: usize, idx: usize) -> usize {
    n >> idx & 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc_test() {
        let input = "190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20";

        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(3749_u64));
    }

    #[test]
    fn combinations() {
        let input = "190: 10 19 1 1 1 1 1 1 1 1 1";

        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(190_u64));
    }
}
