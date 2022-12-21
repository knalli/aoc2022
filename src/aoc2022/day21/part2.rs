use std::collections::HashMap;

use crate::aoc2022::day21::part1::{Dependency, Monkey, parse_input};
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    // path to humn -> for reversed
    // if multiple -> D&C
    let input = patch_root(parse_input(scope, "puzzle1"));
    let mut map: HashMap<String, Monkey> = HashMap::new();
    input.into_iter()
        .for_each(|monkey| {
            map.insert(monkey.name.clone(), monkey);
        });

    // too high 9780869100463

    let humn = find_r(&mut map, "root");
    write_solution(&scope, format!("number = {:?}", humn).as_str());
}

fn compute_reverse_ops(monkeys: &HashMap<String, Monkey>) -> HashMap<String, Monkey> {
    let mut result: HashMap<String, Monkey> = HashMap::new();
    monkeys.values()
        .filter(|m| m.number.is_some() && m.name != "humn")
        .for_each(|monkey| {
            result.insert(monkey.name.clone(), Monkey::new(
                monkey.name.clone(),
                monkey.number.clone(),
                None,
            ));
        });
    monkeys.values()
        .filter(|m| m.dependency.is_some())
        .for_each(|monkey| {
            if let Some(d) = monkey.dependency.clone() {
                match d {
                    Dependency::Add(a, b) => {
                        result.entry(a.clone()).or_insert(Monkey::new(
                            a.clone(),
                            None,
                            Some(Dependency::Sub(monkey.name.clone(), b.clone())),
                        ));
                        result.entry(b.clone()).or_insert(Monkey::new(
                            b.clone(),
                            None,
                            Some(Dependency::Sub(monkey.name.clone(), a.clone())),
                        ));
                    }
                    // Here we go..
                    // 4 = x - 2 => x = 4 + 2
                    // 4 = 6 - x => x = 6 - 4
                    Dependency::Sub(a, b) => {
                        result.entry(a.clone()).or_insert(Monkey::new(
                            a.clone(),
                            None,
                            Some(Dependency::Add(monkey.name.clone(), b.clone())),
                        ));
                        result.entry(b.clone()).or_insert(Monkey::new(
                            b.clone(),
                            None,
                            Some(Dependency::Sub(a.clone(), monkey.name.clone())),
                        ));
                    }
                    Dependency::Mul(a, b) => {
                        result.entry(a.clone()).or_insert(Monkey::new(
                            a.clone(),
                            None,
                            Some(Dependency::Div(monkey.name.clone(), b.clone())),
                        ));
                        result.entry(b.clone()).or_insert(Monkey::new(
                            b.clone(),
                            None,
                            Some(Dependency::Div(monkey.name.clone(), a.clone())),
                        ));
                    }
                    Dependency::Div(a, b) => {
                        result.entry(a.clone()).or_insert(Monkey::new(
                            a.clone(),
                            None,
                            Some(Dependency::Mul(monkey.name.clone(), b.clone())),
                        ));
                        // Here we go..
                        // 4 = x / 2 => x = 4 * 2
                        // 4 = 8 / x => x = 8 / 4
                        result.entry(b.clone()).or_insert(Monkey::new(
                            b.clone(),
                            None,
                            Some(Dependency::Mul(a.clone(), monkey.name.clone())),
                        ));
                    }
                    Dependency::Eq(_, _) => {}
                }
            }
        });
    result
}

fn find_r(monkeys: &mut HashMap<String, Monkey>, current: &str) -> Option<isize> {
    if !monkeys.contains_key(current) {
        return None;
    }
    let monkey = &monkeys[current];

    let sub_find = |monkeys: &mut HashMap<String, Monkey>, a: &str, b: &str| -> (Option<isize>, Option<isize>) {
        let v_a = find_r(monkeys, a);
        let v_b = find_r(monkeys, b);
        (v_a, v_b)
    };

    if let Some(n) = monkey.number {
        Some(n)
    } else if let Some(d) = monkey.dependency.clone() {
        match d {
            Dependency::Add(a, b) => {
                let sub = sub_find(monkeys, &a, &b);
                if sub.0.is_some() && sub.1.is_some() {
                    let v = sub.0.unwrap() + sub.1.unwrap();
                    monkeys.insert(current.to_string(), Monkey::new_by_value(current.to_string(), v));
                    return Some(v);
                } else if sub.0.is_some() {
                    println!("Add: A={:?} B = {:?}", sub.0, sub.1);
                    return None;
                } else if sub.1.is_some() {
                    println!("Add: A={:?} B = {:?}", sub.0, sub.1);
                    return None;
                } else {
                    unreachable!();
                }
            }
            Dependency::Sub(a, b) => {
                let sub = sub_find(monkeys, &a, &b);
                if sub.0.is_some() && sub.1.is_some() {
                    let v = sub.0.unwrap() - sub.1.unwrap();
                    monkeys.insert(current.to_string(), Monkey::new_by_value(current.to_string(), v));
                    return Some(v);
                } else if sub.0.is_some() {
                    println!("Sub: A={:?} B = {:?}", sub.0, sub.1);
                    return None;
                } else if sub.1.is_some() {
                    println!("Sub: A={:?} B = {:?}", sub.0, sub.1);
                    return None;
                } else {
                    unreachable!();
                }
            }
            Dependency::Mul(a, b) => {
                let sub = sub_find(monkeys, &a, &b);
                if sub.0.is_some() && sub.1.is_some() {
                    let v = sub.0.unwrap() * sub.1.unwrap();
                    monkeys.insert(current.to_string(), Monkey::new_by_value(current.to_string(), v));
                    return Some(v);
                } else if sub.0.is_some() {
                    println!("Mul: A={:?} B = {:?}", sub.0, sub.1);
                    return None;
                } else if sub.1.is_some() {
                    println!("Mul: A={:?} B = {:?}", sub.0, sub.1);
                    return None;
                } else {
                    unreachable!();
                }
            }
            Dependency::Div(a, b) => {
                let sub = sub_find(monkeys, &a, &b);
                if sub.0.is_some() && sub.1.is_some() {
                    let v = sub.0.unwrap() / sub.1.unwrap();
                    monkeys.insert(current.to_string(), Monkey::new_by_value(current.to_string(), v));
                    return Some(v);
                } else if sub.0.is_some() {
                    println!("Div: A={:?} B = {:?}", sub.0, sub.1);
                    return None;
                } else if sub.1.is_some() {
                    println!("Div: A={:?} B = {:?}", sub.0, sub.1);
                    return None;
                } else {
                    unreachable!();
                }
            }
            Dependency::Eq(a, b) => {
                let sub = sub_find(monkeys, &a, &b);
                if sub.0.is_some() && sub.1.is_some() {
                    return None;
                } else if let Some(t) = sub.0 {
                    // right contains humn
                    let mut reverse = compute_reverse_ops(&monkeys);
                    reverse.insert(a.clone(), Monkey::new_by_value(
                        a.clone(),
                        t,
                    ));
                    reverse.insert(b.clone(), Monkey::new_by_value(
                        b.clone(),
                        t,
                    ));
                    return find_r(&mut reverse, "humn");
                } else if let Some(t) = sub.1 {
                    let mut reverse = compute_reverse_ops(&monkeys);
                    // left contains humn
                    reverse.insert(a.clone(), Monkey::new_by_value(
                        a.clone(),
                        t,
                    ));
                    reverse.insert(b.clone(), Monkey::new_by_value(
                        b.clone(),
                        t,
                    ));
                    return find_r(&mut reverse, "humn");
                } else {
                    unreachable!();
                }
            }
        }
    } else {
        None
    }
}

fn patch_root(origin: Vec<Monkey>) -> Vec<Monkey> {
    let mut target: Vec<Monkey> = vec![];
    // patch
    for m in origin {
        if m.name == "root" {
            match &m.dependency {
                Some(Dependency::Add(a, b)) => {
                    target.push(Monkey {
                        name: m.name,
                        number: None,
                        dependency: Some(Dependency::Eq(a.clone(), b.clone())),
                    });
                }
                _ => unreachable!()
            }
        } else if m.name == "humn" {
            target.push(Monkey {
                name: m.name,
                number: None,
                dependency: None,
            });
        } else {
            target.push(m);
        }
    }
    target
}

