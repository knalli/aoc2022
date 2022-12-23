use anyhow::Result;

use crate::aoc2022::day22::board::{Board, Point2D, Tile};
use crate::aoc2022::day22::direction::Direction;
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let (board, directions) = parse_input(scope, "puzzle1").unwrap();
    let (point, facing) = do_run(&board, &directions).unwrap();
    let password = (1000 * point.1) + (4 * point.0) + match facing {
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3,
    };
    write_solution(&scope, format!("password = {}", password).as_str());
}

fn do_run(board: &Board, directions: &[Direction]) -> Result<(Point2D, Facing)> {
    // first
    let mut facing = Facing::Up;
    let mut pos = board.top_left();

    for direction in directions {
        let mut steps = 0;
        match direction {
            Direction::Right(v) => {
                facing = match facing {
                    Facing::Up => Facing::Right,
                    Facing::Right => Facing::Down,
                    Facing::Down => Facing::Left,
                    Facing::Left => Facing::Up,
                };
                steps += v;
            }
            Direction::Left(v) => {
                facing = match facing {
                    Facing::Up => Facing::Left,
                    Facing::Right => Facing::Up,
                    Facing::Down => Facing::Right,
                    Facing::Left => Facing::Down,
                };
                steps += v;
            }
        }
        let offset = match facing {
            Facing::Up => Point2D(0, -1),
            Facing::Right => Point2D(1, 0),
            Facing::Down => Point2D(0, 1),
            Facing::Left => Point2D(-1, 0),
        };

        let mut next = pos + offset.clone();
        while steps > 0 {
            match board.data.get(&next) {
                Some(Tile::Open) => {
                    steps -= 1;
                    pos = next;
                    next = pos + offset.clone();
                    continue;
                }
                Some(Tile::Solid) => {
                    break;
                }
                None => {
                    match offset.0 {
                        -1 => {
                            next = board.find_outer_next_left(&Point2D(board.width, next.1)).unwrap();
                            continue;
                        }
                        1 => {
                            next = board.find_outer_next_right(&Point2D(1, next.1)).unwrap();
                            continue;
                        }
                        _ => (),
                    }
                    match offset.1 {
                        -1 => {
                            next = board.find_outer_next_up(&Point2D(next.0, board.height)).unwrap();
                            continue;
                        }
                        1 => {
                            next = board.find_outer_next_down(&Point2D(next.0, 1)).unwrap();
                            continue;
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    Ok((pos, facing))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Facing {
    Right,
    Down,
    Left,
    Up,
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Result<(Board, Vec<Direction>)> {
    let lines = io::read_puzzle_as_list(scope.day(), puzzle);
    let board = lines[0..lines.len() - 2].join("\n").parse()?;
    let s = "R".to_owned() + lines.last().unwrap();
    let directions = Direction::read_multiple(s.as_str())?;
    Ok((board, directions))
}
