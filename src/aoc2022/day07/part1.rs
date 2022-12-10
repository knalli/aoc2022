use std::cell::RefCell;

use crate::aoc2022::day07::fs::Filesystem;
use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub const FILE_LIMIT: i32 = 100_000;

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute<'a>(scope: &PuzzleScope) {
    let fs = parse_input(scope, "puzzle1");
    println!("{}", fs.to_string());

    let sum = RefCell::new(0);
    fs.each(|fd| {
        if let Some(f) = fs.get(fd) {
            if f.is_dir() {
                let size = fs.size_total(fd).unwrap();
                if size <= FILE_LIMIT {
                    sum.replace(sum.clone().into_inner() + size);
                }
            }
        }
    });

    write_solution(&scope, format!("result = {}", sum.borrow()).as_str());
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Filesystem {
    let mut filesystem = Filesystem::new();
    let mut current_fd = filesystem.root();
    let mut mode_ls = false;
    for line in io::read_puzzle_as_list(scope.day(), puzzle) {
        if line.starts_with("$") {
            mode_ls = false;
        }
        if line.starts_with("$ cd ") {
            let arg = &line[5..];
            match arg {
                ".." => {
                    // go up
                    current_fd = filesystem.parent(current_fd).unwrap();
                }
                "/" => {
                    current_fd = filesystem.root();
                }
                _ => {
                    current_fd = filesystem.find(current_fd, arg.to_string()).unwrap();
                }
            }
        } else if line.starts_with("$ ls") {
            mode_ls = true;
        } else if mode_ls {
            if line.starts_with("dir ") {
                let name = &line[4..];
                filesystem.mkdir(current_fd, name);
            } else {
                let mut split = line.split(" ");
                let size = parse_int(split.next().unwrap()) as i32;
                let name = split.next().unwrap();
                filesystem.add_file(current_fd, name, size);
            }
        } else {
            // this line will be ignored
        }
    }

    filesystem
}
