use glam::IVec2;
use ndarray::s;
use ndarray::Array;
use ndarray::Array2;
use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

pub fn part_one(input: &str) -> Option<u32> {
    let ninput = input.replace("\n", "");

    let mut numcols = input.find("\n").unwrap();
    let mut numrows = ninput.len() / numcols;

    let arr = Array::from_iter(ninput.bytes());
    let arr2d = arr.into_shape_with_order([numcols, numrows]).unwrap();
    let arr2dpadded = apply_padding(&arr2d, 1, b'.');
    numcols += 2;
    numrows += 2;

    let start_point = arr2dpadded
        .iter()
        .position(|&p| p == b'^' || p == b'<' || p == b'>' || p == b'v')
        .unwrap();
    let mut cur_row: i32 = (start_point / numcols) as i32;
    let mut cur_col: i32 = (start_point % numcols) as i32;

    let start_char = arr2dpadded[[cur_row as usize, cur_col as usize]];
    let mut direction = match start_char {
        b'^' => Direction::Up,
        b'>' => Direction::Right,
        b'<' => Direction::Left,
        b'v' => Direction::Down,
        _ => panic!("CANOT FIND DIR"),
    };

    let mut visited = HashSet::new();

    loop {
        let center_window =
            arr2dpadded.slice(s![cur_row - 1..=cur_row + 1, cur_col - 1..=cur_col + 1]);
        let _ = &visited.insert(IVec2::new(cur_row, cur_col));

        direction = match direction {
            Direction::Up => {
                if center_window[[0, 1]] == b'#' {
                    Direction::Right
                } else {
                    Direction::Up
                }
            }
            Direction::Left => {
                if center_window[[1, 0]] == b'#' {
                    Direction::Up
                } else {
                    Direction::Left
                }
            }
            Direction::Right => {
                if center_window[[1, 2]] == b'#' {
                    Direction::Down
                } else {
                    Direction::Right
                }
            }
            Direction::Down => {
                if center_window[[2, 1]] == b'#' {
                    Direction::Left
                } else {
                    Direction::Down
                }
            }
        };

        match direction {
            Direction::Up => cur_row -= 1,
            Direction::Left => cur_col -= 1,
            Direction::Right => cur_col += 1,
            Direction::Down => cur_row += 1,
        }

        if cur_row < 1 || cur_row > (numrows - 2) as i32 {
            break;
        }
        if cur_col < 1 || cur_col > (numcols - 2) as i32 {
            break;
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ninput = input.replace("\n", "");

    let mut numcols = input.find("\n").unwrap();
    let mut numrows = ninput.len() / numcols;

    let arr = Array::from_iter(ninput.bytes());
    let arr2d = arr.into_shape_with_order([numcols, numrows]).unwrap();
    let arr2dpadded = apply_padding(&arr2d, 2, b'.');
    numcols += 4;
    numrows += 4;

    let start_point = arr2dpadded
        .iter()
        .position(|&p| p == b'^' || p == b'<' || p == b'>' || p == b'v')
        .unwrap();
    let mut cur_row: i32 = (start_point / numcols) as i32;
    let mut cur_col: i32 = (start_point % numcols) as i32;

    let start_char = arr2dpadded[[cur_row as usize, cur_col as usize]];
    let mut direction = match start_char {
        b'^' => Direction::Up,
        b'>' => Direction::Right,
        b'<' => Direction::Left,
        b'v' => Direction::Down,
        _ => panic!("CANOT FIND DIR"),
    };
    let start_direction = direction.clone();

    let mut pos_obstacles = HashSet::new();
    let mut confirmed_obstacles = HashSet::new();

    for i in 2..numcols - 2 {
        for j in 2..numrows - 2 {
            if arr2dpadded[[i, j]] != b'#' {
                pos_obstacles.insert(IVec2::new(i as i32, j as i32));
            }
        }
    }
    pos_obstacles.remove(&IVec2::new(cur_row, cur_col));

    for posob in pos_obstacles.iter() {
        cur_row = (start_point / numcols) as i32;
        cur_col = (start_point % numcols) as i32;
        direction = start_direction;
        let mut arr2dpaddedloop = arr2dpadded.to_owned();
        arr2dpaddedloop[[posob.x as usize, posob.y as usize]] = b'#';

        let mut looplen = 0;
        loop {
            looplen += 1;
            if looplen == 10000 {
                confirmed_obstacles.insert(posob);
                break;
            }

            let center_window =
                arr2dpaddedloop.slice(s![cur_row - 1..=cur_row + 1, cur_col - 1..=cur_col + 1]);

            loop {
                let prevdir = direction.clone();
                direction = match direction {
                    Direction::Up => {
                        if center_window[[0, 1]] == b'#' {
                            Direction::Right
                        } else {
                            Direction::Up
                        }
                    }
                    Direction::Left => {
                        if center_window[[1, 0]] == b'#' {
                            Direction::Up
                        } else {
                            Direction::Left
                        }
                    }
                    Direction::Right => {
                        if center_window[[1, 2]] == b'#' {
                            Direction::Down
                        } else {
                            Direction::Right
                        }
                    }
                    Direction::Down => {
                        if center_window[[2, 1]] == b'#' {
                            Direction::Left
                        } else {
                            Direction::Down
                        }
                    }
                };
                if direction == prevdir {
                    break;
                }
            }

            match direction {
                Direction::Up => cur_row -= 1,
                Direction::Left => cur_col -= 1,
                Direction::Right => cur_col += 1,
                Direction::Down => cur_row += 1,
            }

            if cur_row < 2 || cur_row > (numrows - 3) as i32 {
                break;
            }
            if cur_col < 2 || cur_col > (numcols - 3) as i32 {
                break;
            }
        }
    }

    Some(confirmed_obstacles.len() as u32)
}

// fn print_matrix(mat: &Array2<u8>) {
//     for row in mat.rows() {
//         let line: Vec<char> = row.iter().map(|&e| e as char).collect();
//         let ll: String = line.iter().map(|&c| c.to_string() + " ").collect();
//         println!("{}", ll);
//     }
// }
//
// fn print_matrix_slice(mat: ArrayBase<ViewRepr<&u8>, Dim<[usize; 2]>>) {
//     for row in mat.rows() {
//         let line: Vec<char> = row.iter().map(|&e| e as char).collect();
//         let ll: String = line.iter().map(|&c| c.to_string() + " ").collect();
//         println!("{}", ll);
//     }
// }

fn apply_padding<T: Clone>(array: &Array2<T>, padding: usize, pad_value: T) -> Array2<T> {
    let (rows, cols) = array.dim();
    let padded_rows = rows + 2 * padding;
    let padded_cols = cols + 2 * padding;

    let mut padded_array = Array2::from_elem((padded_rows, padded_cols), pad_value);

    // Place the original array in the center
    padded_array
        .slice_mut(s![padding..(padding + rows), padding..(padding + cols)])
        .assign(array);

    padded_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
