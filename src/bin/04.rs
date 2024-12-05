use ndarray::Array;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let ninput = input.replace("\n", "");

    let numcols = input.find("\n").unwrap();
    let numrows = ninput.len() / numcols;

    let arr = Array::from_iter(ninput.as_bytes());
    let arr2d = arr.into_shape_with_order([numcols, numrows]).unwrap();

    let mut total = 0;
    for w in arr2d.windows((1, 4)) {
        if *w[[0, 0]] == b'X' && *w[[0, 1]] == b'M' && *w[[0, 2]] == b'A' && *w[[0, 3]] == b'S' {
            total += 1;
        }
        else if *w[[0, 0]] == b'S' && *w[[0, 1]] == b'A' && *w[[0, 2]] == b'M' && *w[[0, 3]] == b'X' {
            total += 1;
        }
    }
    for w in arr2d.windows((4, 1)) {
        if *w[[0, 0]] == b'X' && *w[[1, 0]] == b'M' && *w[[2, 0]] == b'A' && *w[[3, 0]] == b'S' {
            total += 1;
        }
        else if *w[[0, 0]] == b'S' && *w[[1, 0]] == b'A' && *w[[2, 0]] == b'M' && *w[[3, 0]] == b'X' {
            total += 1;
        }
    }
    for w in arr2d.windows((4, 4)) {
        if *w[[0, 0]] == b'X' && *w[[1, 1]] == b'M' && *w[[2, 2]] == b'A' && *w[[3, 3]] == b'S' {
            total += 1;
        }
        else if *w[[0, 0]] == b'S' && *w[[1, 1]] == b'A' && *w[[2, 2]] == b'M' && *w[[3, 3]] == b'X' {
            total += 1;
        }
        if *w[[0, 3]] == b'X' && *w[[1, 2]] == b'M' && *w[[2, 1]] == b'A' && *w[[3, 0]] == b'S' {
            total += 1;
        }
        else if *w[[0, 3]] == b'S' && *w[[1, 2]] == b'A' && *w[[2, 1]] == b'M' && *w[[3, 0]] == b'X' {
            total += 1;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ninput = input.replace("\n", "");

    let numcols = input.find("\n").unwrap();
    let numrows = ninput.len() / numcols;

    let arr = Array::from_iter(ninput.as_bytes());
    let arr2d = arr.into_shape_with_order([numcols, numrows]).unwrap();

    let mut total = 0;
    for w in arr2d.windows((3, 3)) {
        if *w[[0, 0]] == b'M' && *w[[0, 2]] == b'S' && *w[[1, 1]] == b'A' && *w[[2, 0]] == b'M' && *w[[2,2]] == b'S' {
            total += 1;
        }
        else if *w[[0, 0]] == b'S' && *w[[0, 2]] == b'S' && *w[[1, 1]] == b'A' && *w[[2, 0]] == b'M' && *w[[2,2]] == b'M' {
            total += 1;
        }
        else if *w[[0, 0]] == b'M' && *w[[0, 2]] == b'M' && *w[[1, 1]] == b'A' && *w[[2, 0]] == b'S' && *w[[2,2]] == b'S' {
            total += 1;
        }
        else if *w[[0, 0]] == b'S' && *w[[0, 2]] == b'M' && *w[[1, 1]] == b'A' && *w[[2, 0]] == b'S' && *w[[2,2]] == b'M' {
            total += 1;
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
