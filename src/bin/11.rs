use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use once_cell::sync::Lazy;
use dashmap::DashMap;

advent_of_code::solution!(11);

static MEMO: Lazy<DashMap<(u64, u8), u64>> = Lazy::new(|| DashMap::new());

pub fn part_one(input: &str) -> Option<u64> {
    let stones_list: Vec<u64> = input
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let total = stones_list
        .iter()
        .map(|&start| {
            dp(start, 0, 25)
        })
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones_list: Vec<u64> = input
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    let total = stones_list
        .par_iter()
        .map(|&start| {
            dp(start, 0, 75)
        })
        .sum();
    Some(total)
}

fn dp(stone_number: u64, iter: u8, iter_target: u8) -> u64 {
    if iter == iter_target {
        return 1;
    }
    if let Some(memo) = MEMO.get(&(stone_number, iter_target-iter)) {
        return *memo;
    }
    let str = stone_number.to_string();
    let total = match str.as_str() {
        "0" => dp(1, iter+1, iter_target),
        n if n.len() % 2 == 0 => {
            dp(str[..n.len() / 2].parse().unwrap(), iter+1, iter_target) +
            dp(str[n.len() / 2..].parse().unwrap(), iter+1, iter_target)
        },
        _ => dp(stone_number * 2024, iter+1, iter_target)
    };
    MEMO.insert((stone_number, iter_target - iter), total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
