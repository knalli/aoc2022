use std::cmp::Ordering;

use crate::aoc2022::lib::common::{measure_time_and_print, parse_int, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let packets = parse_input(scope, "puzzle1");

    println!("Packets");
    packets.iter().for_each(|p| {
        println!("{}", p.to_string());
    });

    let mut p = 0;
    let mut idx = 0;
    let mut sum = 0;
    while p < packets.len() {
        let left = packets.get(p).unwrap();
        let right = packets.get(p + 1).unwrap();
        p = p + 2;
        idx = idx + 1;

        if left < right {
            sum = sum + idx;
        }
    }

    write_solution(&scope, format!("sum = {}", sum).as_str());
}

pub enum Packet {
    EMPTY,
    VALUE(i32),
    LIST(Vec<Packet>),
}

impl Eq for Packet {}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Packet::EMPTY => match other {
                Packet::EMPTY => true,
                _ => false
            },
            Packet::VALUE(v) => match other {
                Packet::VALUE(o) => *v == *o,
                _ => false,
            },
            Packet::LIST(v) => match other {
                Packet::LIST(o) => v.eq(o),
                _ => false
            },
        }
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::EMPTY => match other {
                Packet::EMPTY => Ordering::Equal,
                _ => Ordering::Less, // left runs out, right left => OK
            },
            Packet::VALUE(v) => match other {
                Packet::EMPTY => Ordering::Greater, // left left, right runs out => NOT OK
                Packet::VALUE(o) => v.cmp(o),
                Packet::LIST(_) => Packet::LIST(vec!(self.clone())).cmp(other),
            },
            Packet::LIST(v) => match other {
                Packet::EMPTY => Ordering::Greater, // left left, right runs out => NOT OK
                Packet::VALUE(_) => self.cmp(&Packet::LIST(vec!(other.clone()))),
                Packet::LIST(o) => {
                    let max = v.len().max(o.len());
                    for i in 0..max {
                        let result = match v.get(i) {
                            Some(vi) => match o.get(i) {
                                Some(oi) => vi.cmp(oi),
                                None => Ordering::Greater,
                            }
                            None => match o.get(i) {
                                Some(_) => Ordering::Less,
                                None => Ordering::Equal
                            }
                        };
                        if result == Ordering::Greater {
                            return result;
                        } else if result == Ordering::Less {
                            return result;
                        }
                    }
                    Ordering::Equal
                }
            },
        }
    }
}

impl ToString for Packet {
    fn to_string(&self) -> String {
        match self {
            Packet::EMPTY => "[]".to_string(),
            Packet::VALUE(v) => i32::to_string(v),
            Packet::LIST(l) => {
                let list: Vec<String> = l.iter()
                    .map(|p| p.to_string())
                    .collect();
                "[".to_owned() + &list.join(",") + "]"
            }
        }
    }
}

impl Clone for Packet {
    fn clone(&self) -> Self {
        match self {
            Packet::EMPTY => Packet::EMPTY,
            Packet::VALUE(v) => Packet::VALUE(*v),
            Packet::LIST(l) => Packet::LIST(l.clone()),
        }
    }
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Vec<Packet> {
    io::read_puzzle_as_list(scope.day(), puzzle)
        .iter()
        .filter(|s| !s.is_empty())
        .map(|str| {
            let (_, packet) = parse_packet(str, 0);
            packet
        })
        .collect()
}

fn parse_packet(str: &str, offset: usize) -> (usize, Packet) {
    let mut result: Vec<Packet> = vec!();
    let mut idx = offset;
    let chars: Vec<char> = str.chars().collect();
    let mut read_buffer = 0;
    while idx < str.len() {
        let &ch = chars.get(idx).unwrap();
        match ch {
            '[' => {
                let (next, packet) = parse_packet(str, idx + 1);
                idx = next;
                result.push(packet);
            }
            ']' => {
                if read_buffer > 0 {
                    let str: String = chars[idx - read_buffer..idx].iter().collect();
                    // read_buffer = 0;
                    result.push(Packet::VALUE(parse_int(str.as_str())));
                }
                // abort
                return (idx, Packet::LIST(result));
            }
            ',' => {
                if read_buffer > 0 {
                    let str: String = chars[idx - read_buffer..idx].iter().collect();
                    read_buffer = 0;
                    result.push(Packet::VALUE(parse_int(str.as_str())));
                }
            }
            _ => {
                read_buffer = read_buffer + 1;
            }
        }
        idx = idx + 1;
    }

    if read_buffer > 0 {
        let str: String = chars[idx - read_buffer..idx].iter().collect();
        // read_buffer = 0;
        result.push(Packet::VALUE(parse_int(str.as_str())));
    }

    (idx, match result.len() {
        0 => Packet::EMPTY,
        1 => result.first().unwrap().clone(),
        _ => Packet::LIST(result),
    })
}

#[test]
fn test_parse_packet_simple() {
    let (idx, packet) = parse_packet("1,2,3,4,5", 0);
    assert_eq!(9, idx);
    assert_eq!("[1,2,3,4,5]", packet.to_string());
}

#[test]
fn test_parse_packet_empty() {
    let (idx, packet) = parse_packet("", 0);
    assert_eq!(0, idx);
    assert_eq!("[]", packet.to_string());
}

#[test]
fn test_parse_packet_multi_digit() {
    let (idx, packet) = parse_packet("1,23,3,4,50", 0);
    assert_eq!(11, idx);
    assert_eq!("[1,23,3,4,50]", packet.to_string());
}
