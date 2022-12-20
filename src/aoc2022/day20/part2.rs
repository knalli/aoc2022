use itertools::Itertools;

use crate::aoc2022::day20::part1::{build_list, do_mixing, parse_input, print_values};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

const DECRYPTION_KEY: isize = 811589153;

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle1");
    println!("INPUT");
    print_values(&input);

    let mut list = build_list(
        input.iter()
            .map(|v| v * DECRYPTION_KEY)
            .collect_vec()
    );
    for _ in 0..10 {
        do_mixing(&mut list);
    }

    // find 0
    let pos_i = list.iter().enumerate().find(|(_, (_, v))| v == &0).map(|(i, _)| i).unwrap();

    let len = list.len();
    let sum: isize = vec![1000, 2000, 3000].iter()
        .map(|x| {
            let p = (pos_i + x) % len;
            let v = list[p].1;
            println!("for {x} {v}");
            v
        })
        .sum();

    write_solution(&scope, format!("sum = {}", sum).as_str());
}