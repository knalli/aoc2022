use std::cmp::Ordering;

use itertools::Itertools;

use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle1");
    println!("INPUT");
    print_values(&input);

    let mut list = build_list(input);
    do_mixing(&mut list);

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

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Vec<isize> {
    io::read_puzzle_as_list(scope.day(), puzzle)
        .into_iter()
        .map(|s| parse_int(s.as_str()) as isize)
        .collect_vec()
}

pub fn build_list(input: Vec<isize>) -> Vec<(usize, isize)> {
    // Lesson #0 simple use a tupel. so much easier than 3 offset-hashmaps.. 🫠
    input.iter()
        .enumerate()
        .map(|(a, b)| (a, *b))
        .collect_vec()
}

// after a third rewrite, i looked for better solutions: https://github.com/ephemient/aoc2022/blob/main/rs/src/day20.rs
pub fn do_mixing(list: &mut Vec<(usize, isize)>) {
    for n in 0..list.len() {
        // find by original offset (n)
        let (n, &item) = list.iter().enumerate().find(|(_, (o, _))| o == &n).unwrap();
        // Lesson #1 rem_euclid better than standard modulo (avoid the bound checks)
        // Lesson #2 mod-1 because of shifting (removing).. that explains my off-by-one errors a lot
        let t = (n as isize + item.1).rem_euclid(list.len() as isize - 1) as usize;
        // Lesson #3 aware of slice::copy_within
        match n.cmp(&t) {
            Ordering::Less => list.copy_within(n + 1..t + 1, n),
            Ordering::Equal => (),
            Ordering::Greater => list.copy_within(t..n, t + 1),
        }
        list[t] = item;
    }
}

pub fn print_values(values: &[isize]) {
    println!("{}", values
        .iter()
        .map(|i| format!("{}", i))
        .join(", "));
    println!();
}
