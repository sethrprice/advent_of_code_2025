use crate::{Solution, SolutionPair};
use ndarray::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Error};

const MOVES: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let topographic_map = parse_input("inputs/day10.txt").unwrap();
    let trailheads = find_entries(&topographic_map, 0);

    // Part 1

    let mut total_score = 0;
    for trailhead in trailheads.clone() {
        let mut nines: HashSet<(usize, usize)> = HashSet::new();
        let start = Position::new(0, trailhead);
        start.get_score(&topographic_map, &mut total_score, &mut nines);
    }

    // Part 2

    let mut total_rating = 0;
    for trailhead in trailheads {
        let start = Position::new(0, trailhead);
        start.get_rating(&topographic_map, &mut total_rating);
    }

    // Your solution here...
    let sol1: u64 = total_score as u64;
    let sol2: u64 = total_rating as u64;

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_input(filepath: &str) -> Result<Array2<u32>, Error> {
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<Vec<u32>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let length = lines.len();
    let width = lines[0].len();

    let flat_data: Vec<u32> = lines.into_iter().flatten().collect();

    let topographic_map = Array2::from_shape_vec((length, width), flat_data)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to build grid"))?;

    Ok(topographic_map)
}

fn find_entries(arr: &Array2<u32>, target: u32) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::new();
    for ((row, col), &value) in arr.indexed_iter() {
        if value == target {
            v.push((row, col));
        }
    }
    return v;
}

fn is_in_bounds(x: i32, y: i32, topographic_map: &Array2<u32>) -> bool {
    let (x_bound, y_bound) = topographic_map.dim();
    if x >= 0 && y >= 0 && x < x_bound as i32 && y < y_bound as i32 {
        return true;
    } else {
        return false;
    }
}

struct Position {
    height: u32,
    coords: (usize, usize),
}

impl Position {
    fn new(height: u32, coords: (usize, usize)) -> Self {
        Self { height, coords }
    }

    fn get_score(
        &self,
        topographic_map: &Array2<u32>,
        score: &mut usize,
        nines: &mut HashSet<(usize, usize)>,
    ) {
        for movement in MOVES {
            if let Some(new_position) = self.try_move(movement, topographic_map) {
                if new_position.height == 9 {
                    if nines.insert(new_position.coords) {
                        *score += 1;
                    }
                    continue;
                } else {
                    new_position.get_score(topographic_map, score, nines);
                }
            } else {
                continue;
            }
        }
    }

    fn get_rating(&self, topographic_map: &Array2<u32>, rating: &mut usize) {
        for movement in MOVES {
            if let Some(new_position) = self.try_move(movement, topographic_map) {
                if new_position.height == 9 {
                    *rating += 1;
                    continue;
                } else {
                    new_position.get_rating(topographic_map, rating);
                }
            } else {
                continue;
            }
        }
    }

    fn try_move(&self, movement: (i32, i32), topographic_map: &Array2<u32>) -> Option<Position> {
        let (x, y) = self.coords;
        let (dx, dy) = movement;
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;
        let coords = (new_x as usize, new_y as usize);
        if is_in_bounds(new_x, new_y, topographic_map) && topographic_map[coords] == self.height + 1
        {
            return Some(Position::new(self.height + 1, coords));
        } else {
            return None;
        }
    }
}
