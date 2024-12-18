use advent_of_code::printd;
use regex::Regex;
use glam::I64Vec2;

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    position: I64Vec2,
    velocity: I64Vec2
}

impl Robot {
    fn move_robot(&mut self, iterations: i64, mrows:i64, mcols:i64) -> () {
        for _ in 0..iterations {
            self.position += self.velocity;
            self.position = self.position.rem_euclid(I64Vec2::new(mcols, mrows));
        }
    }

    fn in_quadrant(&self, (min_row, max_row, min_col, max_col):(i64,i64,i64,i64)) -> bool {
        self.position.x >= min_col && self.position.x <= max_col &&
           self.position.y >= min_row && self.position.y <= max_row
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let rows = 103;
    let cols = 101;
    part_one_impl(input, rows, cols)
}

pub fn part_one_testing(input: &str) -> Option<u64> {
    let rows = 7;
    let cols = 11;
    part_one_impl(input, rows, cols)
}

fn part_one_impl(input: &str, rows:i64, cols:i64) -> Option<u64> {
    let regex = Regex::new(r"(\d+),(\d+)...(-?\d+),(-?\d+)").unwrap();
    let mut robots:Vec<Robot> = input.lines().map(|line| {
        let (_, [a1, a2, a3, a4]) = regex.captures(line).unwrap().extract();
        Robot {
            position: I64Vec2::new(a1.parse().unwrap(), a2.parse().unwrap()),
            velocity: I64Vec2::new(a3.parse().unwrap(), a4.parse().unwrap()),
        }
    }).collect();

    robots.iter_mut().for_each(|robot| robot.move_robot(100, rows, cols));

    let ra = (rows-1)/2 - 1;
    let ca = (cols-1)/2 - 1;
    let rb = ra + 2;
    let cb = ca + 2;
    let rc = rows-1;
    let cc = cols-1;
    let quadrants = vec![(0,ra,0,ca), (rb,rc,0,ca), (0,ra,cb,cc), (rb,rc,cb,cc)];
    let total:i64 = quadrants.into_iter().map(|quadrant| robots.iter().filter(|robot| robot.in_quadrant(quadrant)).count() as i64).product();
    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = 103;
    let cols = 101;
    let regex = Regex::new(r"(\d+),(\d+)...(-?\d+),(-?\d+)").unwrap();
    let mut robots:Vec<Robot> = input.lines().map(|line| {
        let (_, [a1, a2, a3, a4]) = regex.captures(line).unwrap().extract();
        Robot {
            position: I64Vec2::new(a1.parse().unwrap(), a2.parse().unwrap()),
            velocity: I64Vec2::new(a3.parse().unwrap(), a4.parse().unwrap()),
        }
    }).collect();
    for ii in 1..1000 {
        robots.iter_mut().for_each(|robot| robot.move_robot(1, rows, cols));
        println!("iteration: {}", ii);
        printmap(&robots, rows, cols);
        let mut line = String::new();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
    }
    None
}

fn printmap(robots:&Vec<Robot>, rows:i64, cols:i64) {
    let ra = (rows-1)/2;
    let ca = (cols-1)/2;
    for i in 0..rows {
        for j in 0..cols {
            // if i == ra || j == ca {
            //     print!(" ");
            //     continue;
            // }
            let total:usize = robots.iter().filter(|robot| robot.in_quadrant((i,i,j,j))).count();
            if total == 0 {
                print!(" ");
            } else {
                print!("{}", total);
            }
        }
        print!("\n");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_testing(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
