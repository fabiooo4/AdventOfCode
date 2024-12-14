use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let machines = parse_input(input);

    const TOKENS_A: u64 = 3;
    const TOKENS_B: u64 = 1;
    let mut total_tokens: u64 = 0;
    for (a, b, p) in &machines {
        let b_presses = (a.x * p.y - a.y * p.x) / (-b.x * a.y + a.x * b.y);
        let a_presses = (p.x - b.x * b_presses) / (a.x);

        if (0..=100).contains(&a_presses)
            && (0..=100).contains(&b_presses)
            && a_presses * a.x + b_presses * b.x == p.x
            && a_presses * a.y + b_presses * b.y == p.y
        {
            total_tokens += a_presses as u64 * TOKENS_A + b_presses as u64 * TOKENS_B;
        }
    }

    let sol1: u64 = total_tokens;

    total_tokens = 0;
    for (a, b, mut p) in machines {
        p.x += 10000000000000;
        p.y += 10000000000000;
        let b_presses = (a.x * p.y - a.y * p.x) / (-b.x * a.y + a.x * b.y);
        let a_presses = (p.x - b.x * b_presses) / (a.x);

        if a_presses >= 0
            && b_presses >= 0
            && a_presses * a.x + b_presses * b.x == p.x
            && a_presses * a.y + b_presses * b.y == p.y
        {
            println!("a{a_presses} b{b_presses}");
            total_tokens += a_presses as u64 * TOKENS_A + b_presses as u64 * TOKENS_B;
        }
    }
    let sol2: u64 = total_tokens;

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug)]
struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coordinate<T> {
    fn new(x: T, y: T) -> Self {
        Coordinate { x, y }
    }
}

fn parse_input(input: &str) -> Vec<(Coordinate<i64>, Coordinate<i64>, Coordinate<i64>)> {
    input
        .trim()
        .split("\n\n")
        .map(|machine| {
            let mut machine = machine.trim().lines();
            let (ax, ay) = machine
                .next()
                .unwrap()
                .split_once(":")
                .expect("The input is not correct")
                .1
                .split_once(",")
                .expect("The input is not correct");

            let (bx, by) = machine
                .next()
                .unwrap()
                .split_once(":")
                .expect("The input is not correct")
                .1
                .split_once(",")
                .expect("The input is not correct");

            let (px, py) = machine
                .next()
                .unwrap()
                .split_once(":")
                .expect("The input is not correct")
                .1
                .split_once(",")
                .expect("The input is not correct");

            (
                Coordinate::new(
                    ax.trim()[2..].parse().unwrap(),
                    ay.trim()[2..].parse().unwrap(),
                ),
                Coordinate::new(
                    bx.trim()[2..].parse().unwrap(),
                    by.trim()[2..].parse().unwrap(),
                ),
                Coordinate::new(
                    px.trim()[2..].parse().unwrap(),
                    py.trim()[2..].parse().unwrap(),
                ),
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn aoc_test() {
        let input = "
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279";

        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(480_u64))
    }
}
