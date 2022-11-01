use crate::aoc2022::lib::common::PuzzleScope;

pub fn write_header(scope: &PuzzleScope) {
    println!();
    println!("Advent Of Code {y}: Day {d} Part {p}", y = scope.year(), d = scope.day(), p = scope.part());
}

pub fn write_solution(scope: &PuzzleScope, str: &str) {
    println!("The solution of Part {p} is: {s}", p = scope.part(), s = str);
}