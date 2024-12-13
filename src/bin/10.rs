use glam::UVec2;
use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<(u8, HashSet<UVec2>)>> = input
        .lines()
        .enumerate()
        .map(|(idr, line)| {
            line.bytes()
                .enumerate()
                .map(|(idc, b)| {
                    let v = (b - b'0') as u8;
                    if v == 9 {
                        (v, HashSet::from([UVec2::new(idr as u32, idc as u32)]))
                    } else {
                        (v, HashSet::new())
                    }
                })
                .collect::<Vec<(u8, HashSet<UVec2>)>>()
        })
        .collect();
    let width = map[0].len();
    let height = map.len();

    for scan in (0..=8).rev() {
        (0..width).cartesian_product(0..height).for_each(|(x, y)| {
            if map[x][y].0 == scan {
                let mut newset = HashSet::new();
                if x > 0 && map[x - 1][y].0 == scan + 1 {
                    newset.extend(map[x - 1][y].1.iter());
                }
                if x < width - 1 && map[x + 1][y].0 == scan + 1 {
                    newset.extend(map[x + 1][y].1.iter());
                }
                if y > 0 && map[x][y - 1].0 == scan + 1 {
                    newset.extend(map[x][y - 1].1.iter());
                }
                if y < height - 1 && map[x][y + 1].0 == scan + 1 {
                    newset.extend(map[x][y + 1].1.iter());
                }
                map[x][y].1 = newset;
            }
        });
    }

    let sum: u32 = map.into_iter().flatten().filter(|m| m.0 == 0).map(|m| m.1.len() as u32).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<(u8, u32)>> = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| {
                    let v = (b - b'0') as u8;
                    if v == 9 {
                        (v, 1)
                    } else {
                        (v, 0)
                    }
                })
                .collect::<Vec<(u8, u32)>>()
        })
        .collect();
    let width = map[0].len();
    let height = map.len();

    for scan in (0..=8).rev() {
        (0..width).cartesian_product(0..height).for_each(|(x, y)| {
            if map[x][y].0 == scan {
                let mut total = 0;
                if x > 0 && map[x - 1][y].0 == scan + 1 {
                    total += map[x - 1][y].1;
                }
                if x < width - 1 && map[x + 1][y].0 == scan + 1 {
                    total += map[x + 1][y].1;
                }
                if y > 0 && map[x][y - 1].0 == scan + 1 {
                    total += map[x][y - 1].1;
                }
                if y < height - 1 && map[x][y + 1].0 == scan + 1 {
                    total += map[x][y + 1].1;
                }
                map[x][y].1 = total;
            }
        });
    }

    let sum: u32 = map.into_iter().flatten().filter(|m| m.0 == 0).map(|m| m.1).sum();
    Some(sum)
}

fn printmap(map: &Vec<Vec<(u8, HashSet<UVec2>)>>) {
    for line in map {
        let str: String = line
            .iter()
            .map(|(a, b)| format!("({},{})", a, b.len()))
            .collect();
        println!("{}", str);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
