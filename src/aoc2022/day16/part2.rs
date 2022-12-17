use std::collections::HashSet;

use itertools::Itertools;

use crate::aoc2022::day16::part1::{build_node_dists, build_node_map, parse_input, resolve_max_pressure0, Valve};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle1");
    let max_pressure = resolve_with_an_elephant_in_the_room(&input);
    write_solution(&scope, format!("max = {:?}", max_pressure).as_str());
}

fn resolve_with_an_elephant_in_the_room(input: &Vec<Valve>) -> i32 {
    let map = build_node_map(input);
    let dists = build_node_dists(input, &map);

    let nodes: HashSet<_> = input.iter()
        .filter(|v| v.rate > 0)
        .map(|v| v.id.clone())
        .collect();

    let mut max = 0;
    for nodes1 in nodes.iter().permutations(nodes.len() / 2)
        .unique()
        .map(|list| {
            let set: HashSet<String> = list.iter()
                .map(|s| s.to_string())
                .collect();
            set
        }) {
        let (max1, _) = resolve_max_pressure0("AA", 26, &mut nodes1.clone(), &dists, &map);

        let nodes2: HashSet<_> = input.iter()
            .filter(|v| v.rate > 0)
            .filter(|v| !nodes1.contains(&v.id))
            .map(|v| v.id.clone())
            .collect();

        let (max2, _) = resolve_max_pressure0("AA", 26, &mut nodes2.clone(), &dists, &map);

        let lmax = max1 + max2;
        if lmax > max {
            println!("max={}", lmax);
        }
        max = max.max(lmax);
    }
    max
}
