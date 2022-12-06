use crate::aoc2022::day06::part1::find_signal;
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let signal = find_signal(scope, "puzzle1", 14);
    write_solution(&scope, format!("signal = {}", signal).as_str());
}
