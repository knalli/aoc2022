use std::collections::HashMap;

use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let source = (500, 0);
    let mut carve = build_carve(parse_input(scope, "puzzle1"));
    carve.insert(source.clone(), Pixel::Source);

    println!("START");
    print_carve(&carve);
    println!();

    let max_y: i32 = carve.iter()
        .map(|((_, y), _)| y.clone())
        .max()
        .unwrap();

    let_that_sink_in(&mut carve, &source, max_y);

    println!("END");
    print_carve(&carve);
    println!();

    let sand_count = carve.iter()
        .filter(|(_, pixel)| {
            match pixel.clone() {
                Pixel::Rested => true,
                _ => false
            }
        })
        .count();

    write_solution(&scope, format!("count = {}", sand_count).as_str());
}

pub fn let_that_sink_in(carve: &mut HashMap<(i32, i32), Pixel>, source: &(i32, i32), max_y: i32) {
    let mut tick = 0;
    loop {
        //println!();
        //print_carve(&carve);
        //println!();

        let mut current = (source.0, source.1);

        let mut desired_state: Option<Pixel> = None;
        loop {
            if current.1 == max_y {
                desired_state = Some(Pixel::Flow);
                break;
            } else if desired_state.is_some() {
                break;
            }
            let mut found = false;
            let candidates = vec!((current.0, current.1 + 1), (current.0 - 1, current.1 + 1), (current.0 + 1, current.1 + 1));
            for nn in candidates.clone() {
                match carve.get(&nn) {
                    Some(Pixel::Flow) => {
                        desired_state = Some(Pixel::Flow);
                        found = true;
                        break;
                    }
                    _ => ()
                }
            }
            if !found {
                for nn in candidates {
                    match carve.get(&nn) {
                        None => {
                            current = nn.clone();
                            found = true;
                            break;
                        }
                        _ => ()
                    }
                }
            }
            if found {
                tick = tick + 1;
            } else {
                break;
            }
        }

        if let Some(pixel) = desired_state {
            carve.insert(current.clone(), pixel);
        } else {
            carve.insert(current.clone(), Pixel::Rested);
        }

        if current == source.clone() {
            break;
        }
    }
}

pub fn build_carve(input: Vec<Vec<(i32, i32)>>) -> HashMap<(i32, i32), Pixel> {
    let mut map: HashMap<(i32, i32), Pixel> = HashMap::new();
    for path in input {
        for i in 1..path.len() {
            let from = path.get(i - 1).unwrap().clone();
            let to = path.get(i).unwrap().clone();
            if from.0 == to.0 {
                let x = from.0;
                let y_min = from.1.min(to.1);
                let y_max = from.1.max(to.1);
                for y in y_min..=y_max {
                    map.insert((x, y), Pixel::Rock);
                }
            } else if from.1 == to.1 {
                let y = from.1;
                let x_min = from.0.min(to.0);
                let x_max = from.0.max(to.0);
                for x in x_min..=x_max {
                    map.insert((x, y), Pixel::Rock);
                }
            }
        }
    }
    map
}

pub enum Pixel {
    Rock,
    Source,
    Rested,
    Flow,
}

pub fn print_carve(carve: &HashMap<(i32, i32), Pixel>) {
    let x_max: i32 = carve.keys()
        .into_iter()
        .map(|(x, _)| x)
        .max()
        .unwrap()
        .clone();
    let y_max: i32 = carve.keys()
        .into_iter()
        .map(|(_, y)| y)
        .max()
        .unwrap()
        .clone();
    for y in 0..=y_max {
        let mut line = "".to_owned();
        for x in 0..=x_max {
            match carve.get(&(x, y)) {
                Some(pixel) => line = line + pixel.to_string().as_str(),
                None => line = line + " ",
            }
        }
        println!("{}", line);
    }
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        match self {
            Pixel::Rock => "#".to_string(),
            Pixel::Source => "+".to_string(),
            Pixel::Rested => "o".to_string(),
            Pixel::Flow => "~".to_string(),
        }
    }
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Vec<Vec<(i32, i32)>> {
    parse_lines(io::read_puzzle_as_list(scope.day(), puzzle))
}

pub fn parse_lines(lines: Vec<String>) -> Vec<Vec<(i32, i32)>> {
    lines
        .iter()
        .map(|line| {
            line.split(" -> ")
                .into_iter()
                .map(|s| {
                    let ints: Vec<i32> = s.split(',')
                        .into_iter()
                        .map(|p| parse_int(p))
                        .collect();
                    // extract first 2 as tuple
                    match &ints[..] {
                        &[a, b, ..] => (a, b),
                        _ => unreachable!(),
                    }
                })
                .collect()
        })
        .collect()
}

#[test]
fn test_parse_lines() {
    let result = parse_lines(vec!("498,4 -> 498,6 -> 496,6".to_owned()));
    assert_eq!(1, result.len());
    let item = result.get(0).unwrap();
    assert_eq!(3, item.len());
    assert_eq!((498, 4), *(item.get(0).unwrap()));
    assert_eq!((498, 6), *(item.get(1).unwrap()));
    assert_eq!((496, 6), *(item.get(2).unwrap()));
}