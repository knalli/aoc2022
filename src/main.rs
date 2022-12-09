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
    hive.register(Day::D08, Part::P01, aoc2022::day08::part1::run);
    hive.register(Day::D08, Part::P02, aoc2022::day08::part2::run);
    hive.register(Day::D09, Part::P01, aoc2022::day09::part1::run);
    hive.register(Day::D09, Part::P02, aoc2022::day09::part2::run);
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
