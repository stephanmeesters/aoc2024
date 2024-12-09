advent_of_code::solution!(9);

#[derive(Debug)]
enum FileBlock {
    Block(u64),
    Free,
}

#[derive(Debug)]
enum FileBlockRow {
    Block(u64, u64),
    Free(u64),
}

impl FileBlockRow {
    fn sum(&self, index: u64) -> u64 {
        match self {
            FileBlockRow::Block(block_index, block_size) => {
                let mut total = 0;
                for ii in 0..*block_size {
                    let rel_index = index + ii;
                    total += rel_index * block_index;
                }
                total
            }
            FileBlockRow::Free(_) => 0,
        }
    }

    fn length(&self) -> u64 {
        match self {
            FileBlockRow::Block(_, l) => *l,
            FileBlockRow::Free(l) => *l
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let ints: Vec<u64> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    let size: u64 = ints.iter().sum();
    let mut block_list: Vec<FileBlock> = Vec::with_capacity(size as usize);

    let mut counter = 0;
    let mut free = false;
    for int in ints.into_iter() {
        // O(M)
        if free {
            for _ in 0..int {
                block_list.push(FileBlock::Free);
            }
        } else {
            for _ in 0..int {
                block_list.push(FileBlock::Block(counter));
            }
            counter += 1;
        }
        free = !free;
    }

    for i in 0..block_list.len() {
        // O(N^2)
        match &block_list[i] {
            FileBlock::Free => {
                if let Some(ind) = &block_list[i..].iter().rposition(|b| match b {
                    FileBlock::Block(_) => true,
                    _ => false,
                }) {
                    let _ = &block_list.swap(i, i + *ind);
                }
            }
            _ => continue,
        }
    }

    let checksum = block_list // O(N)
        .into_iter()
        .enumerate()
        .map(|(ind, b)| match b {
            FileBlock::Free => 0,
            FileBlock::Block(i) => i * (ind as u64),
        })
        .sum();

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ints: Vec<u64> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    let mut block_list: Vec<FileBlockRow> = Vec::with_capacity(ints.len());

    let mut counter = 0;
    let mut free = false;
    for int in ints.into_iter() {
        // O(M)
        if free {
            block_list.push(FileBlockRow::Free(int));
        } else {
            block_list.push(FileBlockRow::Block(counter, int));
            counter += 1;
        }
        free = !free;
    }

    let mut i = 0;
    while i < block_list.len() - 1 {
        // O(N^2)
        match &block_list[i] {
            FileBlockRow::Free(free_size) => {
                if let Some(index) = &block_list[i..].iter().rposition(|b| match b {
                    FileBlockRow::Block(_, block_size) => block_size <= free_size,
                    _ => false,
                }) {
                    if let FileBlockRow::Block(block_index, block_size) = &block_list[i + *index] {
                        if block_size < free_size {
                            let bb = *block_index;
                            let cc = *block_size;
                            let fill = FileBlockRow::Free(free_size - block_size);
                            block_list[i + *index] = FileBlockRow::Free(*block_size);
                            block_list[i] = FileBlockRow::Block(bb, cc);
                            let _ = &block_list.insert(i + 1, fill);
                        } else {
                            let _ = &block_list.swap(i, i + *index);
                        }
                    }
                }
            }
            _ => (),
        }
        i += 1;
    }

    let mut acc = 0;
    let mut checksum = 0;
    for b in block_list.into_iter() {
        checksum += b.sum(acc);
        acc += b.length();
    }

    Some(checksum)
}

// fn printblocks(list: &Vec<FileBlockRow>) {
//     for bb in list.iter() {
//         match bb {
//             FileBlockRow::Block(i, size) => {
//                 print!("{}", std::iter::repeat(i).take(*size as usize).map(|v| v.to_string()).collect::<String>());
//             }
//             FileBlockRow::Free(size) => {
//                 print!("{}", std::iter::repeat('.').take(*size as usize).map(|v| v.to_string()).collect::<String>());
//             }
//         }
//     }
//     println!("");
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
