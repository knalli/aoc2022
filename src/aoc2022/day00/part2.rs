use crate::aoc2022::lib::common::PuzzleScope;
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);

    let ints = io::read_puzzle_as_ints(scope.day(), "puzzle1");
    let sum: i32 = ints.iter().sum();
    write_solution(&scope, format!("sum = {}", sum).as_str());
}
