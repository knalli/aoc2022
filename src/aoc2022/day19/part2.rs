use std::ops::Sub;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::aoc2022::day19::beam_search;
use crate::aoc2022::day19::part1::{Blueprint, MaterialAmount, Problem, Robot, State};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let Problem { blueprints } = io::read_puzzle_as_string(scope.day(), "puzzle1").parse().unwrap();
    //println!("{}", blueprints.iter().map(|s| s.to_string()).collect_vec().join("\n"));
    let result = max_geodes_beam(32, &blueprints[0..3], 20000); // 10k too less, 20k enough
    let product: usize = result
        .iter()
        .map(|(_, max)| max)
        .product();
    write_solution(&scope, format!("product = {:?}", product).as_str());
}

struct Node<'a> {
    state: State,
    blueprint: &'a Blueprint,
}

impl<'a> Node<'a> {
    pub fn new(state: State, blueprint: &'a Blueprint) -> Self {
        Self { state, blueprint }
    }
}


impl<'a> beam_search::Node for Node<'a> {
    // compute all possible next states
    fn children(&self) -> Vec<Self> where Self: Sized {
        let mut children = vec!();

        if self.state.min_left == 0 {
            return children;
        }

        for r in vec![Robot::GEODE, Robot::OBSIDIAN, Robot::CLAY, Robot::ORE] {
            let required_materials = self.blueprint.items.get(&r).unwrap();
            if self.state.materials.satisfies(required_materials.clone()) {
                let mut l_ore_robots = self.state.ore_robots.clone();
                let mut l_clay_robots = self.state.clay_robots.clone();
                let mut l_obsidian_robots = self.state.obsidian_robots.clone();
                let mut l_geode_robots = self.state.geode_robots.clone();
                match r {
                    Robot::ORE => l_ore_robots += 1,
                    Robot::CLAY => l_clay_robots += 1,
                    Robot::OBSIDIAN => l_obsidian_robots += 1,
                    Robot::GEODE => l_geode_robots += 1,
                }

                children.push(Node::new(
                    State {
                        min_left: self.state.min_left - 1,
                        ore_robots: l_ore_robots,
                        clay_robots: l_clay_robots,
                        obsidian_robots: l_obsidian_robots,
                        geode_robots: l_geode_robots,
                        materials: MaterialAmount {
                            ore: self.state.materials.ore + self.state.ore_robots.clone(),
                            clay: self.state.materials.clay + self.state.clay_robots.clone(),
                            obsidian: self.state.materials.obsidian + self.state.obsidian_robots.clone(),
                            geode: self.state.materials.geode + self.state.geode_robots.clone(),
                        }.sub(required_materials.clone()),
                    },
                    self.blueprint,
                ));
            }
        }

        // Otherwise
        children.push(Node {
            state: State {
                min_left: self.state.min_left - 1,
                ore_robots: self.state.ore_robots,
                clay_robots: self.state.clay_robots,
                obsidian_robots: self.state.obsidian_robots,
                geode_robots: self.state.geode_robots,
                materials: MaterialAmount {
                    ore: self.state.materials.ore + self.state.ore_robots.clone(),
                    clay: self.state.materials.clay + self.state.clay_robots.clone(),
                    obsidian: self.state.materials.obsidian + self.state.obsidian_robots.clone(),
                    geode: self.state.materials.geode + self.state.geode_robots.clone(),
                },
            },
            blueprint: self.blueprint,
        });

        children
    }

    // based on the goal (geode amount) compute the current possible outcome
    // internal priority by robot type
    fn score(&self) -> usize {
        let mut state = self.state.clone();

        for _ in 0..self.state.min_left {
            if state.min_left == 0 {
                break;
            }
            for r in vec![Robot::GEODE, Robot::OBSIDIAN, Robot::CLAY, Robot::ORE] {
                if state.min_left == 0 {
                    continue;
                }
                let required_materials = self.blueprint.items.get(&r).unwrap();
                if state.materials.satisfies(required_materials.clone()) {
                    let mut l_ore_robots = state.ore_robots.clone();
                    let mut l_clay_robots = state.clay_robots.clone();
                    let mut l_obsidian_robots = state.obsidian_robots.clone();
                    let mut l_geode_robots = state.geode_robots.clone();
                    match r {
                        Robot::ORE => l_ore_robots += 1,
                        Robot::CLAY => l_clay_robots += 1,
                        Robot::OBSIDIAN => l_obsidian_robots += 1,
                        Robot::GEODE => l_geode_robots += 1,
                    }
                    state = State {
                        min_left: state.min_left - 1,
                        ore_robots: l_ore_robots,
                        clay_robots: l_clay_robots,
                        obsidian_robots: l_obsidian_robots,
                        geode_robots: l_geode_robots,
                        materials: MaterialAmount {
                            ore: state.materials.ore + state.ore_robots.clone(),
                            clay: state.materials.clay + state.clay_robots.clone(),
                            obsidian: state.materials.obsidian + state.obsidian_robots.clone(),
                            geode: state.materials.geode + state.geode_robots.clone(),
                        }.sub(required_materials.clone()),
                    };
                    continue;
                }
                if state.min_left == 0 {
                    continue;
                }
                // otherwise
                state = State {
                    min_left: state.min_left - 1,
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots,
                    materials: MaterialAmount {
                        ore: state.materials.ore + state.ore_robots.clone(),
                        clay: state.materials.clay + state.clay_robots.clone(),
                        obsidian: state.materials.obsidian + state.obsidian_robots.clone(),
                        geode: state.materials.geode + state.geode_robots.clone(),
                    },
                };
            }
        }
        state.materials.geode
    }

    // current geode amount
    fn real_score(&self) -> usize {
        self.state.materials.geode
    }
}

pub fn max_geodes_beam(limit: usize, blueprint_list: &[Blueprint], width: usize) -> Vec<(usize, usize)> {
    let blueprints = blueprint_list.iter().map(|b| b.clone()).collect_vec();

    (0..blueprint_list.len())
        .into_par_iter()
        .map(|i| {
            let blueprint: Blueprint = blueprints[i].clone();

            let source = Node::new(
                State {
                    min_left: limit,
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                    materials: MaterialAmount { ore: 0, clay: 0, obsidian: 0, geode: 0 },
                },
                &blueprint,
            );

            (i + 1, beam_search::beam_search(source, width))
        })
        .collect()
}