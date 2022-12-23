use std::collections::HashMap;

use crate::aoc2022::day23::part1::{Direction, Elf, parse_input, play_round, print_map};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::point::Point2D;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let mut input = parse_input(scope, "puzzle1");

    let rounds = play_rounds_until_lms(&mut input);

    println!();
    println!("Final");
    print_map(&input);

    write_solution(&scope, format!("rounds = {}", rounds).as_str());
}

fn play_rounds_until_lms(map: &mut HashMap<Point2D, Elf>) -> usize {
    println!();
    println!("Initial Round", );
    print_map(map);

    let directions = vec!(
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    );

    let mut direction_offset = 0;

    let mut i = 1;
    loop {
        if play_round(map, &directions, direction_offset) == 0 {
            return i;
        }
        direction_offset = (direction_offset + 1) % directions.len();
        i = i + 1;
    }
}
