use crate::aoc2022::day14::part1::{build_carve, let_that_sink_in, parse_input, Pixel, print_carve};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let source = (500, 0);
    let mut carve = build_carve(parse_input(scope, "puzzle1"));
    carve.insert(source.clone(), Pixel::Source);

    let max_y: i32 = carve.iter()
        .map(|((_, y), _)| y.clone())
        .max()
        .unwrap() + 2;
    let max_x: i32 = carve.iter()
        .map(|((x, _), _)| x.clone())
        .max()
        .unwrap();
    for x in -max_x..2 * max_x {
        carve.insert((x, max_y), Pixel::Rock);
    }

    println!("START");
    print_carve(&carve);
    println!();

    // 2117
    let_that_sink_in(&mut carve, &source, max_y);

    println!("END");
    print_carve(&carve);
    println!();

    let sand_count = carve.iter()
        .filter(|(_, pixel)| {
            match pixel.clone() {
                Pixel::Rested => true,
                _ => false
            }
        })
        .count();

    write_solution(&scope, format!("count = {}", sand_count).as_str());
}
