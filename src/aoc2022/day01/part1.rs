use std::collections::HashMap;

use crate::aoc2022::lib;
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let map: HashMap<i32, i32> = build_carries(scope, "puzzle1");
    let max = map.values().max().unwrap();
    write_solution(&scope, format!("max calories = {}", max).as_str());
}

pub fn build_carries(scope: &PuzzleScope, puzzle: &str) -> HashMap<i32, i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut count = 0;
    for v in lib::io::read_puzzle_as_list(scope.day(), puzzle) {
        if v.len() == 0 {
            count += 1;
        } else {
            let num = lib::common::parse_int(&v);
            map.entry(count)
                .and_modify(|v| { *v += num })
                .or_insert(num);
        }
    }
    map
}
