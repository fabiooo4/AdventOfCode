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
    let mut sol2: u64 = 0;

    for equation in &equations {
        if is_equation_valid(equation, &[Operator::Add, Operator::Multiply]) {
            sol1 += equation.0;
        }

        if is_equation_valid(
            equation,
            &[Operator::Add, Operator::Multiply, Operator::Concatenate],
        ) {
            sol2 += equation.0;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(PartialEq, Default, Debug)]
enum Operator {
    #[default]
    Add,
    Multiply,
    Concatenate,
}

fn is_equation_valid(equation: &(u64, Vec<u64>), operators: &[Operator]) -> bool {
    // Operators is a mask where each digit represents a operator
    let mut mask: usize = operators.len().pow(equation.1.len() as u32 - 1) - 1;

    // Calculate the digits to maintain the leading zeros
    let digits = convert_to_base(mask, operators.len(), 0).len();

    loop {
        // Execute the operator given by the mask and return if valid
        let result = &equation.1[1..].iter().enumerate().fold(
            *equation.1.first().unwrap_or(&0),
            |acc: u64, (idx, n)| match get_nth_operator(mask, idx, operators.len(), digits) {
                Operator::Add if operators.contains(&Operator::Add) => acc + n,
                Operator::Multiply if operators.contains(&Operator::Add) => acc * n,
                Operator::Concatenate if operators.contains(&Operator::Concatenate) => {
                    format!("{}{}", acc, n).parse().unwrap()
                }

                _ => acc,
            },
        );

        if equation.0 == *result {
            return true;
        }

        if mask != 0 {
            mask -= 1;
        } else {
            return false;
        }
    }
}

fn get_nth_operator(n: usize, idx: usize, base: usize, digits: usize) -> Operator {
    let n = convert_to_base(n, base, digits);
    match n
        .to_string()
        .chars()
        .nth(idx)
        .unwrap_or_default()
        .to_digit(base as u32)
    {
        Some(d) => match d {
            0 => Operator::Add,
            1 => Operator::Multiply,
            2 => Operator::Concatenate,
            _ => Operator::default(),
        },
        None => Operator::default(),
    }
}

fn convert_to_base(mut x: usize, base: usize, digits: usize) -> String {
    let mut result = vec![];

    loop {
        let m = x % base;
        x /= base;

        result.push(std::char::from_digit(m as u32, base as u32).unwrap());
        if x == 0 {
            break;
        }
    }

    if result.len() < digits {
        (result.len()..digits).for_each(|_| {
            result.push('0');
        });
    }

    result.into_iter().rev().collect()
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

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(3749_u64));
        assert_eq!(p2, Solution::from(11387_u64));
    }

    #[test]
    fn combinations() {
        let input = "190: 10 19 1 1 1 1 1 1 1 1 1";

        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(190_u64));
    }
}
