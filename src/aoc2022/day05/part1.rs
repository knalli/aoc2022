use std::collections::HashMap;

use crate::aoc2022::day05::model::Instruction;
use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
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
        for _ in 0..instruction.amount() {
            //println!("move {} from {} to {}", 1, instruction.from(), instruction.to());
            let from_idx = instruction.from() - 1;
            let to_idx = instruction.to() - 1;
            let from = stack_map.get_mut(&from_idx).unwrap();
            let value = from.pop().unwrap();
            let to = stack_map.get_mut(&to_idx).unwrap();
            to.push(value);
        }
    }

    get_stacks_top(stack_map)
}

pub fn get_stacks_top(stack_map: HashMap<i32, Vec<char>>) -> String {
    let result: String = (0..stack_map.len())
        .map(|i| {
            let x = i as i32;
            stack_map.get(&x).unwrap().last().unwrap()
        })
        .collect();
    result
}


pub fn parse_input(lines: Vec<String>) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let mut setup: Vec<String> = Vec::new();
    for line in lines.clone() {
        if line.is_empty() {
            break;
        }
        setup.push(line.to_string());
    }
    // ok, rest are instructions
    let instructions: Vec<Instruction> = lines.iter()
        //.skip(stacks.len() + 1)
        .filter(|line| line.starts_with("move"))
        .map(|line| {
            // move A from B to C
            let split: Vec<&str> = line.split(' ').collect();
            Instruction::new(parse_int(split[1]), parse_int(split[3]), parse_int(split[5]))
        })
        .collect();
    // parse stacks
    let mut stacks: Vec<Vec<char>> = Vec::new();
    // number of stacks is last line divided 3 + 1 (because of trimming)
    let number_of_stacks = (setup.last().unwrap().len() / 4) + 1;
    // max height is number of lines - 1
    let stack_height = setup.len() - 1;

    setup.reverse();
    for i in 0..number_of_stacks {
        let mut stack: Vec<char> = Vec::new();
        for h in 0..stack_height {
            let line = setup.get(h + 1).unwrap().clone();
            let offset = i * 4;
            let start = offset + 1;
            let end = offset + 2;
            if line.len() < end {
                break;
            }
            let ch = line[start..end].chars().next().unwrap();
            if ch == ' ' {
                break;
            }
            stack.push(ch);
        }
        stacks.push(stack);
    }

    (stacks, instructions)
}