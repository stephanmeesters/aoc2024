use regex::Regex;
use lazy_static::lazy_static;

advent_of_code::solution!(3);

lazy_static! {
    static ref RE:Regex = regex::Regex::new(r"(mul\((\d+),(\d+)\))").unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(process_part(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut doing = true;
    let mut left = 0;
    let mut total = 0;
    loop {
        let target = if doing { "don't()" } else { "do()" };
        if let Some(mut right) = input[left..].find(target) {
            right = left + right;
            if doing {
                total += process_part(&input[left..right]);
            }
            left = right;
            doing = !doing;
            continue;
        }
        else {
            if doing {
                total += process_part(&input[left..]);
            }
        }
        break;
    }
    Some(total)
}

fn process_part(sub_input: &str) -> u32 {
    let mut total = 0;
    for (_, [_, num1, num2]) in RE.captures_iter(sub_input).map(|c| c.extract()) {
        let n1:u32 = num1.parse().unwrap();
        let n2:u32 = num2.parse().unwrap();
        total += n1 * n2;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
