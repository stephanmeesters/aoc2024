use advent_of_code::printd;
use glam::IVec2;
use itertools::{iproduct, Itertools};
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|i| i.as_bytes().to_owned()).collect();
    let numrows = grid.len();
    let numcols = grid[0].len();
    let mut towers: HashMap<u8, Vec<IVec2>> = HashMap::new();

    for (i, j) in iproduct!(0..numrows, 0..numcols) {
        let pt = grid[i][j];
        if pt != b'.' {
            let idx = IVec2::new(i as i32, j as i32);
            if let Some(map) = towers.get_mut(&pt) {
                map.push(idx);
            } else {
                let _ = &towers.insert(pt, vec![idx]);
            }
        }
    }

    for locs in towers.into_values() {
        for pair in locs.iter().combinations(2) {
            let diff = pair[1] - pair[0];
            let na = pair[0] - diff;
            let nb = pair[1] + diff;
            if na.x >= 0 && na.x < numcols as i32 && na.y >= 0 && na.y < numrows as i32 {
                grid[na.x as usize][na.y as usize] = b'#';
            }
            if nb.x >= 0 && nb.x < numcols as i32 && nb.y >= 0 && nb.y < numrows as i32 {
                grid[nb.x as usize][nb.y as usize] = b'#';
            }
        }
    }

    let total = grid.into_iter().flatten().filter(|&g| g == b'#').count();

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|i| i.as_bytes().to_owned()).collect();
    let numrows = grid.len();
    let numcols = grid[0].len();
    let mut towers: HashMap<u8, Vec<IVec2>> = HashMap::new();

    for (i, j) in iproduct!(0..numrows, 0..numcols) {
        let pt = grid[i][j];
        if pt != b'.' {
            let idx = IVec2::new(i as i32, j as i32);
            if let Some(map) = towers.get_mut(&pt) {
                map.push(idx);
            } else {
                let _ = &towers.insert(pt, vec![idx]);
            }
        }
    }

    for locs in towers.into_values() {
        for pair in locs.iter().combinations(2) {
            let diff = pair[1] - pair[0];
            for ii in 1..100 {
                let na = pair[0] - ii * diff;
                if na.x < 0 || na.x >= numcols as i32 || na.y < 0 || na.y >= numrows as i32 {
                    break;
                }
                grid[na.x as usize][na.y as usize] = b'#';
            }
            for ii in 1..100 {
                let nb = pair[1] + ii * diff;
                if nb.x < 0 || nb.x >= numcols as i32 || nb.y < 0 || nb.y >= numrows as i32 {
                    break;
                }
                grid[nb.x as usize][nb.y as usize] = b'#';
            }
        }
    }

    let total = grid.into_iter().flatten().filter(|&g| g != b'.').count();

    Some(total as u32)
}

fn print_grid(grid: &Vec<Vec<u8>>) {
    for line in grid.iter() {
        let pp = String::from_utf8(line.to_vec()).unwrap();
        println!("{}", pp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
