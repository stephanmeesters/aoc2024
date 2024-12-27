use advent_of_code::*;
use rayon::prelude::*;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut input_itr = input.lines();
    let mut A: u64 = input_itr.parse_last_word();
    let mut B: u64 = input_itr.parse_last_word();
    let mut C: u64 = input_itr.parse_last_word();
    let program: Vec<u64> = input_itr
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    let mut ip = 0;
    let mut output: Vec<u64> = Vec::new();
    while ip < program.len() {
        let opcode = program[ip];
        let literal = program[ip + 1];
        let combo = match literal {
            0..=3 => literal,
            4 => A,
            5 => B,
            6 => C,
            _ => panic!(),
        };
        ip += 2;
        match opcode {
            0 => {
                A = A / 2u64.pow(combo as u32);
            }
            1 => {
                B = B ^ literal;
            }
            2 => {
                B = combo % 8;
            }
            3 => {
                if A > 0 {
                    ip = literal as usize;
                }
            }
            4 => {
                B = B ^ C;
            }
            5 => {
                output.push(combo % 8);
            }
            6 => {
                B = A / 2u64.pow(combo as u32);
            }
            7 => {
                C = A / 2u64.pow(combo as u32);
            }
            _ => panic!(),
        }
    }
    Some(itertools::join(output, ","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input_itr = input.lines();
    let _: u64 = input_itr.parse_last_word();
    let B: u64 = input_itr.parse_last_word();
    let C: u64 = input_itr.parse_last_word();
    let program: Vec<u64> = input_itr
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    // part_two_rec(

    None
}

fn part_two_rec(program: &Vec<u64>, opcode: u64, combo: u64) {
    match opcode {
        0 => {
            A = A / 2u64.pow(combo as u32);
        }
        1 => {
            B = B ^ literal;
        }
        2 => {
            B = combo % 8;
        }
        3 => {
            if A > 0 {
                ip = literal as usize;
            }
        }
        4 => {
            B = B ^ C;
        }
        5 => {
            ()
        }
        6 => {
            B = A / 2u64.pow(combo as u32);
        }
        7 => {
            C = A / 2u64.pow(combo as u32);
        }
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
