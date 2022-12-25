use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    write_solution(&scope, format!("sum = {}", convert_dec2snafu(convert_and_sum(scope, "puzzle1"))).as_str());
}

fn convert_and_sum(scope: &PuzzleScope, puzzle: &str) -> isize {
    io::read_puzzle_as_list(scope.day(), puzzle)
        .iter()
        .map(|s| convert_snafu2dec(s))
        .sum()
}

fn convert_snafu2dec(str: &str) -> isize {
    let len = str.len();
    str.chars().enumerate()
        .into_iter()
        .fold(0isize, |acc, (i, c)| {
            let r = (len - i - 1) as u32;
            let v = 5isize.pow(r);
            acc + match c {
                '-' => -v,
                '=' => -2 * v,
                '0' => 0,
                '1' => v,
                '2' => 2 * v,
                _ => unreachable!(),
            }
        })
}

fn convert_dec2snafu(num: isize) -> String {
    let mut result = "".to_owned();

    let mut from = num;
    let mut base = 0;
    while from != 0 {
        let v10 = from % 10;

        let mut v5_shift = v10 / 5;
        let v5_remainder = v10 % 5;

        match v5_remainder {
            4 => {
                v5_shift = v5_shift + 1;
                //v5_remainder = -1;
                result = "-".to_owned() + result.as_str();
            }
            3 => {
                v5_shift = v5_shift + 1;
                //v5_remainder = -2;
                result = "=".to_owned() + result.as_str();
            }
            v => {
                result = v.to_string() + result.as_str();
            }
        }

        from = (from - v10 + v5_shift * 5) / 5;
        base = base + 1;
    }

    result.to_string()
}

#[test]
fn test_snafu2dec() {
    assert_eq!(1747, convert_snafu2dec("1=-0-2"), "1=-0-2");
}

#[test]
fn test1_dec2snafu() {
    assert_eq!("1=-0-2", convert_dec2snafu(1747), "1747");
}

#[test]
fn test2_dec2snafu() {
    assert_eq!("12", convert_dec2snafu(7), "7");
}

#[test]
fn test3_dec2snafu() {
    assert_eq!("1=", convert_dec2snafu(3), "3");
}

#[test]
fn test4_dec2snafu() {
    assert_eq!("122", convert_dec2snafu(37), "37");
}

#[test]
fn test5_dec2snafu() {
    assert_eq!("21", convert_dec2snafu(11), "11");
}