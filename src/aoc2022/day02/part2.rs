use crate::aoc2022::day02::part1::score_round;
use crate::aoc2022::day02::rps::RPS;
use crate::aoc2022::lib;
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let score: i32 = play(scope, "puzzle1");
    write_solution(&scope, format!("score = {}", score).as_str());
}

fn play(scope: &PuzzleScope, puzzle: &str) -> i32 {
    let mut score: i32 = 0;
    for v in lib::io::read_puzzle_as_list(scope.day(), puzzle) {
        let opponent_select = RPS::decode(v.chars().nth(0).unwrap());
        let player_select_strategy = v.chars().nth(2).unwrap();
        let player_select = match player_select_strategy {
            'X' => opponent_select.require_for_lost(),
            'Y' => opponent_select,
            'Z' => opponent_select.require_for_win(),
            _ => panic!("invalid player select")
        };
        score += score_round(opponent_select, player_select);
    }

    score
}
