use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::Mutex;
use dashmap::DashMap;

advent_of_code::solution!(7);

static MEMO: Lazy<DashMap<u64, HashSet<u64>>> = Lazy::new(|| DashMap::new());

fn simple_hash(data: &[u64]) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let total:u64 = lines.par_iter().map(|line| {
        let (left, right) = line.split(":").collect_tuple().unwrap();
        let sum: u64 = left.parse().unwrap();
        let values: Vec<u64> = right
            .trim()
            .split(" ")
            .map(|v| v.parse().unwrap())
            .collect();
        println!("values: {:?}", values);
        let aap = part1_recursive(&values).unwrap();
        if aap.iter().any(|&a| a == sum) {
            return sum;
        }
        0 as u64
        // println!("{:?}", aap);
        // break;
    }).sum();
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn part1_recursive(values: &[u64]) -> Option<HashSet<u64>> {
    // println!("{:?} -- ", values);
    if values.len() == 1 {
        let mut mm = HashSet::new();
        mm.insert(values[0]);
        return Some(mm);
    }

    let hash = simple_hash(values);
    if MEMO.contains_key(&hash) {
        let aap = MEMO.get(&hash).unwrap();
        return Some(aap.clone());
    }

    let mut collect = HashSet::new();
    for i in 0..values.len() {
        for j in 0..values.len() {
            if i == j {
                continue;
            }

            let subarray_p1: Vec<u64> = std::iter::once(values[i] * values[j])
                .chain(
                    (0..values.len())
                        .filter(|&x| x != i && x != j)
                        .map(|x| values[x]),
                )
                .collect();

            let subarray_p2: Vec<u64> = std::iter::once(values[i] + values[j])
                .chain(
                    (0..values.len())
                        .filter(|&x| x != i && x != j)
                        .map(|x| values[x]),
                )
                .collect();

            let p1 = part1_recursive(&subarray_p1);
            let p2 = part1_recursive(&subarray_p2);

            if let Some(vv) = p1 {
                collect.extend(vv.clone());
                MEMO.insert(simple_hash(&subarray_p1), vv);
            }
            if let Some(vv) = p2 {
                collect.extend(vv.clone());
                MEMO.insert(simple_hash(&subarray_p2), vv);
            }
        }
    }

    Some(collect)
}

// fn part1_recursive(total_sum: u64, partial_sum: u64, values: &[u64]) -> bool {
//     if values[0] == total_sum {
//         return true;
//     }
//     if values.len() == 1 {
//         return false;
//     }
//
//     // let newvalues: Vec<u64>;
//     // if partial_sum == 0 {
//     //     newvalues = values.iter().copied().collect();
//     // } else {
//     //     newvalues = std::iter::once(partial_sum)
//     //         .chain(values.iter().copied())
//     //         .collect();
//     // }
//     // println!(
//     //     "sum: {}, partial sum: {}, values: {:?}",
//     //     total_sum, partial_sum, newvalues
//     // );
//
//     for i in 0..values.len() {
//         for j in 0..values.len() {
//             if i == j {
//                 continue;
//             }
//             // let subarray: Vec<u64> = (0..newvalues.len())
//             //     .filter(|&x| x != i && x != j)
//             //     .map(|x| newvalues[x])
//             //     .collect();
//             // println!("{:?}", subarray);
//             //
//             //
//             //
//
//             let subarray_p1:Vec<u64> = std::iter::once(partial_sum)
//                 .chain(
//                     (0..values.len())
//                         .filter(|&x| x != i && x != j)
//                         .map(|x| values[x])
//                 )
//                 .collect();
//
//             let p1 = part1_recursive(total_sum, newvalues[i] * newvalues[j], &subarray);
//
//
//             let p2 = part1_recursive(total_sum, newvalues[i] + newvalues[j], &subarray);
//
//
//             if p1 || p2 {
//                 return true;
//             }
//         }
//     }
//     false
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
