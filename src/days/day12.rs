use crate::{Solution, SolutionPair};
use core::fmt;
use ndarray::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead, Error};

const MOVES: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let plant_map = parse_input("inputs/day12.txt").unwrap();
    let mut seen_coords: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<Region> = Vec::new();

    for ((row, col), &plot) in plant_map.indexed_iter() {
        if seen_coords.contains(&(row, col)) {
            continue;
        }
        let this_plot = Plot::new(plot, (row, col));
        let mut coordinates: HashSet<(usize, usize)> = HashSet::new();
        let mut perimeter: HashSet<((usize, usize), (i32, i32))> = HashSet::new();
        let region = this_plot.get_region(&plant_map, &mut coordinates, &mut perimeter);
        for c in region.coords() {
            seen_coords.insert(c);
        }
        regions.push(region);
    }

    // Part 1

    let total_price_1: u64 = regions.iter().map(|r| r.get_price()).sum();

    // Part 2

    let discounted_price: u64 = regions.iter().map(|r| r.get_discount_price()).sum();

    // Your solution here...
    let sol1: u64 = total_price_1;
    let sol2: u64 = discounted_price;

    (Solution::from(sol1), Solution::from(sol2))
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

    let plant_map = Array2::from_shape_vec((length, width), flat_data)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to build grid"))?;

    Ok(plant_map)
}

fn is_in_bounds(x: i32, y: i32, plant_map: &Array2<char>) -> bool {
    let (x_bound, y_bound) = plant_map.dim();
    if x >= 0 && y >= 0 && x < x_bound as i32 && y < y_bound as i32 {
        return true;
    } else {
        return false;
    }
}

struct Plot {
    plant_type: char,
    coords: (usize, usize),
}

impl Plot {
    fn new(plant_type: char, coords: (usize, usize)) -> Self {
        Self { plant_type, coords }
    }

    fn coords(&self) -> (usize, usize) {
        self.coords
    }

    fn try_neighbour(&self, movement: (i32, i32), plant_map: &Array2<char>) -> Option<Plot> {
        let (x, y) = self.coords;
        let (dx, dy) = movement;
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;
        let coords = (new_x as usize, new_y as usize);
        if is_in_bounds(new_x, new_y, plant_map) && plant_map[coords] == self.plant_type {
            return Some(Plot::new(self.plant_type, coords));
        } else {
            return None;
        }
    }

    fn get_region(
        &self,
        plant_map: &Array2<char>,
        coordinates: &mut HashSet<(usize, usize)>,
        perimeter: &mut HashSet<((usize, usize), (i32, i32))>,
    ) -> Region {
        coordinates.insert(self.coords());
        for movement in MOVES {
            if let Some(new_plant) = self.try_neighbour(movement, plant_map) {
                if coordinates.contains(&new_plant.coords()) {
                    continue;
                }
                coordinates.insert(new_plant.coords());
                new_plant.get_region(plant_map, coordinates, perimeter);
            } else {
                let new_perim = (self.coords(), movement);
                perimeter.insert(new_perim);
                continue;
            }
        }
        Region::new(self.plant_type, coordinates.clone(), perimeter.clone())
    }
}

#[derive(Debug)]
struct Region {
    plant_type: char,
    coords: HashSet<(usize, usize)>,
    area: usize,
    perimeter: HashSet<((usize, usize), (i32, i32))>,
}

impl Region {
    fn new(
        plant_type: char,
        coords: HashSet<(usize, usize)>,
        perimeter: HashSet<((usize, usize), (i32, i32))>,
    ) -> Self {
        let coords_copy = coords.clone();
        Self {
            plant_type,
            coords,
            area: coords_copy.len(),
            perimeter,
        }
    }

    fn coords(&self) -> HashSet<(usize, usize)> {
        self.coords.clone()
    }

    fn get_price(&self) -> u64 {
        (self.area * self.perimeter.len()) as u64
    }

    fn get_discount_price(&self) -> u64 {
        let n_sides = self.get_n_sides();
        self.area as u64 * n_sides
    }

    fn get_n_sides(&self) -> u64 {
        let mut n_sides: u64 = 4;
        for facing in MOVES {
            let mut fences: Vec<(usize, usize)> = self
                .perimeter
                .iter()
                .filter(|(_, m)| *m == facing)
                .map(|((x, y), m)| if m.0 == 0 { (*y, *x) } else { (*x, *y) })
                .collect();
            fences.sort_by(|(x1, y1), (x2, y2)| x1.cmp(x2).then_with(|| y1.cmp(y2)));
            println!("{:?}", fences);
            for (&(x_curr, y_curr), &(x_prev, y_prev)) in fences.iter().skip(1).zip(fences.iter()) {
                if y_curr - y_prev == 1 && x_curr - x_prev == 0 {
                    continue;
                } else {
                    n_sides += 1;
                }
            }
        }
        println!("{} has {} sides", self.plant_type, n_sides);
        return n_sides;
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.coords.iter().map(|(x, _)| x).max().unwrap_or(&10);
        let max_y = self.coords.iter().map(|(_, y)| y).max().unwrap_or(&10);
        let mut arr: Array2<char> = Array::from_elem((*max_x + 1, *max_y + 1), '.');
        for &c in self.coords.iter() {
            arr[c] = self.plant_type;
        }
        write!(f, "{}", arr)
    }
}
