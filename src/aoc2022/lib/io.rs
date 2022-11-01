use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_puzzle_as_string(day: i32, id: &str) -> String {
    let path = format!("puzzles/day{:02}/{1}.txt", day, id);
    fs::read_to_string(path)
        .expect("puzzle file does not exist")
}

#[allow(dead_code)]
pub fn read_puzzle_as_list(day: i32, id: &str) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for value in read_puzzle_as_string(day, id)
        .split("\n") {
        res.push(String::from(value));
    }
    res
}

#[allow(dead_code)]
pub fn read_puzzle_as_ints(day: i32, id: &str) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for v in read_puzzle_as_string(day, id)
        .split("\n") {
        let type_value: i32 = v.parse().unwrap();
        res.push(type_value);
    }
    res
}

#[allow(dead_code)]
pub fn read_puzzle_first_line(day: i32, id: &str) -> String {
    let path = format!("puzzles/day{:02}/{1}.txt", day, id);
    println!("{}", path);
    let file = File::open(path)
        .expect("puzzle file does not exist");
    let mut buffer = BufReader::new(file);
    let mut line = String::new();
    let _ = buffer.read_line(&mut line);
    line
}