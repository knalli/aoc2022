use std::collections::HashMap;

use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let sum: i32 = resolve_priority_sum(scope, "puzzle1");
    write_solution(&scope, format!("sum = {}", sum).as_str());
}

fn resolve_priority_sum(scope: &PuzzleScope, puzzle: &str) -> i32 {
    let mut sum = 0;
    for line in io::read_puzzle_as_list(scope.day(), puzzle) {
        let compartment_size = line.len() / 2;
        let compartment1: Vec<char> = line.chars()
            .take(compartment_size)
            .collect();
        let compartment2: Vec<char> = line.chars()
            .skip(compartment_size)
            .collect();
        let overlaps = search_overlaps2(compartment1, compartment2);
        assert_eq!(1, overlaps.len());
        let priority = decode_item(overlaps[0]);
        sum += priority;
    }
    sum
}

fn count_chars(list: Vec<char>) -> HashMap<char, i32> {
    let mut counter: HashMap<char, i32> = HashMap::new();
    for ch in list {
        counter.entry(ch)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    counter
}

pub fn search_overlaps2(list1: Vec<char>, list2: Vec<char>) -> Vec<char> {
    let mut lists: Vec<Vec<char>> = Vec::new();
    lists.push(list1);
    lists.push(list2);
    return search_overlaps(lists);
}

pub fn search_overlaps(lists: Vec<Vec<char>>) -> Vec<char> {
    let required_size = lists.len() as i32;
    let mut counter: HashMap<char, i32> = HashMap::new();
    for list in lists {
        // count chars in this list, but treat as 1
        for (ch, count) in count_chars(list) {
            if count > 0 {
                counter.entry(ch)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
    }
    let mut result: Vec<char> = Vec::new();
    for (ch, count) in counter {
        if count >= required_size {
            result.push(ch);
        }
    }
    result
}

pub fn decode_item(ch: char) -> i32 {
    match ch {
        'a'..='z' => {
            (ch as i32) - 96 // ASCII, a is 97
        }
        'A'..='Z' => {
            (ch as i32) - 64 + 26 // ASCII, A is 65, plus offset (A=27)
        }
        _ => {
            0
        }
    }
}
