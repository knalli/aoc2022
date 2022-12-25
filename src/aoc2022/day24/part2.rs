use crate::aoc2022::day24::part1::{parse_input, print_map, shortest_path};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle1");

    println!("Initial");
    print_map(&input, 0, &input.start);

    // start -> goal
    let mut result = shortest_path(&input, &input.start, &input.end, 0).unwrap();
    // goal -> start
    result = shortest_path(&input, &input.end, &input.start, result).unwrap();
    // start -> goal
    result = shortest_path(&input, &input.start, &input.end, result).unwrap();
    write_solution(&scope, format!("result = {:?}", result).as_str());
}
