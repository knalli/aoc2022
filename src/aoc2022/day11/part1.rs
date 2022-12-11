use std::collections::HashMap;

use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let mut game = Game {
        monkeys: parse_input(scope, "puzzle1"),
        items: HashMap::new(),
        inspections: HashMap::new(),
    };

    let rounds = 20;

    play_game(&mut game, rounds, true, 1, 1, 3, 1);
    println!();

    let mut mm: Vec<(usize, usize)> = vec!();
    game.inspections.iter().for_each(|(monkey_id, count)| mm.push((*count, *monkey_id)));
    mm.sort_by(|a, b| b.0.cmp(&a.0));
    let result = mm.get(0).unwrap().0 * mm.get(1).unwrap().0;

    write_solution(&scope, format!("result = {}", result).as_str());
}

pub struct Game {
    pub monkeys: Vec<Monkey>,
    pub items: HashMap<usize, Vec<usize>>,
    pub inspections: HashMap<usize, usize>,
}

pub fn play_game(game: &mut Game, rounds: usize, print_steps: bool, print_summary_on_nth: usize, print_inspections_on_nth: usize, div_by: usize, mod_by: usize) {
    for monkey_id in 0..game.monkeys.len() {
        let monkey = game.monkeys.get(monkey_id).unwrap();
        game.items.insert(monkey_id, monkey.start_items().clone());
    }

    for round in 1..=rounds {
        for monkey_id in 0..game.monkeys.len() {
            let monkey = game.monkeys.get(monkey_id).unwrap();
            if print_steps {
                println!("Monkey {}:", monkey_id);
            }
            let items = game.items.get(&monkey_id).unwrap().clone();
            game.items.insert(monkey_id, vec!()); // clear
            for item in items {
                if print_steps {
                    println!("  Monkey inspects an item with a worry level of {}.", item);
                }
                game.inspections.entry(monkey_id)
                    .and_modify(|c| *c = *c + 1)
                    .or_insert(1);
                let mut level = item;
                level = match monkey.operation() {
                    Operation::MultiplyOld => {
                        let result = level * level;
                        if print_steps {
                            println!("    Worry level is multiplied by itself to {}.", result);
                        }
                        result
                    }
                    Operation::AddOld => {
                        let result = level + level;
                        if print_steps {
                            println!("    Worry level increases by itself to {}.", result);
                        }
                        result
                    }
                    Operation::MultiplyValue(v) => {
                        let result = level * v;
                        if print_steps {
                            println!("    Worry level is multiplied by {} to {}.", v, result);
                        }
                        result
                    }
                    Operation::AddValue(v) => {
                        let result = level + v;
                        if print_steps {
                            println!("    Worry level increases by {} to {}.", v, result);
                        }
                        result
                    }
                };
                if div_by > 1 && level > div_by {
                    level = level / div_by; // REVISIT
                }
                if mod_by > 1 {
                    level = level % mod_by;
                }
                if print_steps {
                    println!("    Monkey gets bored with item. Worry level is divided by {} to {}.", div_by, level);
                }
                if level % monkey.test_div() == 0 {
                    if print_steps {
                        println!("    Current worry level is divisible by {}", monkey.test_div());
                    }
                    let other = game.items.get_mut(&monkey.test_if_true()).unwrap();
                    other.push(level);
                    if print_steps {
                        println!("    Item with worry level {} is thrown to monkey {}.", level, monkey.test_if_true());
                    }
                } else {
                    if print_steps {
                        println!("    Current worry level is not divisible by {}", monkey.test_div);
                    }
                    let other = game.items.get_mut(&monkey.test_if_false()).unwrap();
                    other.push(level);
                    if print_steps {
                        println!("    Item with worry level {} is thrown to monkey {}.", level, monkey.test_if_false());
                    }
                }
            }
        }

        if round % print_summary_on_nth == 0 || round % print_inspections_on_nth == 0 {
            println!();
            println!("== After round {} ==", round);
            if round % print_summary_on_nth == 0 {
                for monkey_id in 0..game.monkeys.len() {
                    let item_str_list: Vec<String> = game.items.get(&monkey_id)
                        .unwrap()
                        .iter()
                        .map(|d| d.to_string())
                        .collect();
                    println!("Monkey {}: {}", monkey_id, item_str_list.join(", "));
                }
            }
            if round % print_inspections_on_nth == 0 {
                for monkey_id in 0..game.monkeys.len() {
                    let count = game.inspections.get(&monkey_id).unwrap();
                    println!("Monkey {} inspected items {} times.", monkey_id, count);
                }
            }
            println!();
        }
    }
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Vec<Monkey> {
    let mut result: Vec<Monkey> = vec!();

    let mut m_items: Vec<usize> = vec!();
    let mut m_operation: Operation = Operation::MultiplyOld;
    let mut m_test: usize = 1;
    let mut m_test_true: usize = 0;
    let mut m_test_false: usize = 1;
    for line in io::read_puzzle_as_list(scope.day(), puzzle) {
        if line.starts_with("Monkey") {
            // skip
        } else if line.contains("Starting") {
            let mut split: Vec<&str> = line.split(": ").collect();
            split = split.get(1).unwrap().split(", ").collect();
            m_items = split.iter()
                .map(|s| parse_int(s) as usize)
                .collect();
        } else if line.contains("Operation") {
            let split: Vec<&str> = line.split(": new = old ").collect();
            let temp = split.get(1).unwrap();
            let parts: Vec<&str> = temp.split(" ").collect();
            match parts.get(1).unwrap().to_string().as_str() {
                "old" => {
                    match parts.get(0).unwrap().to_string().as_str() {
                        "+" => m_operation = Operation::AddOld,
                        "*" => m_operation = Operation::MultiplyOld,
                        _ => ()
                    }
                }
                expr2 => {
                    let v = parse_int(expr2) as usize;
                    match parts.get(0).unwrap().to_string().as_str() {
                        "+" => m_operation = Operation::AddValue(v),
                        "*" => m_operation = Operation::MultiplyValue(v),
                        _ => ()
                    }
                }
            }
        } else if line.contains("Test: ") {
            let split: Vec<&str> = line.split("Test: divisible by ").collect();
            m_test = parse_int(split.get(1).unwrap()) as usize;
        } else if line.contains("If true") {
            let split: Vec<&str> = line.split("throw to monkey ").collect();
            m_test_true = parse_int(split.get(1).unwrap()) as usize;
        } else if line.contains("If false") {
            let split: Vec<&str> = line.split("throw to monkey ").collect();
            m_test_false = parse_int(split.get(1).unwrap()) as usize;
        } else if line.is_empty() {
            let monkey = Monkey::new(
                m_items,
                m_operation,
                m_test,
                m_test_true,
                m_test_false,
            );
            result.push(monkey);
            m_items = vec!();
            m_operation = Operation::AddOld;
            m_test = 0;
            m_test_true = 0;
            m_test_false = 0;
        }
    }

    // last
    let monkey = Monkey::new(
        m_items,
        m_operation,
        m_test,
        m_test_true,
        m_test_false,
    );
    result.push(monkey);

    result
}

pub struct Monkey {
    start_items: Vec<usize>,
    operation: Operation,
    test_div: usize,
    test_if_true: usize,
    test_if_false: usize,
}

impl Monkey {
    pub fn new(start_items: Vec<usize>, operation: Operation, test_div: usize, test_if_true: usize, test_if_false: usize) -> Self {
        Self { start_items, operation, test_div, test_if_true, test_if_false }
    }

    pub fn start_items(&self) -> &Vec<usize> {
        &self.start_items
    }

    pub fn test_div(&self) -> usize {
        self.test_div
    }
    pub fn test_if_true(&self) -> usize {
        self.test_if_true
    }
    pub fn test_if_false(&self) -> usize {
        self.test_if_false
    }
    pub fn operation(&self) -> &Operation {
        &self.operation
    }
}

pub enum Operation {
    MultiplyOld,
    AddOld,
    MultiplyValue(usize),
    AddValue(usize),
}