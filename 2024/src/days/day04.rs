use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let word_search: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();

    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;
    for (row, line) in word_search.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if char == &'X' {
                sol1 += count_xmas((col, row), &word_search);
            } else if char == &'A' {
                sol2 += count_x_mas((col, row), &word_search);
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

/// Checks the following pattern with an X in the middle and counts how many times XMAS or SAMX
/// appears:
/// ```text
/// #  @  #
///  # @ #
///   #@#
/// @@@X@@@
///   #@#
///  # @ #
/// #  @  #
/// ```
fn count_xmas((x, y): (usize, usize), matrix: &[Vec<char>]) -> u64 {
    let mut xmas_count = 0;
    let checks = &["XMAS", "SAMX"];

    let horizontal: String = (0..7)
        .map(|col| match matrix.get(y).unwrap().get((x - 3) + col) {
            Some(c) => c.to_string(),
            None => String::default(),
        })
        .collect();

    let vertical: String = (0..7)
        .map(|row| match matrix.get((y - 3) + row) {
            Some(str) => str.get(x).unwrap().to_string(),
            None => String::default(),
        })
        .collect();

    let diagonal_r: String = (0..7)
        .map(|row| match matrix.get((y - 3) + row) {
            Some(str) => match str.get((x - 3) + row) {
                Some(c) => c.to_string(),
                None => String::default(),
            },
            None => String::default(),
        })
        .collect();

    let diagonal_l: String = (0..7)
        .map(|row| match matrix.get((y - 3) + row) {
            Some(str) => match str.get((x + 3) - row) {
                Some(c) => c.to_string(),
                None => String::default(),
            },
            None => String::default(),
        })
        .collect();

    for check in checks {
        if horizontal.contains(check) {
            xmas_count += 1;
        }
        if vertical.contains(check) {
            xmas_count += 1;
        }
        if diagonal_r.contains(check) {
            xmas_count += 1;
        }
        if diagonal_l.contains(check) {
            xmas_count += 1;
        }
    }

    xmas_count
}

fn count_x_mas((x, y): (usize, usize), matrix: &[Vec<char>]) -> u64 {
    let mut xmas_count = 0;
    let checks = &["MAS", "SAM"];

    let diagonal_r: String = (0..3)
        .map(|row| match matrix.get((y - 1) + row) {
            Some(str) => match str.get((x - 1) + row) {
                Some(c) => c.to_string(),
                None => String::default(),
            },
            None => String::default(),
        })
        .collect();

    let diagonal_l: String = (0..3)
        .map(|row| match matrix.get((y - 1) + row) {
            Some(str) => match str.get((x + 1) - row) {
                Some(c) => c.to_string(),
                None => String::default(),
            },
            None => String::default(),
        })
        .collect();

    if (diagonal_r.contains(checks[0])
        || diagonal_r.contains(checks[1])) && (diagonal_l.contains(checks[0])
        || diagonal_l.contains(checks[1]))
    {
        xmas_count += 1;
    }

    xmas_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_xmas_test() {
        let count = count_xmas(
            (3, 3),
            &[
                "S..S..S".chars().collect(),
                ".A.A.A.".chars().collect(),
                "..MMM..".chars().collect(),
                "SAMXMAS".chars().collect(),
                "..MMM..".chars().collect(),
                ".A.A.A.".chars().collect(),
                "S..S..S".chars().collect(),
            ],
        );

        assert_eq!(count, 8);
    }

    #[test]
    fn find_x_mas_test() {
        let count = count_x_mas(
            (1, 1),
            &[
                "M.S".chars().collect(),
                ".A.".chars().collect(),
                "M.S".chars().collect(),
            ],
        );

        assert_eq!(count, 1);

        let count = count_x_mas(
            (1, 1),
            &[
                "S.M".chars().collect(),
                ".A.".chars().collect(),
                "S.M".chars().collect(),
            ],
        );

        assert_eq!(count, 1);
    }

    #[test]
    fn aoc_test() {
        let input = "
            .M.S......
            ..A..MSMS.
            .M.S.MAA..
            ..A.ASMSM.
            .M.S.M....
            ..........
            S.S.S.S.S.
            .A.A.A.A..
            M.M.M.M.M.
            ..........
            ";

        let (_, p2) = solve(input);
        assert_eq!(p2, Solution::from(9_u64));
    }

    #[test]
    fn edge() {
        let input = "
            M.S
            .A.
            S.S
            ";

        let (_, p2) = solve(input);
        assert_eq!(p2, Solution::from(0_u64));
    }
}
