use std::collections::HashSet;

use crate::aoc2022::day09::part1::parse_input;
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let instructions = parse_input(scope, "puzzle1");

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let start = (0, 0);
    let mut knots: Vec<(i32, i32)> = vec!();

    // head
    knots.push(start.clone());

    // tails
    for _ in 0..9 {
        knots.push(start.clone());
    }

    visited.insert(start.clone());

    let get_delta = |from: (i32, i32), to: (i32, i32)| {
        (to.0 - from.0, to.1 - from.1)
    };

    let delta_abs = |p: (i32, i32)| {
        (i32::abs(p.0), i32::abs(p.1))
    };

    for instruction in instructions {
        for _ in 0..instruction.1 {

            // step for head
            let mut head = knots.first().unwrap().clone();
            match instruction.0 {
                'U' => head = (head.0, head.1 + 1),
                'R' => head = (head.0 + 1, head.1),
                'D' => head = (head.0, head.1 - 1),
                'L' => head = (head.0 - 1, head.1),
                _ => ()
            }
            knots[0] = head;

            // tail?
            for tail_i in 1..knots.len() {
                let target = knots.get(tail_i - 1).unwrap().clone(); // formly head
                let mut tail = knots.get(tail_i).unwrap().clone();

                let delta = get_delta(tail.clone(), target.clone());
                let delta_abs = delta_abs(delta.clone());

                let delta_x = if delta.0 > 0 {
                    1
                } else if delta.0 < 0 {
                    -1
                } else {
                    0
                };
                let delta_y = if delta.1 > 0 {
                    1
                } else if delta.1 < 0 {
                    -1
                } else {
                    0
                };

                if delta_abs.0 == 1 && delta_abs.1 == 1 {
                    // ok
                } else if delta_abs.0 > 0 && delta_abs.1 > 0 {
                    tail = (tail.0 + delta_x, tail.1 + delta_y);
                } else if delta_abs.0 > 1 {
                    tail = (tail.0 + delta_x, tail.1);
                } else if delta_abs.1 > 1 {
                    tail = (tail.0, tail.1 + delta_y);
                }
                knots[tail_i] = tail;
            }

            visited.insert(knots.last().unwrap().clone());
        }
    }

    write_solution(&scope, format!("sum = {}", visited.len()).as_str());
}
