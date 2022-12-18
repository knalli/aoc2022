use std::fmt::Error;
use std::hash::{Hash};
use std::ops::{Add, Sub};
use std::str::FromStr;

use itertools::Itertools;

use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle1");
    let surface = count_surface(&input);
    write_solution(&scope, format!("surface = {}", surface).as_str());
}

#[derive(Debug, Clone, Hash)]
pub struct Cube(pub i64, pub i64, pub i64);

impl FromStr for Cube {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(',').collect_vec();
        let cube = Cube(
            parse_int(split[0]) as i64,
            parse_int(split[1]) as i64,
            parse_int(split[2]) as i64,
        );
        Ok(cube)
    }
}

impl PartialEq<Self> for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for Cube {}

impl Sub for Cube {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add for Cube {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Cube {}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Vec<Cube> {
    io::read_puzzle_as_list(scope.day(), puzzle)
        .iter()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn count_surface(cubes: &[Cube]) -> usize {
    cubes.iter()
        .map(|from| {
            6 - cubes.iter()
                .filter(|&to| !from.eq(to))
                .filter(|&to| {
                    let r = to.clone() - from.clone();
                    if r.0.abs() == 1 && r.1 == 0 && r.2 == 0 {
                        return true;
                    }
                    if r.0 == 0 && r.1.abs() == 1 && r.2 == 0 {
                        return true;
                    }
                    if r.0 == 0 && r.1 == 0 && r.2.abs() == 1 {
                        return true;
                    }
                    false
                })
                .count()
        })
        .sum()
}
