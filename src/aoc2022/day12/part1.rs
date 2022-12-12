use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}


fn execute(scope: &PuzzleScope) {
    let (_, cost) = fewest_steps(parse_input(scope, "puzzle1"));
    write_solution(&scope, format!("cost = {}", cost.unwrap()).as_str());
}

pub fn fewest_steps(input: Input) -> (Vec<(i32, i32)>, Option<i32>) {
    let mut q = BinaryHeap::new();
    let mut dists: HashMap<(i32, i32), i32> = HashMap::new();
    let mut prevs: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    dists.insert(input.start.clone(), 0);
    q.push(Item2::new(input.start.clone(), 0));

    let decode_value = |(x, y)| {
        if let Some(&v) = input.map.get(&(x, y)) {
            Some((v, match v {
                'S' => 'a',
                'E' => 'z',
                v => v,
            }))
        } else {
            None
        }
    };

    while let Some(Item2 { pos, cost }) = q.pop() {
        let (pos_c, pos_v) = decode_value(pos.clone()).unwrap();
        if pos_c == 'E' {
            return (vec!(), Some(cost));
        }

        for next_pos in vec!((pos.0, pos.1 - 1), (pos.0 + 1, pos.1), (pos.0, pos.1 + 1), (pos.0 - 1, pos.1)) {
            // not in path
            /*
            if item.path.contains(&next_pos) {
                continue;
            }
             */
            /*
            if let Some(already_known_cost) = costs.get(&next_pos) {
                if *already_known_cost < item.cost+1 {
                    // abandon this path
                    continue;
                }
            }
             */
            if let Some((_, next_v)) = decode_value(next_pos) {
                if (pos_v as i32) >= (next_v as i32) - 1 {
                    if let Some(&pos_dist) = dists.get(&pos) {
                        let alt = pos_dist + 1;
                        if let Some(&next_dist) = dists.get(&next_pos) {
                            if alt < next_dist {
                                dists.insert(next_pos, alt);
                                prevs.insert(next_pos, pos.clone());
                                q.push(Item2::new(next_pos, alt));
                            }
                        } else {
                            dists.insert(next_pos, alt);
                            prevs.insert(next_pos, pos.clone());
                            q.push(Item2::new(next_pos, alt));
                        }
                    }
                }
            }
        }
    }

    (vec!(), None)
}

pub struct Input {
    pub map: HashMap<(i32, i32), char>,
    pub start: (i32, i32),
}

struct Item2 {
    pos: (i32, i32),
    cost: i32,
}

impl Eq for Item2 {}

impl Ord for Item2 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialEq<Self> for Item2 {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for Item2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Item2 {
    pub fn new(pos: (i32, i32), cost: i32) -> Self {
        Self { pos, cost }
    }
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Input {
    let data: Vec<Vec<char>> = io::read_puzzle_as_list(scope.day(), puzzle)
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut start: Option<(i32, i32)> = None;
    for y in 0..data.len() {
        let row = data.get(y).unwrap();
        for x in 0..row.len() {
            if let Some(v) = row.get(x) {
                if *v == 'S' {
                    start = Some((x as i32, y as i32));
                }
                map.insert((x as i32, y as i32), v.clone());
            }
        }
    }
    Input {
        map,
        start: start.unwrap(),
    }
}
