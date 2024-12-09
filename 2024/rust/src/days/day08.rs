use std::collections::HashMap;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    input.trim().lines().enumerate().for_each(|(y, l)| {
        l.trim().chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                let antenna = antennas.entry(c).or_default();
                antenna.push((x, y));
            }
        })
    });

    let map_width = input.trim().lines().next().unwrap_or_default().trim().len();
    let map_height = input.trim().lines().count();

    let antinodes = calculate_antinodes(&antennas, (map_width, map_height), false);

    #[cfg(all(not(test), feature = "visualize"))]
    _print_map((map_width, map_height), &antennas, &antinodes);

    let sol1: u64 = antinodes.len() as u64;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn calculate_antinodes(
    antennas: &HashMap<char, Vec<(usize, usize)>>,
    (map_width, map_height): (usize, usize),
    resonance: bool,
) -> Vec<(usize, usize)> {
    let mut antinodes: Vec<(usize, usize)> = vec![];
    antennas.iter().for_each(|(_, positions)| {
        for antenna in positions {
            for other in positions {
                let x_distance: i32 = antenna.0 as i32 - other.0 as i32;
                let y_distance: i32 = antenna.1 as i32 - other.1 as i32;

                let mut upper_antinodes: Vec<(usize, usize)> = vec![];
                loop {
                    upper_antinodes.push((
                        (antenna.0 as i32 + x_distance) as usize,
                        (antenna.1 as i32 + y_distance) as usize,
                    ));
                    if !resonance {
                        break;
                    }
                }

                let mut lower_antinodes: Vec<(usize, usize)> = vec![];
                loop {
                    lower_antinodes.push((
                        (other.0 as i32 - x_distance) as usize,
                        (other.1 as i32 - y_distance) as usize,
                    ));
                    if !resonance {
                        break;
                    }
                }

                for antinode in upper_antinodes.iter().chain(&lower_antinodes) {
                    if antinode.0 < map_width
                        && antinode.1 < map_height
                        && !antinodes.contains(antinode)
                        && *antinode != *antenna
                    {
                        antinodes.push(*antinode);
                    }
                }
            }
        }
    });
    antinodes
}

fn _print_map(
    (map_width, map_height): (usize, usize),
    antennas: &HashMap<char, Vec<(usize, usize)>>,
    antinodes: &[(usize, usize)],
) {
    let mut map: Vec<Vec<char>> = vec![];
    for y in 0..map_height {
        map.push(vec![]);
        for _ in 0..map_width {
            map[y].push('.');
        }
    }

    for antenna in antennas {
        for position in antenna.1 {
            let cell: &mut char = map
                .get_mut(position.1)
                .unwrap()
                .get_mut(position.0)
                .unwrap();
            *cell = *antenna.0;
        }
    }

    for antinode in antinodes {
        let cell: &mut char = map
            .get_mut(antinode.1)
            .unwrap()
            .get_mut(antinode.0)
            .unwrap();
        *cell = '#';
    }

    for line in map {
        for char in line {
            print!("{char}");
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn aoc_test() {
        let input = "............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............";

        let (p1, p2) = solve(input);
        assert_eq!(p1, Solution::from(14_u64));
        assert_eq!(p2, Solution::from(0_u64));
    }
}
