use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Open,
    Solid,
}

pub struct Board {
    pub data: HashMap<Point2D, Tile>,
    pub height: isize,
    pub width: isize,
}

impl Board {
    pub fn new(data: HashMap<Point2D, Tile>) -> Self {
        let height = data.iter().map(|(Point2D(_, y), _)| *y).max().unwrap();
        let width = data.iter().map(|(Point2D(x, _), _)| *x).max().unwrap();
        Self { data, height, width }
    }
    pub fn iter(&self) -> Iter<'_, Point2D, Tile> {
        self.data.iter()
    }

    pub fn top_left(&self) -> Point2D {
        let (top_left, _) = self.iter()
            .sorted_by_key(|(Point2D(x, y), _)| {
                (self.width * (*y - 1)) + *x
            })
            .enumerate()
            .filter(|(i, _)| i == &0)
            .map(|(_, a)| a)
            .next()
            .unwrap();
        top_left.clone()
    }

    pub fn find_outer_next_up(&self, from: &Point2D) -> Option<Point2D> {
        for y in (1..=from.1).rev() {
            let point = Point2D(from.0, y);
            match self.data.get(&point) {
                Some(_) => {
                    return Some(point);
                }
                None => ()
            };
        }
        None
    }

    pub fn find_outer_next_down(&self, from: &Point2D) -> Option<Point2D> {
        for y in from.1..=self.height {
            let point = Point2D(from.0, y);
            match self.data.get(&point) {
                Some(_) => {
                    return Some(point);
                }
                None => ()
            };
        }
        None
    }

    pub fn find_outer_next_left(&self, from: &Point2D) -> Option<Point2D> {
        for x in (1..=from.0).rev() {
            let point = Point2D(x, from.1);
            match self.data.get(&point) {
                Some(_) => {
                    return Some(point);
                }
                None => ()
            };
        }
        None
    }

    pub fn find_outer_next_right(&self, from: &Point2D) -> Option<Point2D> {
        for x in from.0..=self.width {
            let point = Point2D(x, from.1);
            match self.data.get(&point) {
                Some(_) => {
                    return Some(point);
                }
                None => ()
            };
        }
        None
    }
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => { data.insert(Point2D(x as isize + 1, y as isize + 1), Tile::Open); }
                    '#' => { data.insert(Point2D(x as isize + 1, y as isize + 1), Tile::Solid); }
                    _ => (),
                }
            }
        }
        Ok(Board::new(data))
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point2D(pub isize, pub isize);

impl Add for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D(
            self.0 + rhs.0,
            self.1 + rhs.1,
        )
    }
}