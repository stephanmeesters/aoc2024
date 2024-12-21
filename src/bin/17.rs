use rayon::prelude::*;
use advent_of_code::*;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut input_itr = input.lines();
    let mut A: u32 = input_itr.parse_last_word();
    let mut B: u32 = input_itr.parse_last_word();
    let mut C: u32 = input_itr.parse_last_word();
    let program: Vec<u32> = input_itr
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
    let mut output:Vec<u32> = Vec::new();
    while ip < program.len() {
        let opcode = program[ip];
        let literal = program[ip+1];
        let combo = match literal {
            0..=3 => literal,
            4 => A,
            5 => B,
            6 => C,
            _ => panic!()
        };
        ip += 2;
        match opcode {
            0 => {
                A = A / 2u32.pow(combo as u32);
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
            },
            4 => {
                B = B ^ C;
            },
            5 => {
                output.push(combo % 8);
            },
            6 => {
                B = A / 2u32.pow(combo as u32);
            },
            7 => {
                C = A / 2u32.pow(combo as u32);
            },
            _ => panic!()
        }
    }
    Some(itertools::join(output, ","))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input_itr = input.lines();
    let _: u32 = input_itr.parse_last_word();
    let mut B: u32 = input_itr.parse_last_word();
    let mut C: u32 = input_itr.parse_last_word();
    let program: Vec<u32> = input_itr
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    let range = 0..u32::max_value();
    if let Some(solution) = range.into_par_iter().find_any(|&x| part_two_loop(x, B, C, &program) != 0) {
        return Some(solution);
    }

    None
}

fn part_two_loop(ai:u32, bi:u32, ci:u32, program: &Vec<u32>) -> u32 {
    let mut A = ai;
    let mut B = bi;
    let mut C = ci;

    let mut ip = 0;
    let mut output:Vec<u32> = Vec::new();
    let mut aap = 0;
    while ip < program.len() {
        aap += 1;
        if output.len() > program.len() {
            return 0;
        }
        let opcode = program[ip];
        let literal = program[ip+1];
        let combo = match literal {
            0..=3 => literal,
            4 => A,
            5 => B,
            6 => C,
            _ => panic!()
        };
        ip += 2;
        match opcode {
            0 => {
                A = A / 2u32.pow(combo as u32);
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
            },
            4 => {
                B = B ^ C;
            },
            5 => {
                output.push(combo % 8);
            },
            6 => {
                B = A / 2u32.pow(combo as u32);
            },
            7 => {
                C = A / 2u32.pow(combo as u32);
            },
            _ => panic!()
        }
    }

    if program.len() == output.len() && program.iter().zip(output.iter()).all(|(a,b)| *a == *b) {
        return ai;
    }
    return 0;
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(117440));
    }
}
