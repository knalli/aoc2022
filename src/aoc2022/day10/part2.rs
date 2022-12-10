use std::collections::HashMap;

use crate::aoc2022::day10::part1::parse_input;
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let instructions = parse_input(scope, "puzzle1");

    let mut pc = 1;
    let mut register_x = 1;
    let mut last_instruction: Option<usize> = None;
    let mut active_instruction: Option<(usize, i32)> = None;

    let mut crt: HashMap<i32, bool> = HashMap::new();

    loop {
        if let None = active_instruction {
            let next_instruction_idx = if let Some(last) = last_instruction {
                last + 1
            } else {
                // init
                0
            };
            if next_instruction_idx >= instructions.len() {
                break;
            }
            let next_instruction = instructions.get(next_instruction_idx).unwrap();
            let next_instruction_cycles = match next_instruction.name() {
                "noop" => 1,
                "addx" => 2,
                _ => 0,
            };
            active_instruction = Some((next_instruction_idx, next_instruction_cycles));
            //println!("Start cycle {}: begin executing {} {}", pc, next_instruction.name(), next_instruction.value());
        }

        if let Some((instruction_idx, cycles)) = active_instruction {

            //println!("Cycle {}", pc);
            let position_in_row = (pc % 40) - 1;
            crt.insert(pc, [register_x - 1, register_x, register_x + 1].contains(&position_in_row));

            //println!("During cycle {}: CRT draws pixel at position {}", pc, position_in_row);

            if cycles > 1 {
                active_instruction = Some((instruction_idx, cycles - 1));
            } else {
                let instruction = instructions.get(instruction_idx).unwrap();
                //println!("Processing {} {}", instruction.name(), instruction.value());
                match instruction.name() {
                    "addx" => {
                        register_x = register_x + instruction.value();
                    }
                    _ => ()
                };
                active_instruction = None;
                last_instruction = Some(instruction_idx);
            }
        }
        pc = pc + 1;
    }

    let result_list: Vec<String> = (1..pc).into_iter()
        .map(|i| {
            let ch = match crt.get(&i) {
                Some(true) => '#',
                _ => '.'
            };
            if i % 40 == 0 {
                String::from(ch) + "\n"
            } else {
                String::from(ch)
            }
        })
        .collect();
    let result = result_list.join("");

    // PAPJCBHP
    write_solution(&scope, format!("\n{}", result.as_str()).as_str());
}