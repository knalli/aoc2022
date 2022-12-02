use crate::aoc2022::day02::rps::RPS;
use crate::aoc2022::day02::rps::Winning;
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
        let player_select = RPS::decode(v.chars().nth(2).unwrap());
        score += score_round(opponent_select, player_select);
    }

    score
}

pub fn score_round(opponent_select: RPS, player_select: RPS) -> i32 {
    let player_score = if player_select == RPS::ROCK {
        1
    } else if player_select == RPS::PAPER {
        2
    } else if player_select == RPS::SCISSOR {
        3
    } else {
        panic!("invalid player select")
    };
    let win = player_select.wins(&opponent_select);

    let mut result = 0;
    result += player_score;
    result += if win == Winning::YES {
        6
    } else if win == Winning::DRAW {
        3
    } else {
        0
    };
    result
}
