use std::collections::HashMap;

use crate::aoc2022::lib::common::PuzzleScope;

#[allow(unreachable_patterns, unreachable_code)]
pub enum Day {
    D00,
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    //GEN_DAY_ITEM
}

impl Clone for Day {
    fn clone(&self) -> Self {
        Day::from_str(self.as_str())
            .expect("invalid day")
    }
}

impl Copy for Day {}

#[allow(unreachable_patterns, unreachable_code)]
impl Day {
    pub fn as_str(&self) -> String {
        match self {
            Day::D00 => String::from("d00"),
            Day::D01 => String::from("d01"),
            Day::D02 => String::from("d02"),
            Day::D03 => String::from("d03"),
            Day::D04 => String::from("d04"),
            Day::D05 => String::from("d05"),
            Day::D06 => String::from("d06"),
            Day::D07 => String::from("d07"),
            Day::D08 => String::from("d08"),
            Day::D09 => String::from("d09"),
            Day::D10 => String::from("d10"),
            Day::D11 => String::from("d11"),
            Day::D12 => String::from("d12"),
            Day::D13 => String::from("d13"),
            Day::D14 => String::from("d14"),
            Day::D15 => String::from("d15"),
            //GEN_DAY_STR
            _ => panic!("invalid day")
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            Day::D00 => 0,
            Day::D01 => 1,
            Day::D02 => 2,
            Day::D03 => 3,
            Day::D04 => 4,
            Day::D05 => 5,
            Day::D06 => 6,
            Day::D07 => 7,
            Day::D08 => 8,
            Day::D09 => 9,
            Day::D10 => 10,
            Day::D11 => 11,
            Day::D12 => 12,
            Day::D13 => 13,
            Day::D14 => 14,
            Day::D15 => 15,
            //GEN_DAY_INT
            _ => panic!("invalid day")
        }
    }

    pub fn from_str(str: String) -> Result<Day, String> {
        match str.to_lowercase().as_str() {
            "d00" => Ok(Day::D00),
            "d01" => Ok(Day::D01),
            "d02" => Ok(Day::D02),
            "d03" => Ok(Day::D03),
            "d04" => Ok(Day::D04),
            "d05" => Ok(Day::D05),
            "d06" => Ok(Day::D06),
            "d07" => Ok(Day::D07),
            "d08" => Ok(Day::D08),
            "d09" => Ok(Day::D09),
            "d10" => Ok(Day::D10),
            "d11" => Ok(Day::D11),
            "d12" => Ok(Day::D12),
            "d13" => Ok(Day::D13),
            "d14" => Ok(Day::D14),
            "d15" => Ok(Day::D15),
            //GEN_DAY_PARSE
            _ => Err("invalid day".to_string())
        }
    }
}

#[allow(unreachable_patterns, unreachable_code)]
pub enum Part {
    P00,
    P01,
    P02,
}

impl Clone for Part {
    fn clone(&self) -> Self {
        Part::from_str(self.as_str())
            .expect("invalid part")
    }
}

impl Copy for Part {}

#[allow(unreachable_patterns, unreachable_code)]
impl Part {
    pub fn as_str(&self) -> String {
        match self {
            Part::P00 => String::from("p00"),
            Part::P01 => String::from("p01"),
            Part::P02 => String::from("p02"),
            _ => panic!("invalid part")
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            Part::P00 => 0,
            Part::P01 => 1,
            Part::P02 => 2,
            _ => panic!("invalid part")
        }
    }

    pub fn from_str(str: String) -> Result<Part, String> {
        match str.to_lowercase().as_str() {
            "p00" => Ok(Part::P00),
            "p01" => Ok(Part::P01),
            "p02" => Ok(Part::P02),
            _ => Err("invalid part".to_string())
        }
    }
}

pub struct Hive {
    data: HashMap<String, fn(PuzzleScope) -> ()>,
}

impl Hive {
    pub fn create() -> Hive {
        Hive {
            data: HashMap::new(),
        }
    }

    pub fn register(&mut self, day: Day, part: Part, cb: fn(PuzzleScope) -> ()) {
        let key = format!("{}_{}", day.as_str(), part.as_str());
        self.data.insert(key, cb);
    }

    pub fn lookup(&self, day: Day, part: Part) -> Option<&fn(PuzzleScope) -> ()> {
        let key = format!("{}_{}", day.as_str(), part.as_str());
        self.data.get(key.as_str())
    }
}

