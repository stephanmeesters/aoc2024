use itertools::Itertools;
advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<(u8, u32, u32)>> = input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| (b, 0, 0))
                .collect::<Vec<(u8, u32, u32)>>()
        })
        .collect();
    let types: Vec<u8> = vec![
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
        b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
    ];
    let width = map[0].len();
    let height = map.len();

    let mut counter = 0;

    types.into_iter().for_each(|t| {
        (0..width).cartesian_product(0..height).for_each(|(x, y)| {
            if map[x][y].0 == t {
                let mut perimeter = 0;
                let mut class = 0;
                if x == 0 || map[x - 1][y].0 != t {
                    perimeter += 1;
                }
                if y == 0 || map[x][y - 1].0 != t {
                    perimeter += 1;
                }
                if x == width - 1 || map[x + 1][y].0 != t {
                    perimeter += 1;
                }
                if y == height - 1 || map[x][y + 1].0 != t {
                    perimeter += 1;
                }

                if x > 0 && map[x - 1][y].0 == t {
                    class = map[x - 1][y].1;
                }
                if y > 0 && map[x][y - 1].0 == t {
                    class = map[x][y - 1].1;
                }
                if class == 0 {
                    counter += 1;
                    class = counter;
                }
                map[x][y].1 = class;
                map[x][y].2 = perimeter;
            }
        });

        for _ in 0..=10 {
            (0..width).cartesian_product(0..height).for_each(|(x, y)| {
                if map[x][y].0 == t {
                    assert!(map[x][y].1 != 0);
                    if x > 0 && map[x - 1][y].0 == t && map[x - 1][y].1 != map[x][y].1 {
                        if map[x][y].1 > map[x - 1][y].1 {
                            map[x][y].1 = map[x - 1][y].1;
                        } else {
                            map[x - 1][y].1 = map[x][y].1;
                        }
                    }
                    if x < width - 1 && map[x + 1][y].0 == t && map[x + 1][y].1 != map[x][y].1 {
                        if map[x][y].1 > map[x + 1][y].1 {
                            map[x][y].1 = map[x + 1][y].1;
                        } else {
                            map[x + 1][y].1 = map[x][y].1;
                        }
                    }
                    if y > 0 && map[x][y - 1].0 == t && map[x][y - 1].1 != map[x][y].1 {
                        if map[x][y].1 > map[x][y - 1].1 {
                            map[x][y].1 = map[x][y - 1].1;
                        } else {
                            map[x][y - 1].1 = map[x][y].1;
                        }
                    }
                    if y < height - 1 && map[x][y + 1].0 == t && map[x][y + 1].1 != map[x][y].1 {
                        if map[x][y].1 > map[x][y + 1].1 {
                            map[x][y].1 = map[x][y + 1].1;
                        } else {
                            map[x][y + 1].1 = map[x][y].1;
                        }
                    }
                }
            });
        }
    });

    let mut score = 0;
    for count in 1..=counter {
        let ttt: Vec<u8> = map
            .iter()
            .flatten()
            .filter(|(_, c, _)| *c == count)
            .map(|(t, _, _)| *t)
            .take(1)
            .collect();
        if ttt.len() == 0 {
            continue;
        }
        let ff: Vec<u32> = map
            .iter()
            .flatten()
            .filter(|(_, c, _)| *c == count)
            .map(|(_, _, p)| *p)
            .collect();
        let area = ff.len() as u32;
        let perimeter: u32 = ff.into_iter().sum();
        let subscore = area * perimeter;
        println!(
            "{}: {} * {} = {}",
            String::from_utf8(ttt).unwrap(),
            area,
            perimeter,
            subscore
        );
        score += subscore;
    }

    println!("----------");
    printmap(&map);
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn np((a, b): (u32, u32)) -> (u32, u32) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

fn printmap(map: &Vec<Vec<(u8, u32, u32)>>) {
    for line in map {
        let str: String = line
            .iter()
            .map(|(a, b, _)| format!("({},{})\t", String::from_utf8([*a].to_vec()).unwrap(), b))
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
