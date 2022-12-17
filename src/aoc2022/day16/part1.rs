use std::collections::{HashMap, HashSet, VecDeque};

use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle0");
    let (max_pressure, _) = resolve_max_pressure(&input, 30);
    write_solution(&scope, format!("max = {:?}", max_pressure).as_str());
}

pub fn resolve_max_pressure(input: &Vec<Valve>, limit: i32) -> (i32, Vec<String>) {
    let map = build_node_map(input);
    let dists = build_node_dists(input, &map);

    // nodes of interest are only the one with a rate (+ the start)
    // lesson learnt: it does not make sense to go to a rate=0 node. only via.
    let nodes: HashSet<_> = input.iter()
        .filter(|v| v.id == "AA" || v.rate > 0)
        .map(|v| v.id.clone())
        .collect();

    let (max, mut path) = resolve_max_pressure0("AA", limit, &mut nodes.clone(), &dists, &map);
    path.reverse();
    (max, path)
}

pub fn resolve_max_pressure0(current: &str, minutes_left: i32, nodes: &mut HashSet<String>, dists: &HashMap<(String, String), i32>, map: &HashMap<String, &Valve>) -> (i32, Vec<String>) {

    // lesson learnt: don't retry an already open one. only via.
    nodes.remove(current);
    let mut best_path = vec!(); // reversed

    // look for all nodes which are useful to open(!)
    let mut max_gain = 0;
    nodes.iter().for_each(|next_id| {
        if let Some(dist) = dists.get(&(current.to_string(), next_id.clone())) {
            let remain = minutes_left - dist - 1; // dist = how many minutes it would take + 1 minute for open
            if remain > 0 {
                let next = map.get(next_id).unwrap();
                let (next_gain, next_path) = resolve_max_pressure0(next_id, remain, &mut nodes.clone(), dists, map);
                let gain = (remain * next.rate) + next_gain;
                if max_gain < gain {
                    //println!("gain={}", gain);
                    max_gain = max_gain.max(gain);
                    best_path = next_path.clone();
                    best_path.push(current.to_string()); // reversed
                }
            }
        }
    });
    (max_gain, best_path)
}

pub fn build_node_dists(input: &Vec<Valve>, map: &HashMap<String, &Valve>) -> HashMap<(String, String), i32> {
    let mut dists: HashMap<(String, String), i32> = HashMap::new();
    let mut seen = HashSet::new();
    input.iter().for_each(|from| {
        let mut q = VecDeque::new();
        q.push_back(from.id.clone());
        let mut d = 0;
        dists.insert((from.id.clone(), from.id.clone()), 0);
        while !q.is_empty() {
            d += 1;
            let mut next = vec!();
            while let Some(pos_via) = q.pop_front() {
                for pos_to in map.get(&pos_via).unwrap().options.clone() {
                    if !seen.contains(&(from.id.clone(), pos_to.clone())) {
                        next.push(pos_to.clone());
                        dists.insert((from.id.clone(), pos_to.clone()), d);
                        seen.insert((from.id.clone(), pos_to.clone()));
                    }
                }
            }
            while let Some(n) = next.pop() {
                q.push_back(n);
            }
        }
    });
    dists
}

pub fn build_node_map(input: &Vec<Valve>) -> HashMap<String, &Valve> {
    let mut map = HashMap::new();
    input.iter().for_each(|v| {
        map.insert(v.id.clone(), v);
    });
    map
}

pub struct Valve {
    pub id: String,
    pub rate: i32,
    pub options: Vec<String>,
}

impl Valve {
    pub fn new(id: String, rate: i32, options: Vec<String>) -> Self {
        Self { id, rate, options }
    }
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Vec<Valve> {
    io::read_puzzle_as_list(scope.day(), puzzle)
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split("; tunnel leads to valve ")
                .into_iter()
                .flat_map(|s| s.split("; tunnels lead to valves ").collect::<Vec<&str>>())
                .collect();
            let part1: Vec<&str> = parts[0].split(" has flow rate=").collect();
            let options: Vec<String> = parts[1].to_string()
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            Valve::new(
                part1[0].to_string()
                    .strip_prefix("Valve ").unwrap()
                    .to_string(),
                parse_int(part1[1]),
                options,
            )
        })
        .collect()
}