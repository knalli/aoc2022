use std::borrow::Borrow;
use std::ops::Range;
use std::str::Split;

use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let ranges = parse_ranges(scope, "puzzle1");
    let mut count = 0;
    for r in ranges {
        //println!("({}-{}),({}-{})", r.0.start, r.0.end, r.1.start, r.1.end);
        if range_full_contains(&r.0, &r.1) || range_full_contains(&r.1, &r.0) {
            //println!("+")
            count += 1;
        }
    }
    write_solution(&scope, format!("count = {}", count).as_str());
}

fn range_full_contains(this: &Range<i32>, other: &Range<i32>) -> bool {
    this.contains(other.start.borrow()) && this.contains((other.end - 1).borrow())
}

pub fn parse_ranges(scope: &PuzzleScope, puzzle: &str) -> Vec<(Range<i32>, Range<i32>)> {
    return io::read_puzzle_as_list(scope.day(), puzzle)
        .iter()
        .map(|line| {
            let mut split = line.split(',');
            let r1 = parse_range(&mut split);
            let r2 = parse_range(&mut split);
            (r1, r2)
        })
        .collect();
}

fn parse_range(split: &mut Split<char>) -> Range<i32> {
    let mut x = split.next().unwrap().split('-');
    Range {
        start: parse_int(x.next().unwrap()),
        end: parse_int(x.next().unwrap()) + 1, // off by one, exclusive
    }
}
