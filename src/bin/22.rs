use advent_of_code::printd;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    let secrets:Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let sum = secrets.into_iter().map(|s| process1(s, 2000)).sum();
    Some(sum)
}

fn process1(mut secret: u64, iterations: u32) -> u64 {
    for _ in 0..iterations {
        let mut num = secret * 64;
        secret ^= num;
        secret ^= 16777216;

        num = (secret as f64 / 64.0).round() as u64;
        secret ^= num;
        secret ^= 16777216;

        num = secret * 2048;
        secret ^= num;
        secret ^= 16777216;
    }
    secret
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
