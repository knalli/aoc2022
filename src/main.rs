extern crate core;

use crate::aoc2022::hive::{Day, Hive, Part};
use crate::aoc2022::lib::common::{parse_int, PuzzleScope};

mod aoc2022;

fn main() {
    let mut hive = Hive::create();
    hive.register(Day::D00, Part::P01, aoc2022::day00::part1::run);
    hive.register(Day::D00, Part::P02, aoc2022::day00::part2::run);
    hive.register(Day::D01, Part::P01, aoc2022::day01::part1::run);
    hive.register(Day::D01, Part::P02, aoc2022::day01::part2::run);
    hive.register(Day::D02, Part::P01, aoc2022::day02::part1::run);
    hive.register(Day::D02, Part::P02, aoc2022::day02::part2::run);
    hive.register(Day::D03, Part::P01, aoc2022::day03::part1::run);
    hive.register(Day::D03, Part::P02, aoc2022::day03::part2::run);
    hive.register(Day::D04, Part::P01, aoc2022::day04::part1::run);
    hive.register(Day::D04, Part::P02, aoc2022::day04::part2::run);
    hive.register(Day::D05, Part::P01, aoc2022::day05::part1::run);
    hive.register(Day::D05, Part::P02, aoc2022::day05::part2::run);
    hive.register(Day::D06, Part::P01, aoc2022::day06::part1::run);
    hive.register(Day::D06, Part::P02, aoc2022::day06::part2::run);
    hive.register(Day::D07, Part::P01, aoc2022::day07::part1::run);
    hive.register(Day::D07, Part::P02, aoc2022::day07::part2::run);
    hive.register(Day::D08, Part::P01, aoc2022::day08::part1::run);
    hive.register(Day::D08, Part::P02, aoc2022::day08::part2::run);
    hive.register(Day::D09, Part::P01, aoc2022::day09::part1::run);
    hive.register(Day::D09, Part::P02, aoc2022::day09::part2::run);
    hive.register(Day::D10, Part::P01, aoc2022::day10::part1::run);
    hive.register(Day::D10, Part::P02, aoc2022::day10::part2::run);
    hive.register(Day::D11, Part::P01, aoc2022::day11::part1::run);
    hive.register(Day::D11, Part::P02, aoc2022::day11::part2::run);
    hive.register(Day::D12, Part::P01, aoc2022::day12::part1::run);
    hive.register(Day::D12, Part::P02, aoc2022::day12::part2::run);
    hive.register(Day::D13, Part::P01, aoc2022::day13::part1::run);
    hive.register(Day::D13, Part::P02, aoc2022::day13::part2::run);
    hive.register(Day::D14, Part::P01, aoc2022::day14::part1::run);
    hive.register(Day::D14, Part::P02, aoc2022::day14::part2::run);
    hive.register(Day::D15, Part::P01, aoc2022::day15::part1::run);
    hive.register(Day::D15, Part::P02, aoc2022::day15::part2::run);
    hive.register(Day::D16, Part::P01, aoc2022::day16::part1::run);
    hive.register(Day::D16, Part::P02, aoc2022::day16::part2::run);
    hive.register(Day::D17, Part::P01, aoc2022::day17::part1::run);
    hive.register(Day::D17, Part::P02, aoc2022::day17::part2::run);
    hive.register(Day::D18, Part::P01, aoc2022::day18::part1::run);
    hive.register(Day::D18, Part::P02, aoc2022::day18::part2::run);
    hive.register(Day::D19, Part::P01, aoc2022::day19::part1::run);
    hive.register(Day::D19, Part::P02, aoc2022::day19::part2::run);
    hive.register(Day::D20, Part::P01, aoc2022::day20::part1::run);
    hive.register(Day::D20, Part::P02, aoc2022::day20::part2::run);
    hive.register(Day::D21, Part::P01, aoc2022::day21::part1::run);
    hive.register(Day::D21, Part::P02, aoc2022::day21::part2::run);
    //GEN_HIVE_REGISTER

    let day = format!("d{:02}", parse_int(std::env::var("DAY").unwrap_or("1".to_string()).as_str()));
    let part = format!("p{:02}", parse_int(std::env::var("PART").unwrap_or("1".to_string()).as_str()));

    //println!("Dispatching {day} / {part}", day = day.clone(), part = part.clone());
    dispatch(&hive, day, part);

    println!();
    println!("kthxbye!")
}

fn dispatch(hive: &Hive, day: String, part: String) {
    let d = Day::from_str(day.clone())
        .expect(format!("invalid day: {}", day).as_str());
    let p = Part::from_str(part.clone())
        .expect(format!("invalid part: {}", part).as_str());
    let cb = hive.lookup(d, p)
        .expect("invalid day");
    let scope = PuzzleScope::create(2022, d.as_int(), p.as_int());
    cb(scope);
}
