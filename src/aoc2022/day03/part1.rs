use std::collections::HashSet;

use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let sum: i32 = resolve_priority_sum(scope, "puzzle1");
    write_solution(&scope, format!("sum = {}", sum).as_str());
}

fn resolve_priority_sum(scope: &PuzzleScope, puzzle: &str) -> i32 {
    let mut sum = 0;
    for line in io::read_puzzle_as_list(scope.day(), puzzle) {
        let compartment_size = line.len() / 2;
        let compartment1: HashSet<_> = line.chars()
            .take(compartment_size)
            .collect();
        let compartment2: HashSet<_> = line.chars()
            .skip(compartment_size)
            .collect();
        let overlaps: Vec<_> = compartment1.intersection(&compartment2)
            .collect();
        assert_eq!(1, overlaps.len());
        let ch = **overlaps.last().unwrap();
        let priority = decode_item(ch);
        sum += priority;
    }
    sum
}

pub fn decode_item(ch: char) -> i32 {
    match ch {
        'a'..='z' => {
            (ch as i32) - 96 // ASCII, a is 97
        }
        'A'..='Z' => {
            (ch as i32) - 64 + 26 // ASCII, A is 65, plus offset (A=27)
        }
        _ => {
            0
        }
    }
}
