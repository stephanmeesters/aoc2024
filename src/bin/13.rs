use glam::IVec2;
use glam::I64Vec2;
use regex::Regex;
advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"(\d+).{4}(\d+)").unwrap();
    let input_lines:Vec<&str> = input.lines().collect();

    let mut total = 0;
    for lines in input_lines.chunks(4) {
        let v1:IVec2 = extract_vec(&regex, lines[0]);
        let v2:IVec2 = extract_vec(&regex, lines[1]);
        let t:IVec2 = extract_vec(&regex, lines[2]);

        let a = (t.x * v2.y - t.y * v2.x) / (v1.x * v2.y - v1.y * v2.x);
        let b = (-t.x * v1.y + t.y * v1.x) / (v1.x * v2.y - v1.y * v2.x);

        if v1 * a + v2 * b == t {
            let cost = 3 * a + b;
            total += cost;
        }
    }
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let regex = Regex::new(r"(\d+).{4}(\d+)").unwrap();
    let input_lines:Vec<&str> = input.lines().collect();

    let mut total = 0;
    for lines in input_lines.chunks(4) {
        let v1:I64Vec2 = extract_vec_64(&regex, lines[0]);
        let v2:I64Vec2 = extract_vec_64(&regex, lines[1]);
        let t:I64Vec2 = extract_vec_64(&regex, lines[2]) + I64Vec2::new(10000000000000, 10000000000000);

        let a = (t.x * v2.y - t.y * v2.x) / (v1.x * v2.y - v1.y * v2.x);
        let b = (-t.x * v1.y + t.y * v1.x) / (v1.x * v2.y - v1.y * v2.x);

        if v1 * a + v2 * b == t {
            let cost = 3 * a + b;
            total += cost;
        }
    }
    Some(total as u64)
}

fn extract_vec(regex:&Regex, input: &str) -> IVec2 {
    let (_, [l1, l2]) = regex.captures(input).unwrap().extract();
    IVec2::new(l1.parse().unwrap(), l2.parse().unwrap())
}

fn extract_vec_64(regex:&Regex, input: &str) -> I64Vec2 {
    let (_, [l1, l2]) = regex.captures(input).unwrap().extract();
    I64Vec2::new(l1.parse().unwrap(), l2.parse().unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
