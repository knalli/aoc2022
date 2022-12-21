use std::collections::{HashMap, HashSet};
use std::fmt::Error;
use std::io::ErrorKind;
use std::str::FromStr;

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
    let mut map: HashMap<String, isize> = HashMap::new();
    solve(&input, &mut map).unwrap();
    write_solution(&scope, format!("number = {}", map.get("root").unwrap()).as_str());
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Vec<Monkey> {
    let Problem { monkeys } = io::read_puzzle_as_string(scope.day(), puzzle).parse().unwrap();
    monkeys
}

pub fn solve(monkeys: &[Monkey], map: &mut HashMap<String, isize>) -> Result<(), ErrorKind> {
    let mut workload: HashSet<&Monkey> = HashSet::from_iter(monkeys.iter());

    while !workload.is_empty() {
        let mut to_remove: HashSet<&Monkey> = HashSet::new();
        for monkey in &workload {
            if let Some(n) = &monkey.number {
                //println!("{} => N {}", monkey.name.as_str(), n);
                map.insert(monkey.name.clone(), *n);
                to_remove.insert(monkey);
                continue;
            }
            if let Some(d) = &monkey.dependency {
                match d {
                    Dependency::Add(a, b) => {
                        if map.contains_key(a) && map.contains_key(b) {
                            let v = map[a] + map[b];
                            //println!("{} => D {}", monkey.name.as_str(), v);
                            map.insert(monkey.name.clone(), v);
                            to_remove.insert(monkey);
                        }
                    }
                    Dependency::Sub(a, b) => {
                        if map.contains_key(a) && map.contains_key(b) {
                            let v = map[a] - map[b];
                            //println!("{} => D {}", monkey.name.as_str(), v);
                            map.insert(monkey.name.clone(), v);
                            to_remove.insert(monkey);
                        }
                    }
                    Dependency::Mul(a, b) => {
                        if map.contains_key(a) && map.contains_key(b) {
                            let v = map[a] * map[b];
                            //println!("{} => D {}", monkey.name.as_str(), v);
                            map.insert(monkey.name.clone(), v);
                            to_remove.insert(monkey);
                        }
                    }
                    Dependency::Div(a, b) => {
                        if map.contains_key(a) && map.contains_key(b) {
                            let v = map[a] / map[b];
                            //println!("{} => D {}", monkey.name.as_str(), v);
                            map.insert(monkey.name.clone(), v);
                            to_remove.insert(monkey);
                        }
                    }
                    Dependency::Eq(a, b) => {
                        if map.contains_key(a) && map.contains_key(b) {
                            if map[a] != map[b] {
                                return Err(ErrorKind::Other);
                            }
                            //println!("{} => D {}", monkey.name.as_str(), v);
                            map.insert(monkey.name.clone(), 0);
                            to_remove.insert(monkey);
                        }
                    }
                }
            }
        }
        to_remove.into_iter()
            .for_each(|m| {
                workload.remove(m);
            });
    }

    Ok(())
}

pub struct Problem {
    pub monkeys: Vec<Monkey>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkeys: Vec<Monkey> = vec!();
        for line in s.lines() {
            monkeys.push(line.parse()?);
        }
        Ok(Problem { monkeys })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Monkey {
    pub name: String,
    pub number: Option<isize>,
    pub dependency: Option<Dependency>,
}

impl Monkey {
    pub fn new(name: String, number: Option<isize>, dependency: Option<Dependency>) -> Self {
        Self { name, number, dependency }
    }
    pub fn new_by_value(name: String, number: isize) -> Self {
        Self { name, number: Some(number), dependency: None }
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(": ").collect_vec();
        let name = split[0].to_string();
        let number = match split[1].split(' ').count() {
            1 => Some(parse_int(split[1]) as isize),
            3 => None,
            _ => unreachable!(),
        };
        let dependency = match split[1].split(' ').count() {
            1 => None,
            3 => {
                let parts = split[1].split(' ').collect_vec();
                match parts[1] {
                    "+" => Some(Dependency::Add(parts[0].to_string(), parts[2].to_string())),
                    "-" => Some(Dependency::Sub(parts[0].to_string(), parts[2].to_string())),
                    "*" => Some(Dependency::Mul(parts[0].to_string(), parts[2].to_string())),
                    "/" => Some(Dependency::Div(parts[0].to_string(), parts[2].to_string())),
                    "=" => Some(Dependency::Eq(parts[0].to_string(), parts[2].to_string())),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };
        Ok(Monkey { name, number, dependency })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Dependency {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Eq(String, String),
}