use crate::aoc2022::day07::fs::Fd;
use crate::aoc2022::day07::part1::parse_input;
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

const TOTAL_LIMIT: usize = 70_000_000;
const UNUSED_LIMIT: usize = 30_000_000;

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute<'a>(scope: &PuzzleScope) {
    let fs = parse_input(scope, "puzzle1");
    //println!("{}", fs.to_string());

    let mut list: Vec<(Fd, i32)> = Vec::new();

    let mut ss = vec!(fs.root());
    while let Some(fd) = ss.pop() {
        let f = fs.get(fd).unwrap();
        if f.is_dir() {
            list.push((fd, fs.size_total(fd).unwrap()));
            for c_fd in f.children() {
                ss.push(c_fd);
            }
        }
    }
    // sort by size desc
    list.sort_by(|(_, size1), (_, size2)| size1.cmp(size2));

    let total = fs.size_total(fs.root()).unwrap();
    let required: i32 = (TOTAL_LIMIT - UNUSED_LIMIT) as i32;

    let mut found = false;
    for (_, size) in list {
        if required > (total - size) {
            write_solution(&scope, format!("result = {}", size).as_str());
            found = true;
            break;
        }
    }

    if !found {
        write_solution(&scope, "NO RESULT");
    }
}
