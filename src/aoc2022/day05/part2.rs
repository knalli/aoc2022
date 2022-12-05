use std::collections::HashMap;

use crate::aoc2022::day05::part1::{get_stacks_top, parse_input};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let result = execute_puzzle(&scope, "puzzle1");
    write_solution(&scope, format!("top = {}", result).as_str());
}

fn execute_puzzle(scope: &PuzzleScope, puzzle: &str) -> String {
    let (stacks, instructions) = parse_input(io::read_puzzle_as_list(scope.day(), puzzle));

    let mut stack_map: HashMap<i32, Vec<char>> = HashMap::new();
    for stack in stacks {
        stack_map.entry(stack_map.len() as i32).or_insert(stack.clone());
    }
    for instruction in instructions {
        //println!("move {} from {} to {}", 1, instruction.from(), instruction.to());
        let from_idx = instruction.from() - 1;
        let to_idx = instruction.to() - 1;
        let from = stack_map.get_mut(&from_idx).unwrap();
        let mut values: Vec<char> = Vec::new();
        for _ in 0..instruction.amount() {
            values.push(from.pop().unwrap());
        }
        let to = stack_map.get_mut(&to_idx).unwrap();
        values.reverse();
        for value in values {
            to.push(value);
        }
    }

    get_stacks_top(stack_map)
}
