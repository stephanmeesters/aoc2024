use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let num_lines = input.lines().count();
    let mut left:Vec<u32> = Vec::with_capacity(num_lines);
    let mut right:Vec<u32> = Vec::with_capacity(num_lines);
    for line in input.lines() {
        let digits:Vec<u32> = line.split_whitespace().map(|w| w.parse().unwrap()).collect();
        left.push(digits[0]);
        right.push(digits[1]);
    }
    left.sort();
    right.sort();

    let mut total = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        total = total + l.abs_diff(*r);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left:Vec<u32> = Vec::with_capacity(input.lines().count());
    let mut right:HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let digits:Vec<u32> = line.split_whitespace().map(|w| w.parse().unwrap()).collect();
        left.push(digits[0]);
        right.entry(digits[1]).and_modify(|r| *r += 1).or_insert(1);
    }

    let mut total = 0;
    for l in left.iter() {
        if right.contains_key(l) {
            total += l * right[l];
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
