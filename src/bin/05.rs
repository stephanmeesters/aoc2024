use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
enum Priority {
    Before(u32),
    After(u32),
}

pub fn part_one(input: &str) -> Option<u32> {
    let split = input.find("\n\n").unwrap();
    let map = gen_map(input, split);

    let mut total = 0;
    for line in input[split + 2..].lines() {
        let pages: Vec<u32> = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
        let mut ok = true;
        for i in 0..pages.len() {
            let imap = &map[&pages[i]];
            let prange = &pages[i + 1..];
            ok &= !imap.iter().any(|m| match m {
                Priority::After(vv) => prange.contains(vv),
                _ => false,
            });
            if !ok {
                break;
            }
        }
        if ok {
            total += pages[pages.len() / 2];
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let split = input.find("\n\n").unwrap();
    let map = gen_map(input, split);

    let mut total = 0;
    for line in input[split + 2..].lines() {
        let pages: Vec<u32> = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
        let mut ok = true;
        for i in 0..pages.len() {
            let imap = &map[&pages[i]];
            let prange = &pages[i + 1..];
            ok &= !imap.iter().any(|m| match m {
                Priority::After(vv) => prange.contains(vv),
                _ => false,
            });
            if !ok {
                break;
            }
        }
        if !ok {
            let mut reordered: Vec<u32> = Vec::with_capacity(pages.len());
            reordered.push(pages[0]);
            for page in pages[1..].iter() {
                let imap = &map[page];
                let nn: usize = imap
                    .iter()
                    .map(|m| match m {
                        Priority::After(vv) => reordered.iter().position(|r| r == vv).unwrap_or(usize::max_value()),
                        _ => usize::max_value(),
                    })
                    .min()
                    .unwrap();
                if nn == usize::max_value() {
                    reordered.push(*page);
                } else {
                    reordered.insert(nn, *page)
                }
            }
            total += reordered[pages.len() / 2];
        }
    }
    Some(total)
}

fn gen_map(input: &str, split: usize) -> HashMap<u32, Vec<Priority>> {
    let mut map: HashMap<u32, Vec<Priority>> = HashMap::new();
    for line in input[..split].lines() {
        let (left, right) = line
            .split("|")
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        if map.contains_key(&left) {
            map.get_mut(&left).unwrap().push(Priority::Before(right));
        } else {
            map.insert(left, vec![Priority::Before(right)]);
        }
        if map.contains_key(&right) {
            map.get_mut(&right).unwrap().push(Priority::After(left));
        } else {
            map.insert(right, vec![Priority::After(left)]);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
