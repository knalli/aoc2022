use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
use crate::aoc2022::lib::io;
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

    let mut signal_strength_results: Vec<i32> = vec!();

    loop {
        if let None = active_instruction {
            let next_instruction = if let Some(last) = last_instruction {
                last + 1
            } else {
                // init
                0
            };
            if next_instruction >= instructions.len() {
                break;
            }
            let next_instruction_cycles = match instructions.get(next_instruction).unwrap().name() {
                "noop" => 1,
                "addx" => 2,
                _ => 0,
            };
            active_instruction = Some((next_instruction, next_instruction_cycles));
        }

        //println!("Cycle {}", pc);

        // signal strength are 40er steps minus 20
        if (pc + 20) % 40 == 0 {
            signal_strength_results.push(pc * register_x);
        }

        if let Some((instruction_idx, cycles)) = active_instruction {
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

    write_solution(&scope, format!("register x = {}", register_x).as_str());

    let signal_strength_sum: i32 = signal_strength_results.iter().sum();
    write_solution(&scope, format!("signal strength sum = {}", signal_strength_sum).as_str());
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Vec<Instruction> {
    io::read_puzzle_as_list(scope.day(), puzzle).iter()
        .map(|line| {
            let split: Vec<&str> = line.split(' ').collect();
            match split.get(0) {
                Some(&name) => {
                    let value = match split.get(1) {
                        Some(value) => parse_int(value),
                        _ => 0,
                    };
                    Instruction::new(name.to_string(), value)
                }
                _ => Instruction::new("invalid".to_string(), 0)
            }
        })
        .collect()
}

pub struct Instruction {
    name: String,
    value: i32,
}

impl Instruction {
    pub fn new(name: String, value: i32) -> Self {
        return Instruction {
            name,
            value,
        };
    }


    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
