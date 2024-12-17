use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::hash::Hash;

const POWERS_OF_10: [u64; 15] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
];

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let num_blinks = 75;

    let binding = read_to_string("inputs/day11.txt").unwrap();
    let stones: Vec<u64> = binding
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // let stones = vec![0, 1001];

    let mut stones_counter: HashMap<u64, u64> = HashMap::new();
    for stone in stones {
        stones_counter
            .entry(stone)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let final_stones = blink_n_times(stones_counter, num_blinks);

    // Your solution here...
    let sol1: u64 = final_stones.values().sum();
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn split_stone(stone: u64, num_digits: u64) -> Vec<u64> {
    let half_digits = num_digits / 2;
    let first = stone / POWERS_OF_10[half_digits as usize];
    let second = stone % POWERS_OF_10[half_digits as usize];
    vec![first, second]
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next_stones: HashMap<u64, u64> = HashMap::new();
    for (&stone, &count) in &stones {
        let num_digits = count_digits(stone);
        if stone == 0 {
            next_stones
                .entry(1)
                .and_modify(|counter| *counter += count)
                .or_insert(count);
        } else if num_digits % 2 == 0 {
            let two_stones = split_stone(stone, num_digits);
            for stone_part in two_stones {
                next_stones
                    .entry(stone_part)
                    .and_modify(|counter| *counter += count)
                    .or_insert(count);
            }
        } else {
            let bigger_stone = stone * 2024;
            next_stones
                .entry(bigger_stone)
                .and_modify(|counter| *counter += count)
                .or_insert(count);
        }
    }
    return next_stones;
}

fn blink_n_times(stones: HashMap<u64, u64>, n: u8) -> HashMap<u64, u64> {
    let mut stones_copy = stones.clone();
    for _ in 0..n {
        stones_copy = blink(stones_copy);
    }

    return stones_copy;
}

fn count_digits(mut num: u64) -> u64 {
    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}
