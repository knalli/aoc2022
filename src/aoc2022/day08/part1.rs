use std::collections::VecDeque;

use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::grid::{Cell, DynGrid2D, Grid2D, GridValue};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let grid = &mut parse_input(scope, "puzzle1");
    let height = grid.height();
    let width = grid.width();

    let shadow_grid = &mut DynGrid2D::create(height, width);
    let mut ss: VecDeque<(i32, i32)> = VecDeque::new();

    // prepare edges (visible)
    for x in vec!(0, width - 1) {
        for y in 0..height {
            shadow_grid.set((x, y), Cell::Bool(true));
        }
    }
    for x in 0..width {
        for y in vec!(0, height - 1) {
            shadow_grid.set((x, y), Cell::Bool(true));
        }
    }

    let get_adjacents = |x, y| get_adjacents(height, width, x, y);

    let get_adjacents_values = |x, y| get_adjacent_edge_ranges(height, width, x, y);

    for x in vec!(0, width - 1) {
        for y in 0..height {
            for a in get_adjacents(x, y) {
                if !dyn_grid_is_visited(shadow_grid, a.clone()) {
                    ss.push_back(a);
                }
            }
        }
    }
    for x in 0..width {
        for y in vec!(0, height - 1) {
            for a in get_adjacents(x, y) {
                if !dyn_grid_is_visited(shadow_grid, a.clone()) {
                    ss.push_back(a);
                }
            }
        }
    }
    while !ss.is_empty() {
        let (px, py) = ss.pop_front().unwrap();

        // avoid loop
        if dyn_grid_is_visited(shadow_grid, (px, py)) {
            continue;
        }

        let pv = grid_extract_value(&grid.get(px, py));
        let mut p_visible = false;
        for a_list in get_adjacents_values(px, py) {
            let mut a_lower = true;
            for (ax, ay) in a_list {
                let av = grid_extract_value(&grid.get(ax, ay));
                if pv <= av {
                    a_lower = false;
                    break;
                }
            }
            if a_lower {
                p_visible = true;
                break;
            }
        }
        shadow_grid.set((px, py), Cell::Bool(p_visible));

        // append next
        for (ax, ay) in get_adjacents(px, py) {
            if !dyn_grid_is_visited(shadow_grid, (ax, ay)) {
                ss.push_back((ax, ay));
            }
        }
    }

    let mut count = 0;
    grid.each().iter().for_each(|(p, _)| {
        let x = p.x();
        let y = p.y();
        if dyn_grid_is_visible(shadow_grid, (x, y)) {
            count = count + 1;
        }
    });

    write_solution(&scope, format!("sum = {}", count).as_str());
}

pub fn grid_extract_value(v: &Option<&GridValue>) -> i32 {
    if let Some(gv) = v {
        match gv {
            GridValue::Int(i) => *i,
            _ => 0
        }
    } else {
        0
    }
}

pub fn get_adjacents(height: i32, width: i32, x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = vec!();
    if x > 0 {
        result.push((x - 1, y));
    }
    if x < width - 1 {
        result.push((x + 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if y < height - 1 {
        result.push((x, y + 1));
    }
    result
}

pub fn get_adjacent_edge_ranges(height: i32, width: i32, x: i32, y: i32) -> Vec<Vec<(i32, i32)>> {
    let mut result: Vec<Vec<(i32, i32)>> = vec!();
    if x > 0 {
        let mut sr: Vec<(i32, i32)> = vec!();
        for ax in 0..x {
            sr.push((ax, y));
        }
        sr.reverse();
        result.push(sr);
    }
    if x < width - 1 {
        let mut sr: Vec<(i32, i32)> = vec!();
        for ax in x + 1..width{
            sr.push((ax, y));
        }
        result.push(sr);
    }
    if y > 0 {
        let mut sr: Vec<(i32, i32)> = vec!();
        for ay in 0..y {
            sr.push((x, ay));
        }
        sr.reverse();
        result.push(sr);
    }
    if y < height - 1 {
        let mut sr: Vec<(i32, i32)> = vec!();
        for ay in y + 1..height {
            sr.push((x, ay));
        }
        result.push(sr);
    }
    result
}

pub fn dyn_grid_is_visited(grid: &mut DynGrid2D, (x, y): (i32, i32)) -> bool {
    match grid.get(&(x, y)) {
        Some(_) => true,
        None => false
    }
}

pub fn dyn_grid_is_visible(grid: &mut DynGrid2D, (x, y): (i32, i32)) -> bool {
    match grid.get(&(x, y)) {
        Some(Cell::Bool(v)) => v,
        _ => false
    }
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Grid2D {
    let lines = io::read_puzzle_as_list(scope.day(), puzzle);
    let mut grid = Grid2D::create(lines.len() as i32, lines.iter().next().unwrap().len() as i32);

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            let i = c as i32 - 48;
            grid.set(x, y, GridValue::Int(i));
            x = x + 1;
        }
        y = y + 1;
    }

    grid
}
