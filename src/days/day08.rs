use crate::{Solution, SolutionPair};
use itertools::Itertools;
use ndarray::prelude::*;
use std::collections::HashSet;
use std::fs::{read_to_string, File};
use std::hash::Hash;
use std::io::{self, BufRead, Error};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let grid = parse_input("inputs/day8.txt").unwrap();
    let antennas = find_antennas(&grid);

    // Part 1
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..antennas.len() {
        for j in 0..antennas.len() {
            if i == j {
                continue;
            }
            if let Some(antinode) = antennas[i].find_antinode(&antennas[j], &grid) {
                let _ = antinodes.insert(antinode);
            }
        }
    }

    // Part 2
    let mut all_antinodes: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..antennas.len() {
        for j in (i + 1)..antennas.len() {
            if let Some(antinode_set) = antennas[i].find_all_antinodes(&antennas[j], &grid) {
                all_antinodes.extend(antinode_set);
            }
        }
    }

    // Your solution here...
    let sol1: u64 = antinodes.len() as u64;
    let sol2: u64 = all_antinodes.len() as u64;

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug)]
struct Antenna {
    frequency: char,
    coords: (usize, usize),
}

impl Antenna {
    fn new(frequency: char, coords: (usize, usize)) -> Self {
        Self { frequency, coords }
    }

    fn displacement_from(&self, other: &Antenna) -> (isize, isize) {
        let (x1, y1) = self.coords;
        let (x2, y2) = other.coords;
        return (x1 as isize - x2 as isize, y1 as isize - y2 as isize);
    }

    // each antenna is associated with one antinode
    fn find_antinode(&self, other: &Antenna, grid: &Array2<char>) -> Option<(usize, usize)> {
        if self.frequency != other.frequency {
            return None;
        }
        let (dx, dy) = self.displacement_from(other);
        let (x, y) = self.coords;
        let (ax, ay) = (x as isize + dx, y as isize + dy);
        if in_bounds((ax, ay), grid) {
            Some((ax as usize, ay as usize))
        } else {
            None
        }
    }

    fn find_all_antinodes(
        &self,
        other: &Antenna,
        grid: &Array2<char>,
    ) -> Option<HashSet<(usize, usize)>> {
        if self.frequency != other.frequency {
            return None;
        }
        let mut antinode_line: HashSet<(usize, usize)> = HashSet::new();
        let (x, y) = self.coords;
        let (dx, dy) = self.displacement_from(other);
        let mut i = 0;
        // Find antinodes on positive side
        loop {
            let (ax, ay) = (x as isize + i * dx, y as isize + i * dy);
            // println!("({ax},{ay})");
            if !in_bounds((ax, ay), grid) {
                break;
            }
            let _ = antinode_line.insert((ax as usize, ay as usize));
            i += 1;
        }
        // Find antinodes on negative side
        let mut i = 0;
        loop {
            i += 1;
            let (ax, ay) = (x as isize - i * dx, y as isize - i * dy);
            if !in_bounds((ax, ay), grid) {
                break;
            }
            let _ = antinode_line.insert((ax as usize, ay as usize));
        }
        Some(antinode_line)
    }
}

fn parse_input(filepath: &str) -> Result<Array2<char>, Error> {
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    let length = lines.len();
    let width = lines[0].len();

    let flat_data: Vec<char> = lines.into_iter().flatten().collect();

    let grid = Array2::from_shape_vec((length, width), flat_data)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to build grid"))?;

    Ok(grid)
}

fn in_bounds(coords: (isize, isize), grid: &Array2<char>) -> bool {
    let (width, length) = grid.dim();
    let (x, y) = coords;
    if x < 0 || y < 0 || x >= width as isize || y >= length as isize {
        false
    } else {
        true
    }
}

fn find_antennas(grid: &Array2<char>) -> Vec<Antenna> {
    let mut antennas: Vec<Antenna> = Vec::new();
    for ((row, col), &value) in grid.indexed_iter() {
        if value != '.' {
            antennas.push(Antenna::new(value, (row, col)));
        }
    }
    return antennas;
}
