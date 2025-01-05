use image::ImageBuffer;

use std::{fs, sync::mpsc::channel};

#[cfg(all(not(test), feature = "visualize"))]
use owo_colors::OwoColorize;
#[cfg(all(not(test), feature = "visualize"))]
use std::{collections::HashMap, thread::sleep, time::Duration};

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> SolutionPair {
    let _ = fs::remove_dir_all("target/day14");

    let mut robots = parse_input(input);
    let grid: Grid = Grid::new(101, 103);
    let seconds = 100;

    let mut robots_p1 = robots.clone();
    for _ in 0..seconds {
        for robot in robots_p1.iter_mut() {
            *robot = robot.accelerate(grid.bounds());
        }
        #[cfg(all(not(test), feature = "visualize"))]
        grid.print_robots(&robots_p1);
    }

    let sol1: u64 = grid.safety_factor(&robots) as u64;
    let sol2: String = String::from("Check the target folder for the images generated");

    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    println!("Generating images for part 2");
    println!("Press Ctrl-C to stop generating...");
    let mut seconds = 0;
    loop {
        for robot in robots.iter_mut() {
            *robot = robot.accelerate(grid.bounds());
        }

        seconds += 1;
        grid.create_map_image(&robots, seconds);


        // Stop generaing images if ctrl-c is recieved
        if rx.try_recv().is_ok() {
            break;
        }
    }
    println!("\nElapsed: {}", seconds);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coordinate<T> {
    fn new(x: T, y: T) -> Self {
        Coordinate { x, y }
    }
}

impl<T> From<(T, T)> for Coordinate<T> {
    fn from(tuple: (T, T)) -> Self {
        Coordinate::new(tuple.0, tuple.1)
    }
}

struct Grid {
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid { width, height }
    }

    /// Returns the bounds of the grid as (width, height)
    fn bounds(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    #[cfg(all(not(test), feature = "visualize"))]
    fn print_robots(&self, robots: &Vec<Robot>) {
        println!("{}[2J", 27 as char);
        let mut robots_map: HashMap<Coordinate<i64>, usize> = HashMap::with_capacity(robots.len());

        for robot in robots {
            let pos = robots_map.entry(robot.position).or_insert(0);
            *pos += 1;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let mut is_robot = false;
                for robot in robots {
                    if Coordinate::from((x as i64, y as i64)) == robot.position {
                        let robot_count = robots_map.get(&robot.position).unwrap_or(&1);
                        print!("{:2}", robot_count.on_red());
                        is_robot = true;
                        break;
                    }
                }

                if !is_robot {
                    if y % 2 == 0 {
                        if x % 2 == 0 {
                            print!("{}", "  ".on_truecolor(60, 60, 60));
                        } else {
                            print!("{}", "  ".on_truecolor(50, 50, 50));
                        }
                    } else if x % 2 != 0 {
                        print!("{}", "  ".on_truecolor(60, 60, 60));
                    } else {
                        print!("{}", "  ".on_truecolor(50, 50, 50));
                    }
                }
            }
            println!();
        }
        println!("Safety factor: {}", self.safety_factor(robots));
        sleep(Duration::from_millis(5));
    }

    fn create_map_image(&self, robots: &[Robot], seconds: u64) {
        // Create an image
        let mut imgbuf = ImageBuffer::new(self.width as u32, self.height as u32);

        for y in 0..self.height {
            for x in 0..self.width {
                let mut is_robot = false;
                for robot in robots {
                    if Coordinate::from((x as i64, y as i64)) == robot.position {
                        is_robot = true;
                        break;
                    }
                }

                let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
                if !is_robot {
                    *pixel = image::Rgb([0_u8, 0, 0]);
                } else {
                    *pixel = image::Rgb([255_u8, 255, 255]);
                }
            }
        }

        // Create the empty directory to store all the generated images
        fs::create_dir_all("target/day14").expect("Unable to create folder to generate images");

        imgbuf
            .save(format!("target/day14/{}.png", seconds))
            .expect("The target folder doesn't exist");
    }

    fn robots_in_quadrant(&self, quadrant: Quadrant, robots: &[Robot]) -> usize {
        let mut count = 0;
        match quadrant {
            Quadrant::TopLeft => {
                for robot in robots {
                    if robot.position.x < (self.width / 2) as i64
                        && robot.position.y < (self.height / 2) as i64
                    {
                        count += 1;
                    }
                }
            }
            Quadrant::TopRight => {
                for robot in robots {
                    if (robot.position.x as f64) > (self.width as f64) / 2.
                        && robot.position.y < (self.height / 2) as i64
                    {
                        count += 1;
                    }
                }
            }
            Quadrant::BottomLeft => {
                for robot in robots {
                    if robot.position.x < (self.width / 2) as i64
                        && (robot.position.y as f64) > (self.height as f64) / 2.
                    {
                        count += 1;
                    }
                }
            }
            Quadrant::BottomRight => {
                for robot in robots {
                    if (robot.position.x as f64) > (self.width as f64) / 2.
                        && (robot.position.y as f64) > (self.height as f64) / 2.
                    {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn safety_factor(&self, robots: &[Robot]) -> usize {
        self.robots_in_quadrant(Quadrant::TopLeft, robots)
            * self.robots_in_quadrant(Quadrant::TopRight, robots)
            * self.robots_in_quadrant(Quadrant::BottomLeft, robots)
            * self.robots_in_quadrant(Quadrant::BottomRight, robots)
    }
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Robot {
    position: Coordinate<i64>,
    velocity: Coordinate<i64>,
}

impl Robot {
    fn new<P: Into<Coordinate<i64>>, V: Into<Coordinate<i64>>>(position: P, velocity: V) -> Self {
        Self {
            position: position.into(),
            velocity: velocity.into(),
        }
    }

    /// Moves the robot to the next position using its velocity
    fn accelerate(self, bounds: (usize, usize)) -> Robot {
        let next_x = match self.position.x.saturating_add(self.velocity.x) % bounds.0 as i64 >= 0 {
            true => self.position.x.saturating_add(self.velocity.x) % bounds.0 as i64,
            false => {
                bounds.0 as i64 + self.position.x.saturating_add(self.velocity.x) % bounds.0 as i64
            }
        };

        let next_y = match self.position.y.saturating_add(self.velocity.y) % bounds.1 as i64 >= 0 {
            true => self.position.y.saturating_add(self.velocity.y) % bounds.1 as i64,
            false => {
                bounds.1 as i64 + self.position.y.saturating_add(self.velocity.y) % bounds.1 as i64
            }
        };

        Robot::new((next_x, next_y), self.velocity)
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (position, velocity) = l.trim().split_once(' ').unwrap();
            let position: Coordinate<i64> = position[2..]
                .trim()
                .split_once(',')
                .map(|(x, y)| Coordinate::new(x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            let velocity: Coordinate<i64> = velocity[2..]
                .trim()
                .split_once(',')
                .map(|(x, y)| Coordinate::new(x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();

            Robot::new(position, velocity)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn move_robot() {
        let grid: Grid = Grid::new(3, 3);
        let robot = Robot::new((0, 0), (1, 1));

        let robot = robot.accelerate(grid.bounds());
        assert_eq!(Robot::new((1, 1), robot.velocity), robot);

        let robot = robot.accelerate(grid.bounds());
        assert_eq!(Robot::new((2, 2), robot.velocity), robot);

        let robot = robot.accelerate(grid.bounds());
        assert_eq!(Robot::new((0, 0), robot.velocity), robot);
    }

    #[test]
    fn move_robot_negative() {
        let grid: Grid = Grid::new(3, 3);
        let robot = Robot::new((2, 2), (-1, -1));

        let robot = robot.accelerate(grid.bounds());
        assert_eq!(Robot::new((1, 1), robot.velocity), robot);

        let robot = robot.accelerate(grid.bounds());
        assert_eq!(Robot::new((0, 0), robot.velocity), robot);

        let robot = robot.accelerate(grid.bounds());
        assert_eq!(Robot::new((2, 2), robot.velocity), robot);
    }
}
