use std::collections::HashSet;

use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let signal = find_signal(scope, "puzzle1", 4);
    write_solution(&scope, format!("signal = {}", signal).as_str());
}

pub fn find_signal(scope: &PuzzleScope, puzzle: &str, length: usize) -> i32 {
    let input = io::read_puzzle_as_string(scope.day(), puzzle);
    for i in length..input.len() {
        if input[(i - length)..i].chars().collect::<HashSet<_>>().len() == length {
            return i as i32;
        }
    }
    -1
}
