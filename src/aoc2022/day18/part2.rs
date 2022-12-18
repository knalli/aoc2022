use std::collections::{HashSet, VecDeque};
use std::ops::Add;

use crate::aoc2022::day18::part1::{Cube, parse_input};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    // !4032
    let input = parse_input(scope, "puzzle1");
    let surface = count_exterior_surface(&input);
    write_solution(&scope, format!("surface = {}", surface).as_str());
}

fn count_exterior_surface(cubes: &[Cube]) -> usize {
    let ops = vec![
        Cube(0, 0, -1),
        Cube(0, 0, 1),
        Cube(0, -1, 0),
        Cube(0, 1, 0),
        Cube(-1, 0, 0),
        Cube(1, 0, 0),
    ];

    let min = cubes.iter().map(|v| v.0.min(v.1).min(v.2)).min().unwrap() - 1;
    let max = cubes.iter().map(|v| v.0.max(v.1).max(v.2)).max().unwrap() + 1;
    let valid_range = min..=max;

    let mut count = 0;
    let mut visited: HashSet<Cube> = HashSet::new();

    let mut q = VecDeque::from([Cube(min, min, min)]);
    visited.insert(Cube(min, min, min));

    while let Some(next) = q.pop_front() {
        for op in ops.clone() {
            let adjacent = next.clone().add(op);
            if valid_range.contains(&adjacent.0) && valid_range.contains(&adjacent.1) & valid_range.contains(&adjacent.2) {
                if cubes.contains(&adjacent) {
                    count += 1;
                    visited.insert(adjacent.clone());
                    continue;
                }
                if !visited.contains(&adjacent) {
                    visited.insert(adjacent.clone());
                    q.push_back(adjacent);
                }
            }
        }
    }

    count
}