use std::{
    fmt::{Debug, Display},
    thread::sleep,
    time::Duration,
};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let mut input = input.trim().to_string();
    if input.len() % 2 != 0 {
        input.push('0');
    }

    let mut files: Vec<File> = input
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .step_by(2)
        .enumerate()
        .map(|(id, pair)| {
            let size = pair
                .first()
                .unwrap_or(&'0')
                .to_string()
                .parse()
                .unwrap_or_default();
            let free = pair
                .last()
                .unwrap_or(&'0')
                .to_string()
                .parse()
                .unwrap_or_default();
            File::new(id as u64, size, free)
        })
        .collect();

    // order_files_fragments(&mut files);

    let sol1: u64 = calculate_checksum(&files);

    order_files(&mut files);

    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn _print_files(files: &Vec<File>) {
    for file in files {
        print!("{}", file);
    }
    println!();
}

fn order_files_fragments(files: &mut Vec<File>) {
    loop {
        // println!("{:?}", files);
        #[cfg(all(not(test), any(feature = "visualize", feature = "debug")))]
        _print_files(files);
        #[cfg(all(not(test), any(feature = "visualize", feature = "debug")))]
        sleep(Duration::from_millis(3));

        let last_file: &File = match files.iter().rev().find(|f| f.size > 0) {
            Some(file) => file,
            None => return,
        };
        let first_free = match files.iter().find(|f| f.free > 0) {
            Some(file) => file,
            None => return,
        };

        /* println!(
            "last: {:?}, {}",
            last_file,
            files.len() - 1 - files.iter().rev().position(|f| f == last_file).unwrap()
        );
        println!(
            "free: {:?}, {}",
            first_free,
            files.iter().position(|f| f == first_free).unwrap()
        ); */

        if (last_file == first_free
            && files.len() - 1 - files.iter().rev().position(|f| f == last_file).unwrap()
                == files.iter().position(|f| f == first_free).unwrap())
            || files.len() - 1 - files.iter().rev().position(|f| f == last_file).unwrap()
                <= files.iter().position(|f| f == first_free).unwrap()
        {
            return;
        }

        let last_file: &mut File = files.iter_mut().rev().find(|f| f.size > 0).unwrap();

        last_file.size = match last_file.size.checked_sub(1) {
            Some(size) => size,
            None => return,
        };
        last_file.free += 1;

        let last_id = last_file.id;

        let free_idx = files.iter().position(|f| f.free > 0).unwrap() + 1;
        let first_free = match files.iter_mut().find(|f| f.free > 0) {
            Some(file) => file,
            None => return,
        };

        if first_free.id == last_id {
            first_free.size += 1;
            first_free.free = first_free.free.saturating_sub(1);
        } else {
            let new_file = File::new(last_id, 1, first_free.free.saturating_sub(1));

            first_free.free = 0;

            // println!("{free_idx}");
            // println!("new: {:?}", new_file);

            files.insert(free_idx, new_file);
        }
    }
}

fn order_files(files: &mut Vec<File>) {
    let current_order = files.clone();
    for file in current_order.iter().rev() {
        // println!("{:?}", files);
        #[cfg(all(not(test), any(feature = "visualize", feature = "debug")))]
        _print_files(files);
        #[cfg(all(not(test), any(feature = "visualize", feature = "debug")))]
        sleep(Duration::from_millis(3));

        let last_file = match files.pop() {
            Some(file) => file,
            None => return,
        };

        let first_free: &mut u64 = match files.iter_mut().find(|f| f.free >= last_file.size) {
            Some(file) => &mut file.free,
            None => continue,
        };

        *first_free = 0;
        files.last_mut().unwrap().free += last_file.free;

        let free_idx = files.iter().position(|f| f.free >= last_file.size).unwrap();
        files.insert(
            free_idx,
            File::new(
                last_file.id,
                last_file.size,
                first_free.saturating_sub(last_file.size),
            ),
        );
    }
}

fn calculate_checksum(files: &[File]) -> u64 {
    let mut count: u64 = 0;
    files.iter().fold(0, |acc, file| {
        let mut checksum: u64 = 0;
        for _ in 0..file.size {
            checksum += count * file.id;
            count += 1;
            // println!("{},{count},{checksum},{acc}", file.id);
        }
        acc + checksum
    })
}

#[derive(PartialEq, Copy, Clone, Default)]
struct File {
    id: u64,
    size: u64,
    free: u64,
}

impl File {
    fn new(id: u64, size: u64, free: u64) -> File {
        File { id, size, free }
    }
}

impl Display for File {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.size {
            #[cfg(all(not(test), feature = "visualize"))]
            write!(_f, "█")?;
            #[cfg(all(not(test), feature = "debug"))]
            write!(_f, "{}", self.id)?;
        }
        for _ in 0..self.free {
            #[cfg(all(not(test), feature = "visualize"))]
            write!(_f, " ")?;
            #[cfg(all(not(test), feature = "debug"))]
            write!(_f, ".")?;
        }

        Ok(())
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.id, self.size, self.free)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn small() {
        let input = "12345";
        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(60_u64));
    }

    #[test]
    fn medium() {
        let input = "233313312141413140211";
        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(2132_u64));
    }

    #[test]
    fn medium2() {
        let input = "111010101010101010101";
        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(340_u64));
    }

    #[test]
    fn aoc_test() {
        let input = "2333133121414131402";
        let (p1, _) = solve(input);
        assert_eq!(p1, Solution::from(1928_u64));
    }
}
