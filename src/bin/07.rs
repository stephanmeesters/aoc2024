use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(7);

#[derive(Debug)]
enum Operator {
    Plus,
    Multiply,
    Concatenate
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let total:u64 = lines.par_iter().map(|line| {
        let (left, right) = line.split(":").collect_tuple().unwrap();
        let sum: u64 = left.parse().unwrap();
        let values: Vec<u64> = right
            .trim()
            .split(" ")
            .map(|v| v.parse().unwrap())
            .collect();
        match search(sum, &values, &[Operator::Multiply, Operator::Plus]) {
            true => sum,
            false => 0
        }
    }).sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let total:u64 = lines.par_iter().map(|line| {
        let (left, right) = line.split(":").collect_tuple().unwrap();
        let sum: u64 = left.parse().unwrap();
        let values: Vec<u64> = right
            .trim()
            .split(" ")
            .map(|v| v.parse().unwrap())
            .collect();
        match search(sum, &values, &[Operator::Multiply, Operator::Plus, Operator::Concatenate]) {
            true => sum,
            false => 0
        }
    }).sum();
    Some(total)
}

fn search(sum: u64, values: &[u64], operators: &[Operator]) -> bool {
    std::iter::repeat(operators)
        .take(values.len()-1)
        .multi_cartesian_product()
        .map(|op| exec_operators(values, &op)).any(|v| v == sum)
}

fn exec_operators(values: &[u64], operators: &[&Operator]) -> u64 {
    let mut total = values[0];
    for i in 0..operators.len() {
        match operators[i] {
            Operator::Plus => total += values[i+1],
            Operator::Multiply => total *= values[i+1],
            Operator::Concatenate => total = total * digit_count(values[i+1]) + values[i+1]
        }
    }
    total
}

fn digit_count(n: u64) -> u64 {
    if n >= 1_000_000_000 { 10_000_000_000 }
    else if n >= 100_000_000 { 1_000_000_000 }
    else if n >= 10_000_000 { 100_100_100 }
    else if n >= 1_000_000 { 10_000_000 }
    else if n >= 100_000 { 1_000_000 }
    else if n >= 10_000 { 100_000 }
    else if n >= 1_000 { 10_000 }
    else if n >= 100 { 1_000 }
    else if n >= 10 { 100 }
    else { 10 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
