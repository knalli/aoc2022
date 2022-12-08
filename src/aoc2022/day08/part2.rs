use crate::aoc2022::day08::part1::{get_adjacent_edge_ranges, grid_extract_value, parse_input};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let grid = &mut parse_input(scope, "puzzle1");
    let height = grid.height();
    let width = grid.width();

    let get_adjacents_values = |x, y| get_adjacent_edge_ranges(height, width, x, y);

    let mut max_p: (i32, i32) = (0, 0);
    let mut max_score: i32 = 0;
    for py in 1..height - 1 {
        for px in 1..width - 1 {
            let pv = grid_extract_value(&grid.get(px, py));
            let mut range_scores: Vec<i32> = vec!();
            for adjacent_ranges in get_adjacents_values(px, py) {
                let mut score = 0;
                for (ax, ay) in adjacent_ranges {
                    let av = grid_extract_value(&grid.get(ax, ay));
                    score = score + 1;
                    if av >= pv {
                        break;
                    }
                }
                if score > 0 {
                    range_scores.push(score);
                }
            }
            let max = if range_scores.len() > 0 {
                range_scores.iter()
                    .fold(1, |x, y| x * y)
            } else {
                0
            };
            if max_score < max {
                max_score = max;
                max_p = (px, py);
            }
        }
    }

    write_solution(&scope, format!("max = ({}/{}) = {}", max_p.0, max_p.1, max_score).as_str());
}
