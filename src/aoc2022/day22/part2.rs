use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

use crate::aoc2022::day22::board::{Board, Point2D, Tile};
use crate::aoc2022::day22::direction::Direction;
use crate::aoc2022::day22::part1::{Facing, parse_input};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let (board, directions) = parse_input(scope, "puzzle1").unwrap();
    let (point, facing) = do_run(&board, 50, &directions).unwrap();
    let password = (1000 * point.1) + (4 * point.0) + match facing {
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3,
    };
    println!("{:?}", point);
    println!("{:?}", facing);
    write_solution(&scope, format!("password = {}", password).as_str());
}

enum Op {
    Same,
    RotateX,
    RotateY,
    Flip,
}

fn do_run(board: &Board, n: isize, directions: &[Direction]) -> Result<(Point2D, Facing)> {
    let mut facing = Facing::Up;
    let mut pos = board.top_left();
    let neighbors = build_cube_neighbors(board, n);
    let mut last_facing_map: HashMap<Point2D, Facing> = HashMap::new();
    println!("starting {:?} {:?}", &pos, &facing);

    neighbors.iter();

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
                println!("{:?} {:?}", &facing, v);
            }
            Direction::Left(v) => {
                facing = match facing {
                    Facing::Up => Facing::Left,
                    Facing::Right => Facing::Up,
                    Facing::Down => Facing::Right,
                    Facing::Left => Facing::Down,
                };
                steps += v;
                println!("{:?} {:?}", &facing, v);
            }
        }
        last_facing_map.insert(pos.clone(), facing.clone());

        let mut offset = match facing {
            Facing::Up => Point2D(0, -1),
            Facing::Right => Point2D(1, 0),
            Facing::Down => Point2D(0, 1),
            Facing::Left => Point2D(-1, 0),
        };

        let mut next = pos + offset.clone();
        while steps > 0 {
            println!("? {:?}={:?}", &next, board.data.get(&next));

            /*
            println!("{}", "=".repeat(100));
            for y in 1..=board.height {
                let str = (1..=board.width).into_iter()
                    .map(|x| {
                        let p = Point2D(x,y);
                        if let Some(f) = last_facing_map.get(&p) {
                            match f {
                                Facing::Up => "^",
                                Facing::Down => "v",
                                Facing::Left => "<",
                                Facing::Right => ">",
                            }
                        } else if let Some(t) = board.data.get(&p) {
                            match t {
                                Tile::Open => ".",
                                Tile::Solid => "#",
                            }
                        } else {
                            " "
                        }
                    })
                    .collect_vec()
                    .join("");
                println!("{}", str);
                println!();
            }
            println!("{}", "=".repeat(100));
             */

            if !board.data.contains_key(&next) {
                match offset.0 {
                    -1 => {
                        let (p, f) = neighbors.get(&(pos.clone(), facing)).unwrap();
                        next = p.clone();
                        facing = f.clone();
                    }
                    1 => {
                        let (p, f) = neighbors.get(&(pos.clone(), facing)).unwrap();
                        next = p.clone();
                        facing = f.clone();
                    }
                    _ => (),
                }
                match offset.1 {
                    -1 => {
                        let (p, f) = neighbors.get(&(pos.clone(), facing)).unwrap();
                        next = p.clone();
                        facing = f.clone();
                    }
                    1 => {
                        let (p, f) = neighbors.get(&(pos.clone(), facing)).unwrap();
                        next = p.clone();
                        facing = f.clone();
                    }
                    _ => (),
                }
            }

            offset = match facing {
                Facing::Up => Point2D(0, -1),
                Facing::Right => Point2D(1, 0),
                Facing::Down => Point2D(0, 1),
                Facing::Left => Point2D(-1, 0),
            };

            match board.data.get(&next) {
                Some(Tile::Open) => {
                    println!("p={:?}", &next);
                    steps -= 1;
                    pos = next;
                    next = pos + offset.clone();
                    last_facing_map.insert(pos.clone(), facing.clone());
                    continue;
                }
                Some(Tile::Solid) => {
                    break;
                }
                _ => unreachable!(),
            }
        }
    }

    Ok((pos, last_facing_map.get(&pos).unwrap().clone()))
}

fn build_cube_neighbors(board: &Board, n: isize) -> HashMap<(Point2D, Facing), (Point2D, Facing)> {
    let all_points = board.data.keys().map(|p| p.clone()).collect_vec();

    let all_blocks: Vec<Point2D> = (1..=board.height).step_by(n as usize).into_iter()
        .flat_map(|y| {
            (1..=board.width).step_by(n as usize).into_iter()
                .map(move |x| Point2D(x, y))
        })
        .filter(|p| all_points.contains(p))
        .collect_vec();

    let mut neighbors: HashMap<(Point2D, Facing), (Point2D, Facing)> = HashMap::new();
    let cube_ops = build_cube_operations();

    for block in &all_blocks {
        println!("block = {:?}", block);
        for (index, veto_blocks, offset_facing, offset_ops, calc_facing, calc_ops) in &cube_ops {
            let other = block.clone() + Point2D(index.0 * n, index.1 * n);
            println!("  index = {:?}, => other = {:?} [veto={:?}]", index, &other, veto_blocks);
            if index.0 == -1 && index.1 == 3 {
                println!();
            }
            if index.0 == 1 && index.1 == -1 {
                println!();
            }
            if all_blocks.contains(&other) {
                if veto_blocks.iter().any(|veto| all_blocks.contains(&(block.clone() + Point2D(veto.0 * n, veto.1 * n)))) {
                    continue;
                }
                println!("  y");
                let block_offsets = (0..n).into_iter()
                    .map(|x| Point2D(x, 0))
                    .map(|p| {
                        let mut p = p;
                        for offset_op in offset_ops {
                            match offset_op {
                                Op::Flip => {
                                    p = Point2D(p.1, p.0);
                                }
                                Op::RotateX => {
                                    p = Point2D((n - 1) - p.0, p.1)
                                }
                                Op::RotateY => {
                                    p = Point2D(p.0, (n - 1) - p.1)
                                }
                                Op::Same => {}
                            }
                        }
                        p
                    })
                    .collect_vec();

                let other_offsets = block_offsets.clone().iter()
                    .map(|p| p.clone())
                    .map(|p| {
                        let mut p = p;
                        for offset_op in calc_ops {
                            match offset_op {
                                Op::Flip => {
                                    p = Point2D(p.1, p.0);
                                }
                                Op::RotateX => {
                                    p = Point2D((n - 1) - p.0, p.1)
                                }
                                Op::RotateY => {
                                    p = Point2D(p.0, (n - 1) - p.1)
                                }
                                Op::Same => {}
                            }
                        }
                        p
                    })
                    .collect_vec();

                for i in 0..n as usize {
                    let block_p = block.clone() + block_offsets[i];
                    let other_p = other.clone() + other_offsets[i];
                    let key = (block_p, offset_facing.clone());
                    let val = (other_p, calc_facing.clone());
                    println!("    neighbor: {:?} => {:?}", key.clone(), val.clone());
                    neighbors.insert(key, val);
                }
            }
        }
    }
    neighbors
}

fn build_cube_operations() -> Vec<(Point2D, Vec<Point2D>, Facing, Vec<Op>, Facing, Vec<Op>)> {
    // (offset_block, veto_blocks, offset_facing, offset_ops, target_facing, target_ops)
    let mut mm: Vec<(Point2D, Vec<Point2D>, Facing, Vec<Op>, Facing, Vec<Op>)> = vec!();

    // left
    mm.push((
        Point2D(-1, 0),
        vec!(),
        Facing::Left,
        vec!(Op::Flip),
        Facing::Left,
        vec!(Op::RotateX),
    ));
    // right
    mm.push((
        Point2D(1, 0),
        vec!(),
        Facing::Right,
        vec!(Op::Flip, Op::RotateX),
        Facing::Right,
        vec!(Op::RotateX),
    ));
    // up
    mm.push((
        Point2D(0, -1),
        vec!(),
        Facing::Up,
        vec!(Op::Same),
        Facing::Up,
        vec!(Op::RotateY),
    ));
    // down
    mm.push((
        Point2D(0, 1),
        vec!(),
        Facing::Down,
        vec!(Op::RotateY),
        Facing::Down,
        vec!(Op::RotateY),
    ));


    mm.push((
        Point2D(1, 1),
        vec!(Point2D(1, 0)),
        Facing::Right,
        vec!(Op::Flip, Op::RotateX),
        Facing::Down,
        vec!(Op::RotateX, Op::Flip, Op::RotateX),
    ));
    mm.push((
        Point2D(1, 1),
        vec!(Point2D(0, 1)),
        Facing::Down,
        vec!(Op::RotateY),
        Facing::Right,
        vec!(Op::RotateY, Op::Flip, Op::RotateY),
    ));

    mm.push((
        Point2D(-1, -1),
        vec!(Point2D(-1, 0)),
        Facing::Left,
        vec!(Op::Flip),
        Facing::Up,
        vec!(Op::Flip, Op::RotateY, Op::RotateX),
    ));
    mm.push((
        Point2D(-1, -1),
        vec!(Point2D(0, -1)),
        Facing::Right,
        vec!(Op::Same),
        Facing::Down,
        vec!(Op::Flip, Op::RotateY),
    ));

    mm.push((
        Point2D(-1, 1),
        vec!(Point2D(-1, 0)),
        Facing::Left,
        vec!(Op::Flip),
        Facing::Down,
        vec!(Op::Flip),
    ));
    mm.push((
        Point2D(-1, 1),
        vec!(Point2D(0, 1)),
        Facing::Down,
        vec!(Op::RotateY),
        Facing::Left,
        vec!(Op::Flip),
    ));

    mm.push((
        Point2D(1, -1),
        vec!(Point2D(0, -1)),
        Facing::Up,
        vec!(Op::Same),
        Facing::Right,
        vec!(Op::Flip),
    ));
    mm.push((
        Point2D(1, -1),
        vec!(Point2D(1, 0)),
        Facing::Right,
        vec!(Op::Flip, Op::RotateX),
        Facing::Up,
        vec!(Op::Flip),
    ));

    mm.push((
        Point2D(1, 2),
        vec!(Point2D(1, 0), Point2D(1, 1), Point2D(1, 1)),
        Facing::Right,
        vec!(Op::Flip, Op::RotateX),
        Facing::Left,
        vec!(Op::RotateY),
    ));
    mm.push((
        Point2D(1, 2),
        vec!(Point2D(-1, 0), Point2D(0, 1), Point2D(0, 2)),
        Facing::Left,
        vec!(Op::Flip),
        Facing::Right,
        vec!(Op::RotateY),
    ));

    mm.push((
        Point2D(1, -2),
        vec!(Point2D(1, 0), Point2D(1, -1), Point2D(2, -2)),
        Facing::Right,
        vec!(Op::Flip, Op::RotateX),
        Facing::Left,
        vec!(Op::RotateX),
    ));
    mm.push((
        Point2D(1, -2),
        vec!(Point2D(-1, 0), Point2D(0, -1), Point2D(0, -2)),
        Facing::Left,
        vec!(Op::Flip),
        Facing::Right,
        vec!(Op::RotateY),
    ));

    mm.push((
        Point2D(-1, 2),
        vec!(Point2D(1, 0), Point2D(0, 1), Point2D(0, 2)),
        Facing::Right,
        vec!(Op::Flip, Op::RotateX),
        Facing::Left,
        vec!(Op::RotateY),
    ));
    mm.push((
        Point2D(-1, 2),
        vec!(Point2D(-1, 0), Point2D(-1, 1), Point2D(-2, 2)),
        Facing::Left,
        vec!(Op::Flip),
        Facing::Right,
        vec!(Op::RotateY),
    ));

    mm.push((
        Point2D(-1, -2),
        vec!(Point2D(1, 0), Point2D(0, -1), Point2D(0, -2)),
        Facing::Right,
        vec!(Op::Flip, Op::RotateX),
        Facing::Left,
        vec!(Op::RotateY),
    ));
    mm.push((
        Point2D(-1, -2),
        vec!(Point2D(-1, 0), Point2D(-1, -1), Point2D(-2, -2)),
        Facing::Left,
        vec!(Op::Flip),
        Facing::Right,
        vec!(Op::RotateY),
    ));

    mm.push((
        Point2D(2, 1),
        vec!(Point2D(0, 1), Point2D(1, 1), Point2D(2, 2)),
        Facing::Down,
        vec!(Op::RotateY),
        Facing::Up,
        vec!(Op::RotateX),
    ));
    mm.push((
        Point2D(2, 1),
        vec!(Point2D(0, -1), Point2D(1, 0), Point2D(2, 0)),
        Facing::Up,
        vec!(Op::Same),
        Facing::Down,
        vec!(Op::RotateX),
    ));

    mm.push((
        Point2D(2, -1),
        vec!(Point2D(0, -1), Point2D(1, -1), Point2D(2, -2)),
        Facing::Up,
        vec!(Op::Same),
        Facing::Down,
        vec!(Op::RotateX),
    ));
    mm.push((
        Point2D(2, -1),
        vec!(Point2D(0, 1), Point2D(1, 0), Point2D(2, 0)),
        Facing::Down,
        vec!(Op::RotateY),
        Facing::Up,
        vec!(Op::RotateX),
    ));

    mm.push((
        Point2D(-2, 1),
        vec!(Point2D(0, -1), Point2D(-1, 0), Point2D(-2, 0)),
        Facing::Up,
        vec!(Op::Same),
        Facing::Down,
        vec!(Op::RotateX),
    ));
    mm.push((
        Point2D(-2, 1),
        vec!(Point2D(0, 1), Point2D(-1, 1), Point2D(-2, 2)),
        Facing::Down,
        vec!(Op::RotateY),
        Facing::Up,
        vec!(Op::RotateX),
    ));

    mm.push((
        Point2D(-2, -1),
        vec!(Point2D(0, 1), Point2D(-1, 0), Point2D(-2, 0)),
        Facing::Down,
        vec!(Op::RotateY),
        Facing::Up,
        vec!(Op::RotateX),
    ));
    mm.push((
        Point2D(-2, -1),
        vec!(Point2D(0, -1), Point2D(-1, -1), Point2D(-2, -2)),
        Facing::Up,
        vec!(Op::Same),
        Facing::Down,
        vec!(Op::RotateX),
    ));

    // 3rd?
    mm.push((
        Point2D(-1, 3),
        vec!(Point2D(0, -1), Point2D(-1, 0), Point2D(-2, 2), Point2D(-2, 3)),
        Facing::Up,
        vec!(Op::Same),
        Facing::Right,
        vec!(Op::Flip),
    ));
    mm.push((
        Point2D(1, -3),
        vec!(Point2D(-1, 0), Point2D(-1, -1), Point2D(0, -2), Point2D(0, -3)),
        Facing::Left,
        vec!(Op::Flip),
        Facing::Down,
        vec!(Op::Same),
    ));

    mm.push((
        Point2D(-2, 3),
        vec!(Point2D(0, -1), Point2D(1, 0), Point2D(-1, 3), Point2D(-2, 4)),
        Facing::Up,
        vec!(Op::Same),
        Facing::Up,
        vec!(Op::Same),
    ));
    mm.push((
        Point2D(2, -3),
        vec!(Point2D(0, 1), Point2D(1, 0), Point2D(3, -3), Point2D(2, -4)),
        Facing::Down,
        vec!(Op::RotateY),
        Facing::Down,
        vec!(Op::RotateY),
    ));

    mm
}
