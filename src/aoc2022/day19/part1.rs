use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Error;
use std::ops::{Add, Sub};
use std::str::FromStr;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let Problem { blueprints } = io::read_puzzle_as_string(scope.day(), "puzzle1").parse().unwrap();
    //println!("{}", blueprints.iter().map(|s| s.to_string()).collect_vec().join("\n"));
    println!("Standard with BFS, some optimizations");
    let result = max_geodes(24, &blueprints);
    let ql: usize = result
        .iter()
        .map(|(i, max)| {
            let level = i * max;
            println!("Blueprint #{} with max geodes={} => quality_level={}", i, max, level);
            level
        })
        .sum();
    write_solution(&scope, format!("quality level = {:?}", ql).as_str());
}

#[derive(Eq, PartialEq, Clone)]
pub struct State {
    pub min_left: usize,
    pub ore_robots: usize,
    pub clay_robots: usize,
    pub obsidian_robots: usize,
    pub geode_robots: usize,
    pub materials: MaterialAmount,
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.materials.geode.cmp(&other.materials.geode)
    }
}

pub fn max_geodes(limit: usize, blueprint_list: &[Blueprint]) -> Vec<(usize, usize)> {
    let blueprints = blueprint_list.iter().map(|b| b.clone()).collect_vec();

    (0..blueprint_list.len())
        .into_par_iter()
        .map(|i| {
            let blueprint: Blueprint = blueprints[i].clone();
            //let mut q = VecDeque::new();
            let mut q = BinaryHeap::new();
            q.push(State {
                min_left: limit,
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
                materials: MaterialAmount { ore: 0, clay: 0, obsidian: 0, geode: 0 },
            });

            let mut max_geode: (usize, usize) = (0, 0);
            println!("[{:04}] Run...", i);

            while let Some(State { min_left, ore_robots, clay_robots, obsidian_robots, geode_robots, materials }) = q.pop() {
                if materials.geode > max_geode.0 || materials.geode == max_geode.0 && min_left > max_geode.1 {
                    let old = max_geode.clone();
                    max_geode = (materials.geode, min_left);
                    println!("[{:04}] {:?} => {:?}", i, old, max_geode);
                }

                if min_left == 0 {
                    continue;
                }

                // optimize branches which cannot reach the max technically
                // (building each step an additional geode robot, summed up using gaus)
                {
                    let n = min_left - 1;
                    if materials.geode + (geode_robots * min_left) + ((n * n + n) / 2) < max_geode.0 {
                        continue;
                    }
                }

                for (recipe_robot, recipe_materials) in blueprint.items.clone() {
                    let not_relevant = !match recipe_robot {
                        Robot::ORE => blueprint.max_costs.ore > ore_robots,
                        Robot::CLAY => blueprint.max_costs.clay > clay_robots,
                        Robot::OBSIDIAN => blueprint.max_costs.obsidian > obsidian_robots,
                        _ => true,
                    };
                    if not_relevant {
                        continue;
                    }

                    if materials.satisfies(recipe_materials.clone()) {
                        let mut l_ore_robots = ore_robots.clone();
                        let mut l_clay_robots = clay_robots.clone();
                        let mut l_obsidian_robots = obsidian_robots.clone();
                        let mut l_geode_robots = geode_robots.clone();

                        match recipe_robot {
                            Robot::ORE => l_ore_robots += 1,
                            Robot::CLAY => l_clay_robots += 1,
                            Robot::OBSIDIAN => l_obsidian_robots += 1,
                            Robot::GEODE => l_geode_robots += 1,
                        }

                        q.push(State {
                            min_left: min_left - 1,
                            ore_robots: l_ore_robots,
                            clay_robots: l_clay_robots,
                            obsidian_robots: l_obsidian_robots,
                            geode_robots: l_geode_robots,
                            materials: MaterialAmount {
                                ore: materials.ore + ore_robots.clone(),
                                clay: materials.clay + clay_robots.clone(),
                                obsidian: materials.obsidian + obsidian_robots.clone(),
                                geode: materials.geode + geode_robots.clone(),
                            }.sub(recipe_materials.clone()),
                        });
                    }
                }

                q.push(State {
                    min_left: min_left.clone() - 1,
                    ore_robots: ore_robots.clone(),
                    clay_robots: clay_robots.clone(),
                    obsidian_robots: obsidian_robots.clone(),
                    geode_robots: geode_robots.clone(),
                    materials: MaterialAmount {
                        ore: materials.ore + ore_robots.clone(),
                        clay: materials.clay + clay_robots.clone(),
                        obsidian: materials.obsidian + obsidian_robots.clone(),
                        geode: materials.geode + geode_robots.clone(),
                    },
                });
            }

            println!("[{:04}] Found for blueprint: {:?}", i, max_geode);
            (i + 1, max_geode.0)
        })
        .collect()
}

pub struct Problem {
    pub blueprints: Vec<Blueprint>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blueprints: Vec<Blueprint> = vec!();

        for line in s.lines() {
            if line.is_empty() {
                continue;
            }

            if !line.starts_with("Blueprint ") {
                panic!("invalid line");
            }

            let mut items = HashMap::new();

            let parts = line.split(": ")
                .collect_vec();
            for s in parts[1].split(". ").filter(|s| s.starts_with("Each")).collect_vec() {
                let pp = s.split(" ").collect_vec();
                let robot = match pp[1] {
                    "ore" => {
                        Robot::ORE
                    }
                    "clay" => {
                        Robot::CLAY
                    }
                    "obsidian" => {
                        Robot::OBSIDIAN
                    }
                    "geode" => {
                        Robot::GEODE
                    }
                    _ => panic!("invalid robot type")
                };
                let mut materials = [0usize; 4];
                for i in (4..pp.len()).step_by(3) {
                    let amount: usize = parse_int(pp[i]) as usize;
                    match pp[i + 1].strip_suffix(".").unwrap_or(pp[i + 1]) {
                        "ore" => {
                            materials[0] = amount
                        }
                        "clay" => {
                            materials[1] = amount
                        }
                        "obsidian" => {
                            materials[2] = amount
                        }
                        "geode" => {
                            materials[3] = amount
                        }
                        _ => panic!("invalid robot type")
                    };
                }
                items.insert(robot, MaterialAmount { ore: materials[0], clay: materials[1], obsidian: materials[2], geode: materials[3] });
            }
            blueprints.push(Blueprint::new_with_items(items));
        }
        Ok(Problem { blueprints })
    }
}

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub items: HashMap<Robot, MaterialAmount>,
    pub max_costs: MaterialAmount,
}

impl ToString for Blueprint {
    fn to_string(&self) -> String {
        let mut result = "".to_owned();
        result += "Blueprint:\n";
        for robot in vec![Robot::ORE, Robot::CLAY, Robot::OBSIDIAN, Robot::GEODE] {
            if let Some(costs) = self.items.get(&robot) {
                result += format!("  Each {:?} robot costs {} {}, {} {}, {} {} and {} {}\n",
                                  robot,
                                  costs.ore, "ore",
                                  costs.clay, "clay",
                                  costs.obsidian, "obsidian",
                                  costs.geode, "geode"
                ).as_str();
            }
        }
        result
    }
}

impl Blueprint {
    pub fn new_with_items(items: HashMap<Robot, MaterialAmount>) -> Self {
        let max_costs = MaterialAmount {
            ore: items.iter().map(|(_, m)| m.ore).max().unwrap(),
            clay: items.iter().map(|(_, m)| m.clay).max().unwrap(),
            obsidian: items.iter().map(|(_, m)| m.obsidian).max().unwrap(),
            geode: items.iter().map(|(_, m)| m.geode).max().unwrap(),
        };
        Self { items, max_costs }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Robot {
    ORE,
    CLAY,
    OBSIDIAN,
    GEODE,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct MaterialAmount {
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub geode: usize,
}

impl MaterialAmount {
    pub fn satisfies(&self, other: Self) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian && self.geode >= other.geode
    }
}

impl Add for MaterialAmount {
    type Output = MaterialAmount;

    fn add(self, rhs: Self) -> Self::Output {
        MaterialAmount {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub for MaterialAmount {
    type Output = MaterialAmount;

    fn sub(self, rhs: Self) -> Self::Output {
        MaterialAmount {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}