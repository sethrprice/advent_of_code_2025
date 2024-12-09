use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::{fs::read_to_string, io::Error};

///////////////////////////////////////////////////////////////////////////////

struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn new(lhs: u64, rhs: Vec<u64>) -> Self {
        Self {
            test_value: lhs,
            numbers: rhs,
        }
    }

    fn test_value(&self) -> u64 {
        self.test_value
    }

    fn numbers(&self) -> Vec<u64> {
        self.numbers.clone()
    }
}

pub fn solve() -> SolutionPair {
    let mut equations = parse_input("inputs/day7.txt").unwrap();

    let mut sum_of_test_values_1: u64 = 0;
    let mut sum_of_test_values_2: u64 = 0;

    for eq in equations.iter_mut() {
        println!("lhs = {}, rhs = {:?}", eq.test_value, eq.numbers);
        let lhs = eq.test_value;
        if works(eq.test_value(), &eq.numbers(), false) {
            sum_of_test_values_1 += lhs;
        }
        if works(eq.test_value(), &eq.numbers(), true) {
            sum_of_test_values_2 += lhs;
        }
    }

    // Your solution here...
    let sol1: u64 = sum_of_test_values_1;
    let sol2: u64 = sum_of_test_values_2;

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_input(filepath: &str) -> Result<Vec<Equation>, Error> {
    let raw_input = read_to_string(filepath)?;

    let mut equations: Vec<Equation> = Vec::new();

    for line in raw_input.lines() {
        // key, value pair
        let (n, v) = line
            .split_once(": ")
            .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidData, "Invalid input"))?;

        // parse test value as u64
        let n_parsed = n.parse::<u64>().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to parse number")
        })?;

        // parse numbers as u64
        let v_parsed: Vec<u64> = v
            .split_ascii_whitespace()
            .map(|num| {
                num.parse::<u64>().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to parse number")
                })
            })
            .collect::<Result<_, _>>()?;

        let new_equation = Equation::new(n_parsed, v_parsed);

        equations.push(new_equation);
    }
    Ok(equations)
}

fn endswith(a: u64, b: u64) -> bool {
    let digits: u32 = digits(b);
    return ((a - b) % 10).pow(digits) == 0;
}

fn digits(b: u64) -> u32 {
    return b.checked_ilog10().unwrap_or(0) + 1;
}

fn works(lhs: u64, rhs: &[u64], check_concat: bool) -> bool {
    if let Some((&n, head)) = rhs.split_last() {
        if head.is_empty() {
            return n == lhs;
        }

        let (q, r) = (lhs / n, lhs % n);
        if r == 0 && works(q, head, check_concat) {
            return true;
        }

        if check_concat && endswith(lhs, n) {
            let reduced = lhs / 10_u64.pow(digits(n));
            if works(reduced, head, check_concat) {
                return true;
            }
        }

        works(lhs - n, head, check_concat)
    } else {
        false
    }
}
