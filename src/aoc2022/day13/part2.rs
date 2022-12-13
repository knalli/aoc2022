use std::borrow::Borrow;
use crate::aoc2022::day13::part1::{Packet, parse_input};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let mut packets = parse_input(scope, "puzzle1");
    let divider1 = Packet::LIST(vec!(Packet::LIST(vec!(Packet::VALUE(2)))));
    let divider2 = Packet::LIST(vec!(Packet::LIST(vec!(Packet::VALUE(6)))));
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    // that's it
    packets.sort();

    println!("Packets");
    packets.iter().for_each(|p| {
        println!("{}", p.to_string());
    });

    let mut divider1_idx = 0;
    let mut divider2_idx = 0;
    for i in 0..packets.len() {
        if divider1.borrow().eq(packets.get(i).unwrap()) {
            divider1_idx = i+1;
        }
        if divider2.borrow().eq(packets.get(i).unwrap()) {
            divider2_idx = i+1;
        }
    }

    write_solution(&scope, format!("signal = {}", divider1_idx * divider2_idx).as_str());
}