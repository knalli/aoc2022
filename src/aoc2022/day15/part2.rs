use std::borrow::Borrow;

use crate::aoc2022::day15::part1::{build_map, parse_input};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle1");
    let map = build_map(input.borrow());
    let found = map.find_uncovered(0..=4_000_000).unwrap();
    let freq = found.0 as usize * 4_000_000 + found.1 as usize;
    write_solution(&scope, format!("found {:?}, tuning_freq = {}", found, freq).as_str());
}
