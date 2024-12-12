use crate::{Solution, SolutionPair};
use itertools::Itertools;
use ndarray::prelude::*;
use std::collections::HashSet;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead, Error};

const MOVES: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let grid = parse_input("inputs/day6.txt").unwrap();
    let start = find_character(&grid, '^').unwrap();

    // Part 1

    let locations_visited: HashSet<Location> = record_route(start, &grid).unwrap();
    let scalar_locations_visited: HashSet<(usize, usize)> =
        locations_visited.iter().map(|l| l.location()).collect();

    // Part 2

    let loop_points = find_loop_points(start, grid, locations_visited);

    let sol1: u64 = scalar_locations_visited.len() as u64;
    let sol2: u64 = loop_points.len() as u64;

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_tuple(self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn rotate(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Location {
    location: (usize, usize),
    movement: Direction,
}

impl Location {
    fn new(location: (usize, usize), movement: Direction) -> Self {
        Self { location, movement }
    }

    fn step(&self) -> Option<Location> {
        let (row, col) = self.location;
        let (dr, dc) = self.movement.to_tuple();
        // Attempt to calculate new position with bounds checks
        let new_row = usize::try_from(row as isize + dr).ok()?;
        let new_col = usize::try_from(col as isize + dc).ok()?;
        Some(Location::new((new_row, new_col), self.movement))
    }

    fn get_object(&self, grid: &Array2<char>) -> Option<char> {
        let shape = grid.dim();
        if self.location.0 >= shape.0 || self.location.1 >= shape.1 {
            return None;
        } else {
            let object = grid[self.location];
            return Some(object);
        }
    }

    fn location(&self) -> (usize, usize) {
        self.location
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

fn find_character(grid: &Array2<char>, target: char) -> Option<(usize, usize)> {
    for ((row, col), &value) in grid.indexed_iter() {
        if value == target {
            return Some((row, col));
        }
    }
    return None;
}

fn record_route(start: (usize, usize), grid: &Array2<char>) -> Option<HashSet<Location>> {
    let mut loc = start;
    let mut movement = Direction::Up;
    let mut locations_set = HashSet::new();
    locations_set.insert(Location::new(loc, movement));

    loop {
        if let Some(next_location) = Location::new(loc, movement).step() {
            if let Some(next_object) = next_location.get_object(grid) {
                if next_object == '#' {
                    movement = movement.rotate();
                } else {
                    loc = next_location.location();
                }
                if !locations_set.insert(Location::new(loc, movement)) {
                    return None;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    return Some(locations_set);
}

fn find_loop_points(
    start: (usize, usize),
    grid: Array2<char>,
    route: HashSet<Location>,
) -> HashSet<(usize, usize)> {
    let mut loop_points: HashSet<(usize, usize)> = HashSet::new();
    let start_location = Location::new(start, Direction::Up);
    let mut test_grid = grid.clone();
    for location_i in route.difference(&HashSet::from([start_location])) {
        if let Some(original_value) = location_i.get_object(&test_grid) {
            let obstacle_location = location_i.location();
            if original_value == '#' {
                continue;
            }
            test_grid[obstacle_location] = '#';
            match record_route(start, &test_grid) {
                Some(_) => {
                    test_grid[obstacle_location] = original_value;
                    continue;
                }
                None => {
                    loop_points.insert(obstacle_location);
                    test_grid[obstacle_location] = original_value;
                }
            }
        }
    }

    return loop_points;
}
