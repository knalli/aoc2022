use std::str::FromStr;

use anyhow::{Error, Result};
use regex::Regex;

pub enum Direction {
    Left(i32),
    Right(i32),
}

impl Direction {
    pub fn read_multiple(s: &str) -> Result<Vec<Self>> {
        let mut result: Vec<Direction> = vec!();
        let re = Regex::new(r"[LR]\d+").unwrap();
        for x in re.find_iter(s) {
            result.push(x.as_str().parse()?);
        }
        Ok(result)
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let num: i32 = s[1..].parse().expect(format!("invalid number value for direction '{s}'").as_str());
        match s.chars().next() {
            Some('L') => Ok(Direction::Left(num)),
            Some('R') => Ok(Direction::Right(num)),
            _ => Err(Error::msg("invalid direction")),
        }
    }
}
