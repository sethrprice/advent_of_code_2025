use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead, Error};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let disk_map = read_to_string("inputs/day9.txt").unwrap();

    // Part 1
    let mut compressed_disk: Vec<Option<usize>> = Vec::new();
    let mut empty_indices: Vec<usize> = Vec::new();

    // Initialise compressed disk
    for (i, c) in disk_map.chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            for _ in 0..n {
                compressed_disk.push(Some(i / 2));
            }
        } else {
            for _ in 0..n {
                empty_indices.push(compressed_disk.len());
                compressed_disk.push(None);
            }
        }
    }

    let mut index = 0;
    for (i, op) in compressed_disk.clone().iter().enumerate().rev() {
        if let Some(_) = op {
            if let Some(b) = empty_indices.get(index) {
                if *b > i {
                    break;
                }
                compressed_disk.swap(i, *b);
                index += 1;
            } else {
                break;
            }
        }
    }

    let checksum_1: usize = compressed_disk
        .iter()
        .enumerate()
        .map(|(i, op)| i * op.unwrap_or(0))
        .sum();

    // Part 2

    let mut compressed_disk: Vec<Option<usize>> = Vec::new();
    let mut empty_indices: Vec<usize> = Vec::new();
    let mut file_indices: Vec<usize> = Vec::new();

    // Initialise compressed disk
    for (i, c) in disk_map.chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            for _ in 0..n {
                compressed_disk.push(Some(i / 2));
                file_indices.push(compressed_disk.len() - 1);
            }
        } else {
            for _ in 0..n {
                empty_indices.push(compressed_disk.len());
                compressed_disk.push(None);
            }
        }
    }

    let disk_sizes: Vec<usize> = disk_map
        .chars()
        .step_by(2)
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let disk_sizes_copy = disk_sizes.clone();

    let mut disk_spaces: Vec<usize> = disk_map
        .chars()
        .skip(1)
        .step_by(2)
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let disk_spaces_copy = disk_spaces.clone();

    let mut swap_indices: Vec<(usize, usize)> = Vec::new();

    for (i, file_size) in disk_sizes.iter().enumerate().rev() {
        for (j, space_size) in disk_spaces.iter_mut().enumerate() {
            if j > i {
                break;
            }
            if file_size > space_size {
                continue;
            } else {
                for k in 0..*file_size {
                    let mut index: usize = disk_spaces_copy.iter().take(j).sum();
                    index += disk_spaces_copy[j] - *space_size;
                    let mut index_2: usize = disk_sizes_copy.iter().take(i).sum();
                    index_2 += file_size - 1;
                    swap_indices.push((empty_indices[index + k], file_indices[index_2 - k]));
                }
                *space_size -= file_size;

                break;
            }
        }
    }

    for (a, b) in swap_indices {
        compressed_disk.swap(a, b);
    }

    let checksum_2: usize = compressed_disk
        .iter()
        .enumerate()
        .map(|(i, op)| if let Some(n) = op { n * i } else { 0 })
        .sum();

    // Your solution here...
    let sol1: u64 = checksum_1 as u64;
    let sol2: u64 = checksum_2 as u64;

    (Solution::from(sol1), Solution::from(sol2))
}
