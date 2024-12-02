advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut num_safe = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|k| k.parse().unwrap())
            .collect();
        if check_ok(levels) {
            num_safe += 1;
        }
    }
    Some(num_safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut num_safe = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|k| k.parse().unwrap())
            .collect();

        for i in 0..levels.len() {
            let subset:Vec<i32> = levels.clone().into_iter().enumerate().filter(|&(j,_)| i != j).map(|(_,v)| v).collect();
            if check_ok(subset) {
                num_safe += 1;
                break;
            }
        }
    }
    Some(num_safe)
}

fn check_ok(input: Vec<i32>) -> bool {
    let diffs: Vec<i32> = input.windows(2).map(|w| w[0] - w[1]).collect();
    if !diffs.iter().all(|d| d.abs() >= 1 && d.abs() <= 3) {
        return false;
    }
    let signs_positive = diffs.iter().map(|d| d.signum()).all(|s| s == 1);
    let signs_negative = diffs.iter().map(|d| d.signum()).all(|s| s == -1);
    if !(signs_negative || signs_positive) {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
