use dashmap::DashMap;
use once_cell::sync::Lazy;

advent_of_code::solution!(19);

static MEMO: Lazy<DashMap<String, u64>> = Lazy::new(|| DashMap::new());

pub fn part_one(input: &str) -> Option<u64> {
    let split: Vec<&str> = input.split("\n\n").collect();
    let patterns: Vec<&str> = split[0].split(",").map(|r| r.trim()).collect();
    let towels: Vec<&str> = split[1].trim().split("\n").collect();
    let count = towels.into_iter().map(|t| if dp(t, &patterns) > 0 { 1 } else { 0 }).sum();
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let split: Vec<&str> = input.split("\n\n").collect();
    let patterns: Vec<&str> = split[0].split(",").map(|r| r.trim()).collect();
    let towels: Vec<&str> = split[1].trim().split("\n").collect();
    let count = towels.into_iter().map(|t| dp(t, &patterns)).sum();
    Some(count)
}

fn dp(input: &str, patterns: &Vec<&str>) -> u64 {
    if input.len() == 0 {
        return 1;
    }
    if let Some(memo) = MEMO.get(&input.to_owned()) {
        return *memo;
    }
    let mut count = 0;
    let mut found_patterns: Vec<&str> = Vec::new();
    for pattern in patterns.iter() {
        if input.starts_with(pattern) { // &&
            count += dp(&input[pattern.len()..], patterns);
            found_patterns.push(pattern);
        }
    }
    MEMO.insert(input.to_owned(), count);
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
