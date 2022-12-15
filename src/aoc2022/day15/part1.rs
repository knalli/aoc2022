use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::RangeInclusive;

use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle0");
    let map = build_map(input.borrow());
    let row_in_question = 2_000_000;
    write_solution(&scope, format!("count covered in row {} = {}", row_in_question, map.count_covered_in_row(row_in_question)).as_str());
}

pub struct Sensor {
    pub pos: (i32, i32),
    pub closest_beacon: (i32, i32),
    pub distance: i32,
}

impl Sensor {
    pub fn new(pos_x: i32, pos_y: i32, beacon_x: i32, beacon_y: i32) -> Self {
        let pos = (pos_x, pos_y);
        let closest_beacon = (beacon_x, beacon_y);
        let distance = Sensor::manhatten_distance(pos.clone(), closest_beacon.clone());
        Self {
            pos,
            closest_beacon,
            distance,
        }
    }

    pub fn manhatten_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }
}

pub struct Map {
    pub data: HashMap<(i32, i32), Marker>,
    pub signal_distances: HashMap<(i32, i32), i32>,
    pub min: (i32, i32),
    pub max: (i32, i32),
}

struct MultiRange {
    ranges: Vec<(i32, i32)>,
}

impl MultiRange {
    fn new() -> Self {
        Self { ranges: vec!() }
    }

    fn append(&mut self, range: RangeInclusive<i32>) {
        // see also
        // https://github.com/Killavus/Advent-of-Code-2022/blob/aec9418dd4e219863c2a1552d8275806aafc124b/15-beacon-exclusion-zone/src/main.rs

        let mut idx = 0;
        let mut overlaps = vec!();

        while idx < self.ranges.len() {
            let (start, end) = self.ranges[idx];
            if range.contains(&start)
                || range.contains(&end)
                || (start..=end).contains(range.start())
                || (start..=end).contains(range.end())
            {
                overlaps.push(self.ranges.remove(idx));
            } else {
                idx = idx + 1;
            }
        }

        // combine
        let result_range = overlaps
            .into_iter()
            .fold(range, |acc, (start, end)| {
                *acc.start().min(&start)..=*acc.end().max(&end)
            });

        self.ranges.push((*result_range.start(), *result_range.end()));
    }

    fn reduce_to(&mut self, limit_range: RangeInclusive<i32>) {
        let mut idx = 0;
        while idx < self.ranges.len() {
            let (start, end) = self.ranges[idx];
            if limit_range.contains(&start)
                || limit_range.contains(&end)
                || (start..=end).contains(limit_range.start())
                || (start..=end).contains(limit_range.end())
            {
                self.ranges[idx] = (start.max(*limit_range.start()), end.min(*limit_range.end()));
                idx = idx + 1;
            } else {
                self.ranges.remove(idx);
            }
        }

        self.ranges.sort_by_key(|(start, _)| *start);
    }

    fn coverage(&self) -> i32 {
        self.ranges
            .iter()
            .map(|(start, end)| (end - start + 1).abs())
            .sum()
    }

    fn first_gap(&self) -> Option<i32> {
        for ranges in self.ranges.as_slice().windows(2) {
            let (_, end) = ranges[0];
            let (start, _) = ranges[1];
            if start - end == 2 { // why not more than 2?
                return Some(end + 1);
            }
        }
        None
    }
}


impl Map {
    pub fn build_min_max(map: &HashMap<(i32, i32), Marker>) -> ((i32, i32), (i32, i32)) {
        let all_pos: Vec<(i32, i32)> = map
            .iter()
            .map(|(pos, _)| pos.clone())
            .collect();

        let min_x: i32 = all_pos.iter()
            .map(|(x, _)| x.clone())
            .min()
            .unwrap();
        let max_x: i32 = all_pos.iter()
            .map(|(x, _)| x.clone())
            .max()
            .unwrap();
        let min_y: i32 = all_pos.iter()
            .map(|(_, y)| y.clone())
            .min()
            .unwrap();
        let max_y: i32 = all_pos.iter()
            .map(|(_, y)| y.clone())
            .max()
            .unwrap();

        ((min_x, min_y), (max_x, max_y))
    }

    pub fn count_covered_in_row(&self, y: i32) -> i32 {
        let signal_min_x: i32 = self.signal_distances
            .iter()
            .map(|(pos, distance)| pos.0 - distance)
            .min()
            .unwrap();
        let signal_max_x: i32 = self.signal_distances
            .iter()
            .map(|(pos, distance)| pos.0 + distance)
            .max()
            .unwrap();
        let range_min = self.min.0.clone() + signal_min_x;
        let range_max = self.max.0.clone() + signal_max_x;
        (range_min..=range_max).into_iter()
            .filter(|&x| {
                let pos = (x, y);
                // filter out sensor and beacons
                if self.data.get(pos.borrow()).is_some() {
                    return false;
                }
                self.signal_distances
                    .iter()
                    .any(|(&signal_pos, &signal_distance)| {
                        Sensor::manhatten_distance(pos.clone(), signal_pos.clone()) <= signal_distance
                    })
            })
            .count() as i32
    }

    pub fn find_uncovered(&self, limits: RangeInclusive<i32>) -> Option<(i32, i32)> {
        for y in limits.clone() {
            //println!("y={}", y);
            let mut multi_range = MultiRange::new();
            self.signal_distances
                .iter()
                .filter(|(signal, &distance)| (y - signal.1).abs() <= distance)
                .for_each(|(signal, distance)| {
                    // this cost me much.. the scale is reversed!
                    let reverse_scale = distance - (y - signal.1).abs();
                    multi_range.append((signal.0 - reverse_scale)..=(signal.0 + reverse_scale));
                });
            multi_range.reduce_to(limits.clone());
            let cov = multi_range.coverage();
            //println!("  cov={}", cov);
            if cov == limits.clone().last().unwrap() {
                return multi_range.first_gap().map(|x| (x, y));
            }
        }

        None
    }
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Vec<Sensor> {
    io::read_puzzle_as_list(scope.day(), puzzle)
        .iter()
        .map(|line| {
            let split: Vec<&str> = line.split(": ").collect();
            let sensor_pos: Vec<&str> = split.get(0)
                .unwrap()
                .strip_prefix("Sensor at ")
                .unwrap()
                .split(", ")
                .collect();
            let sensor_x: i32 = parse_int(sensor_pos.get(0).unwrap().strip_prefix("x=").unwrap());
            let sensor_y: i32 = parse_int(sensor_pos.get(1).unwrap().strip_prefix("y=").unwrap());
            let beacon_pos: Vec<&str> = split.get(1)
                .unwrap()
                .strip_prefix("closest beacon is at ")
                .unwrap()
                .split(", ")
                .collect();
            let beacon_x: i32 = parse_int(beacon_pos.get(0).unwrap().strip_prefix("x=").unwrap());
            let beacon_y: i32 = parse_int(beacon_pos.get(1).unwrap().strip_prefix("y=").unwrap());
            Sensor::new(sensor_x, sensor_y, beacon_x, beacon_y)
        })
        .collect()
}

#[derive(PartialEq, Eq)]
pub enum Marker {
    Sensor,
    Beacon,
}

impl ToString for Marker {
    fn to_string(&self) -> String {
        match self {
            Marker::Beacon => "B".to_string(),
            Marker::Sensor => "S".to_string(),
        }
    }
}

pub fn build_map(input: &[Sensor]) -> Map {
    let mut map = HashMap::new();
    let mut signal_distances = HashMap::new();
    for sensor in input {
        signal_distances.insert(sensor.pos.clone(), sensor.distance);
        map.entry(sensor.pos.clone())
            .and_modify(|_| unreachable!())
            .or_insert(Marker::Sensor);
        map.entry(sensor.closest_beacon.clone())
            // beacon marked more than once, otherwise error
            .and_modify(|x| match x {
                Marker::Beacon => (),
                _ => unreachable!()
            })
            .or_insert(Marker::Beacon);
    }

    let (min, max) = Map::build_min_max(map.borrow());

    Map { data: map, min, max, signal_distances }
}

pub fn _map_to_string(map: &Map) -> String {
    let mut result = "".to_owned();

    for y in map.min.1..=map.max.1 {
        result = result + format!("{:010}  ", y).as_str();
        for x in map.min.0..=map.max.0 {
            match map.data.get(&(x, y)) {
                Some(marker) => {
                    result = result + marker.to_string().as_str()
                }
                None => result = result + " ",
            }
        }
        result = result + "\n";
    }

    result
}


