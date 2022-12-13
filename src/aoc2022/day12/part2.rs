use std::cell::RefCell;
use crate::aoc2022::day12::part1::{fewest_steps, Input, parse_input};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle1");
    let starts: Vec<(i32, i32)> = input.map.iter()
        .filter(|(_, c)| **c == 'a' || **c == 'S')
        .map(|(pos, _)| pos.clone())
        .collect();
    let min_cost: RefCell<i32> = RefCell::new(i32::MAX);
    for start in starts {
        let (_, cost) = fewest_steps(Input {start, map: input.map.clone()});
        if let Some(v) = cost {
            min_cost.replace(min_cost.clone().into_inner().min(v));
        }
    }
    write_solution(&scope, format!("cost = {}", min_cost.into_inner()).as_str());
}