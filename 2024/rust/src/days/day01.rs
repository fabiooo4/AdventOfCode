use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
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
}
