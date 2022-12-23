use std::collections::HashMap;

use itertools::Itertools;

use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::point::Point2D;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let mut input = parse_input(scope, "puzzle1");
    play_rounds(&mut input, 10);

    println!();
    println!("Final");
    print_map(&input);

    let sum = count_empty(&input);
    write_solution(&scope, format!("sum = {}", sum).as_str());
}

fn play_rounds(map: &mut HashMap<Point2D, Elf>, round_limit: usize) {
    println!();
    println!("Initial Round", );
    print_map(map);

    let directions = vec!(
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    );

    let mut direction_offset = 0;

    for _ in 1..=round_limit {
        play_round(map, &directions, direction_offset);

        direction_offset = (direction_offset + 1) % directions.len();

        /*
        println!();
        println!("Round {}", i);
        print_map(map);
         */
    }
}

pub fn play_round(map: &mut HashMap<Point2D, Elf>, directions: &Vec<Direction>, direction_offset: usize) -> usize {
    let mut current_positions = vec!();
    for pos in map.keys().clone() {
        current_positions.push(pos.clone());
    }

    // compute target "propose moving"
    for pos in &current_positions {
        let mut elf = map.get_mut(&pos).unwrap();

        elf.target = None;
        if pos.adjacents().iter().all(|a| !current_positions.contains(a)) {
            continue;
        }

        let mut dirs = vec!();
        for d in &directions[direction_offset..] {
            dirs.push(d);
        }
        for d in &directions[..direction_offset] {
            dirs.push(d);
        }
        for opt_dir in dirs {
            match opt_dir {
                Direction::North => {
                    let list = vec![pos.top().left(), pos.top(), pos.top().right()];
                    if list.iter().all(|a| !current_positions.contains(a)) {
                        elf.target = Some(pos.top());
                        //println!("{:?}", opt_dir);
                        break;
                    }
                }
                Direction::South => {
                    let list = vec![pos.bottom().left(), pos.bottom(), pos.bottom().right()];
                    if list.iter().all(|a| !current_positions.contains(a)) {
                        elf.target = Some(pos.bottom());
                        //println!("{:?}", opt_dir);
                        break;
                    }
                }
                Direction::West => {
                    let list = vec![pos.left().top(), pos.left(), pos.left().bottom()];
                    if list.iter().all(|a| !current_positions.contains(a)) {
                        elf.target = Some(pos.left());
                        //println!("{:?}", opt_dir);
                        break;
                    }
                }
                Direction::East => {
                    let list = vec![pos.right().top(), pos.right(), pos.right().bottom()];
                    if list.iter().all(|a| !current_positions.contains(a)) {
                        elf.target = Some(pos.right());
                        //println!("{:?}", opt_dir);
                        break;
                    }
                }
            }
        }
        //println!("{:?} => {:?}", pos, elf.target);
    }

    let movable = map.iter()
        // ignore staying ones
        .filter(|(_, e)| e.target.is_some())
        // pos->target
        .map(|(p, e)| {
            (p.clone(), e.target.unwrap().clone())
        })
        // group by target
        .into_group_map_by(|(_, t)| *t)
        .into_iter()
        // only list-len == 1
        .filter(|(_, v)| v.len() == 1)
        // re-map back pos
        .flat_map(|(_, v)| v.into_iter())
        .map(|(p, _)| p)
        .collect_vec();

    let cnt = movable.len();
    for pos in movable {
        let elf = map.remove(&pos).unwrap();
        map.insert(elf.target.unwrap(), elf);
    }

    cnt
}

pub fn print_map(map: &HashMap<Point2D, Elf>) {
    let min_x = map.keys().map(|p| p.x()).min().unwrap() - 1;
    let max_x = map.keys().map(|p| p.x()).max().unwrap() + 1;
    let min_y = map.keys().map(|p| p.y()).min().unwrap() - 1;
    let max_y = map.keys().map(|p| p.y()).max().unwrap() + 1;

    let str0 = (min_x..=max_x).into_iter()
        .map(|d| d % 10)
        .map(|d| format!("{}", d))
        .join("");
    println!("{}", str0);
    for y in min_y..=max_y {
        let str = (min_x..=max_x).into_iter()
            .map(|x| {
                if map.contains_key(&Point2D::create(x, y)) {
                    "#"
                } else {
                    "."
                }
            })
            .join("");
        println!("{}", str)
    }
}

fn count_empty(map: &HashMap<Point2D, Elf>) -> usize {
    let min_x = map.keys().map(|p| p.x()).min().unwrap();
    let max_x = map.keys().map(|p| p.x()).max().unwrap();
    let min_y = map.keys().map(|p| p.y()).min().unwrap();
    let max_y = map.keys().map(|p| p.y()).max().unwrap();

    (min_y..=max_y).into_iter()
        .map(|y| {
            (min_x..=max_x).into_iter()
                .filter(|x| {
                    !map.contains_key(&Point2D::create(*x, y))
                })
                .count()
        })
        .sum()
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> HashMap<Point2D, Elf> {
    let mut map = HashMap::new();
    io::read_puzzle_as_list(scope.day(), puzzle).iter()
        .enumerate()
        .for_each(|(i, line)| {
            line.chars().into_iter()
                .enumerate()
                .for_each(|(j, c)| {
                    match c {
                        '#' => { map.insert(Point2D::create(j as i32, i as i32), Elf::new()); }
                        _ => (),
                    }
                })
        });
    map
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Elf {
    target: Option<Point2D>,
}

impl Elf {
    pub fn new() -> Self {
        Self { target: None }
    }
}