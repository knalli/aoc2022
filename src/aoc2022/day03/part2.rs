use crate::aoc2022::day03::part1::{decode_item, search_overlaps};
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

    let lines = io::read_puzzle_as_list(scope.day(), puzzle);
    let mut groups: Vec<Vec<String>> = Vec::new();
    for g_offset in 0..(lines.len() / 3) {
        let mut group: Vec<String> = Vec::new();
        for i_offset in 0..3 {
            group.push(lines.get(3*g_offset + i_offset).unwrap().to_string());
        }
        groups.push(group);
    }

    for group in groups {
        let list: Vec<Vec<char>> = group.iter()
            .map(|g| g.chars().collect())
            .collect();
        let overlaps = search_overlaps(list);
        assert_eq!(1, overlaps.len());
        let priority = decode_item(overlaps[0]);
        sum += priority;
    }
    sum
}
