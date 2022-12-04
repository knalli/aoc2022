use std::borrow::Borrow;
use std::ops::Range;

use crate::aoc2022::day04::part1::parse_ranges;
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
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
        if range_partly_contains(&r.0, &r.1) || range_partly_contains(&r.1, &r.0) {
            //println!("+")
            count += 1;
        }
    }
    write_solution(&scope, format!("count = {}", count).as_str());
}

fn range_partly_contains(this: &Range<i32>, other: &Range<i32>) -> bool {
    this.contains(other.start.borrow()) || this.contains((other.end - 1).borrow())
}
