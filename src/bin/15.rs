use itertools::Itertools;
use glam::UVec2;
advent_of_code::solution!(15);

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Obstacle {
    Wall,
    Box,
    None,
    Robot,
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<&str> = input.lines().collect();
    let divide = lines.iter().position(|l| l.len() == 0).unwrap();
    let width = lines[0].len();
    let height = divide;
    let mut map: Vec<Vec<Obstacle>> = vec![vec![Obstacle::None; width]; height];

    let mut robot_pos: UVec2 = UVec2::new(0, 0);

    for (row, line) in lines[..divide].iter().enumerate() {
        for (col, cc) in line.bytes().enumerate() {
            match cc {
                b'O' => map[row][col] = Obstacle::Box,
                b'#' => map[row][col] = Obstacle::Wall,
                b'@' => robot_pos = UVec2::new(row as u32, col as u32),
                _ => (),
            }
        }
    }
    let instructions: Vec<Move> = lines[divide..]
        .iter()
        .map(|l| l.trim().chars())
        .flatten()
        .map(|b| match b {
            '<' => Move::Left,
            '>' => Move::Right,
            '^' => Move::Up,
            'v' => Move::Down,
            _ => panic!(),
        })
        .collect();

    for (instruction_index, instruction) in instructions.into_iter().enumerate() {
        match instruction {
            Move::Left => match map[robot_pos.x as usize][(robot_pos.y - 1) as usize] {
                Obstacle::None => robot_pos.y -= 1,
                Obstacle::Box => {
                    if let Some(index) = (0..(robot_pos.y as usize - 1))
                        .rposition(|y| map[robot_pos.x as usize][y as usize] == Obstacle::None)
                    {
                        let wall = (0..(robot_pos.y as usize - 1))
                            .rposition(|y| map[robot_pos.x as usize][y as usize] == Obstacle::Wall);
                        if let Some(www) = wall {
                            if www > index {
                                continue;
                            }
                        }
                        (
                            map[robot_pos.x as usize][(robot_pos.y - 1) as usize],
                            map[robot_pos.x as usize][index],
                        ) = (
                            map[robot_pos.x as usize][index],
                            map[robot_pos.x as usize][(robot_pos.y - 1) as usize],
                        );
                        robot_pos.y -= 1;
                    }
                }
                _ => (),
            },
            Move::Right => match map[robot_pos.x as usize][(robot_pos.y + 1) as usize] {
                Obstacle::None => robot_pos.y += 1,
                Obstacle::Box => {
                    if let Some(index) = (((robot_pos.y + 2) as usize)..width)
                        .position(|y| map[robot_pos.x as usize][y as usize] == Obstacle::None)
                    {
                        let wall = (((robot_pos.y + 2) as usize)..width)
                            .position(|y| map[robot_pos.x as usize][y as usize] == Obstacle::Wall);
                        if let Some(www) = wall {
                            if www < index {
                                continue;
                            }
                        }
                        let lindex = index + robot_pos.y as usize + 2;
                        (
                            map[robot_pos.x as usize][(robot_pos.y + 1) as usize],
                            map[robot_pos.x as usize][lindex as usize],
                        ) = (
                            map[robot_pos.x as usize][lindex],
                            map[robot_pos.x as usize][(robot_pos.y + 1) as usize],
                        );
                        robot_pos.y += 1;
                    }
                }
                _ => (),
            },
            Move::Up => match map[(robot_pos.x - 1) as usize][robot_pos.y as usize] {
                Obstacle::None => robot_pos.x -= 1,
                Obstacle::Box => {
                    if let Some(index) = (0..(robot_pos.x as usize - 1))
                        .rposition(|x| map[x][robot_pos.y as usize] == Obstacle::None)
                    {
                        let wall = (0..(robot_pos.x as usize - 1))
                            .rposition(|x| map[x][robot_pos.y as usize] == Obstacle::Wall);
                        if let Some(www) = wall {
                            if www > index {
                                continue;
                            }
                        }
                        (
                            map[(robot_pos.x - 1) as usize][robot_pos.y as usize],
                            map[index][robot_pos.y as usize],
                        ) = (
                            map[index][robot_pos.y as usize],
                            map[(robot_pos.x - 1) as usize][robot_pos.y as usize],
                        );
                        robot_pos.x -= 1;
                    }
                }
                _ => (),
            },
            Move::Down => match map[(robot_pos.x + 1) as usize][robot_pos.y as usize] {
                Obstacle::None => robot_pos.x += 1,
                Obstacle::Box => {
                    if let Some(index) = ((robot_pos.x as usize + 2)..height)
                        .position(|x| map[x][robot_pos.y as usize] == Obstacle::None)
                    {
                        let wall = (((robot_pos.x + 2) as usize)..height)
                            .position(|x| map[x][robot_pos.y as usize] == Obstacle::Wall);
                        if let Some(www) = wall {
                            if www < index {
                                continue;
                            }
                        }
                        let lindex = index + robot_pos.x as usize + 2;
                        (
                            map[(robot_pos.x + 1) as usize][robot_pos.y as usize],
                            map[lindex][robot_pos.y as usize],
                        ) = (
                            map[lindex][robot_pos.y as usize],
                            map[(robot_pos.x + 1) as usize][robot_pos.y as usize],
                        );
                        robot_pos.x += 1;
                    }
                }
                _ => (),
            },
        }
    }
    let score:usize = (0..width).cartesian_product(0..height).filter(|(x,y)| map[*x][*y] == Obstacle::Box).map(|(x,y)| 100 * x + y).sum();
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn print_map(map: &Vec<Vec<Obstacle>>, robot_pos: &UVec2) {
    let mut cmap = map.clone();
    cmap[robot_pos.x as usize][robot_pos.y as usize] = Obstacle::Robot;
    for line in cmap {
        let str: String = line
            .iter()
            .map(|o| match o {
                Obstacle::None => ".",
                Obstacle::Wall => "#",
                Obstacle::Box => "O",
                Obstacle::Robot => "@",
            })
            .collect();
        println!("{}", str);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
