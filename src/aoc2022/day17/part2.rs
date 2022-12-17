use crate::aoc2022::day17::part1::{build_direction_generator, build_shape_generator, Chamber, parse_input, tower_height};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle1");
    let mut direction_generator = build_direction_generator(input);
    let mut chamber = Chamber::new(7, 0);
    let mut shape_generator = build_shape_generator();
    let size = tower_height(1_000_000_000_000, &mut chamber, &mut shape_generator, &mut direction_generator);
    write_solution(&scope, format!("height = {}", size).as_str());
}