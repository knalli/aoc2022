use std::collections::HashMap;

use crate::aoc2022::day01::part1::build_carries;
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let map: HashMap<i32, i32> = build_carries(scope, "puzzle1");
    let mut values: Vec<i32> = map.values()
        .into_iter()
        .map(|x| *x)
        .collect();
    values.sort_by(|x, y| y.cmp(&x));
    let max: i32 = values.iter().take(3).sum();
    write_solution(&scope, format!("max calories by first three = {}", max).as_str());
}
