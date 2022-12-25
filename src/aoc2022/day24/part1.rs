use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::usize;

use anyhow::{Error, Result};
use itertools::Itertools;
use rayon::prelude::*;

use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::point::Point2D;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle1");

    println!("Initial");
    print_map(&input, 0, &input.start);

    let result = shortest_path(&input, &input.start, &input.end, 0).unwrap();
    write_solution(&scope, format!("result = {:?}", result).as_str());
}

#[derive(Debug, Eq, PartialEq)]
struct State(Point2D, usize, usize);

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        let by_min = self.2.cmp(&other.2);
        let by_dist = self.1.cmp(&other.1);
        by_min.then(by_dist.reverse())
    }
}

pub fn print_map(input: &Input, timing_level: usize, current: &Point2D) {
    let blizzards = get_blizzard_state(input, timing_level);
    for y in -1..=input.height as i32 {
        if y == -1 || y == input.height as i32 {
            println!("{}", "#".repeat(input.width + 2));
        } else {
            let mut str_items: Vec<String> = vec!();
            for x in 0..input.width {
                let p = Point2D::create(x as i32, y);
                if p.eq(current) {
                    str_items.push("E".to_owned());
                } else {
                    let filtered = blizzards.iter().filter(|(b_pos, _)| b_pos.eq(&p))
                        .map(|(_, o)| o)
                        .collect_vec();
                    match filtered.len() {
                        0 => str_items.push(".".to_owned()),
                        1 => {
                            str_items.push(match filtered[0].clone() {
                                Point2D { x: 1, y: 0 } => ">",
                                Point2D { x: -1, y: 0 } => "<",
                                Point2D { x: 0, y: -1 } => "^",
                                Point2D { x: 0, y: 1 } => "v",
                                _ => unreachable!(),
                            }.to_owned());
                        }
                        v => {
                            str_items.push(v.to_string());
                        }
                    }
                }
            }
            println!("#{}#", str_items.join(""));
        }
    }
}

pub fn shortest_path(input: &Input, start: &Point2D, goal: &Point2D, time_offset: usize) -> Result<usize> {
    let max_timing_levels = input.height * input.width;

    println!("Warming up...");

    // prepare: manhatten distances from each point to end (for each timing_level)
    let mut mhd = HashMap::new();
    for x in 0..input.width {
        for y in 0..input.width {
            let p = Point2D::create(x as i32, y as i32);
            mhd.insert(p, p.manhatten_distance(&goal));
        }
    }
    mhd.insert(start.clone(), start.clone().manhatten_distance(&goal));
    mhd.insert(goal.clone(), 0);

    // prepare: blizzard positions for each timing_level
    // layout of positions will repeat after H*W
    let mut ti_blizzard_positions = HashMap::new();
    {
        let items: Vec<_> = (0..max_timing_levels).into_par_iter()
            .map(|i| (i, build_blizzard_state_set(input, i)))
            .collect();
        items.into_iter().for_each(|(i, hs)| {
            ti_blizzard_positions.insert(i, hs);
        });
    }

    // is_in_grid fn utility
    let range_x = 0..input.width as i32;
    let range_y = 0..input.height as i32;
    let is_in_grid = |p: &Point2D| -> bool {
        range_x.contains(&p.x()) && range_y.contains(&p.y())
    };

    println!("Warming completed!");
    println!();

    let mut q = VecDeque::new();
    let mut visited: HashSet<(usize, Point2D)> = HashSet::new();
    q.push_back(State(start.clone(), 0, time_offset));

    let mut minimum: Option<usize> = None;

    while let Some(State(pos, dist, minute)) = q.pop_front() {
        /*
        println!();
        println!("Minute {minute}");
        print_map(&input, minute % (input.height * input.width), &pos);
         */

        if &pos == goal {
            match minimum {
                Some(min_minute) => {
                    if minute < min_minute {
                        println!("Found better minimum, {min_minute} => {minute} ∂={dist}");
                        minimum = Some(minute);
                    }
                }
                None => {
                    println!("Found first minimum at {minute} ∂={dist}");
                    minimum = Some(minute);
                }
            }
            continue;
        }

        let next_minute = minute + 1;

        let mut next_candidates = vec!();
        vec!((1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)).into_iter()
            .for_each(|offset| {
                next_candidates.push(Point2D::create(pos.x() + offset.0, pos.y() + offset.1));
            });

        for a in next_candidates {
            // valid in this grid?
            if !is_in_grid(&a) && !goal.eq(&a) && !start.eq(&a) {
                continue;
            }
            // not blocked by a blizzard?
            if ti_blizzard_positions[&((next_minute) % max_timing_levels)].contains(&a) {
                continue;
            }
            // reachable within less than the known minimum?
            match minimum {
                Some(min_minute) => {
                    if next_minute + mhd[&a] > min_minute {
                        continue;
                    }
                }
                None => {}
            }
            // already visited
            if visited.contains(&(next_minute, a.clone())) {
                continue;
            }

            q.push_back(State(a, dist + 1, next_minute));
            visited.insert((next_minute, a.clone()));
        }
    }

    // position within the block, so we need one more
    match minimum {
        Some(minute) => {
            Ok(minute as usize)
        }
        None => {
            Err(Error::msg("no minimum found"))
        }
    }
}

fn get_blizzard_state(input: &Input, n: usize) -> Vec<(Point2D, Point2D)> {
    input.blizzards.iter()
        .map(|blizzard| {
            let mut p = blizzard.start.clone();
            for _ in 0..n {
                p = p + blizzard.offset;
            }
            let px = p.x().rem_euclid(input.width as i32);
            let py = p.y().rem_euclid(input.height as i32);
            (Point2D::create(
                px,
                py,
            ), blizzard.offset.clone())
        })
        .collect_vec()
}

fn build_blizzard_state_set(input: &Input, n: usize) -> HashSet<Point2D> {
    let mut hs = HashSet::new();
    input.blizzards.iter()
        .for_each(|blizzard| {
            let p = Point2D::create(
                (blizzard.start.x + blizzard.offset.x * (n as i32)).rem_euclid(input.width as i32),
                (blizzard.start.y + blizzard.offset.y * (n as i32)).rem_euclid(input.height as i32),
            );
            assert!(0 <= p.x && p.x < input.width as i32, "x out of range: {:?}", p);
            assert!(0 <= p.y && p.y < input.height as i32, "y out of range: {:?}", p);
            hs.insert(p);
        });
    hs
}

pub struct Input {
    pub height: usize,
    pub width: usize,
    pub start: Point2D,
    pub end: Point2D,
    pub blizzards: Vec<Blizzard>,
}

pub struct Blizzard {
    pub start: Point2D,
    pub offset: Point2D,
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Input {
    let lines = io::read_puzzle_as_list(scope.day(), puzzle);
    let height = lines.len() - 2;
    let width = lines.last().unwrap().len() - 2;
    let start = Point2D::create(
        lines[0].chars().enumerate().find(|(_, c)| c == &'.').map(|(i, _)| i).unwrap() as i32 - 1,
        -1,
    );
    let end = Point2D::create(
        lines.last().unwrap().chars().enumerate().find(|(_, c)| c == &'.').map(|(i, _)| i).unwrap() as i32 - 1,
        height as i32, // +1
    );

    let blizzards = lines.iter().enumerate().filter(|(i, _)| (1..=height).contains(i))
        .flat_map(|(j, line)| {
            let y = j.clone() as i32 - 1;
            line.chars().enumerate()
                .filter(|(j, _)| (1..=width).contains(&j))
                .flat_map(move |(i, c)| {
                    let x = i.clone() as i32 - 1;
                    match c {
                        '>' => vec!(Blizzard { start: Point2D::create(x, y), offset: Point2D::create(1, 0) }),
                        '<' => vec!(Blizzard { start: Point2D::create(x, y), offset: Point2D::create(-1, 0) }),
                        'v' => vec!(Blizzard { start: Point2D::create(x, y), offset: Point2D::create(0, 1) }),
                        '^' => vec!(Blizzard { start: Point2D::create(x, y), offset: Point2D::create(0, -1) }),
                        '.' => vec!(),
                        _ => unreachable!(),
                    }
                })
        })
        .collect_vec();
    Input {
        height,
        width,
        start,
        end,
        blizzards,
    }
}
