use std::collections::HashMap;

use crate::aoc2022::day11::part1::{Game, parse_input, play_game};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
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

    let rounds = 10_000;

    // product of all divisors
    let divisor: usize = game.monkeys.iter()
        .map(|m| m.test_div())
        .fold(1, |a, b| a * b);
    println!("Use divisors {}", divisor);

    play_game(&mut game, rounds, false, rounds + 1, 1000, 1, divisor);
    println!();

    let mut mm: Vec<(usize, usize)> = vec!();
    game.inspections.iter().for_each(|(monkey_id, count)| mm.push((*count, *monkey_id)));
    mm.sort_by(|a, b| b.0.cmp(&a.0));
    let result = mm.get(0).unwrap().0 * mm.get(1).unwrap().0;

    write_solution(&scope, format!("result = {}", result).as_str());
}
