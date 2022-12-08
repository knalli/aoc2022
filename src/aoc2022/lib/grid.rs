use std::collections::HashMap;

use crate::aoc2022::lib::point::Point2D;

#[derive(Debug)]
pub struct DynGrid2D {
    height: i32,
    width: i32,
    values: HashMap<(i32, i32), Cell>,
}

impl DynGrid2D {
    pub fn create(height: i32, width: i32) -> DynGrid2D {
        return DynGrid2D {
            height,
            width,
            values: HashMap::new(),
        };
    }

    pub fn to_string(&self) -> String {
        let mut res: String = "".to_string();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.get(&(x, y)) {
                    match cell {
                        Cell::Bool(v) => res = res + (if v { "Y" } else { "N" }),
                        Cell::Int(v) => res = res + &*v.to_string(),
                        Cell::Text(v) => res = res + &*v,
                        Cell::Char(c) => res = res + &*String::from(c),
                        _ => res = res + " ",
                    }
                } else {
                    res = res + " ";
                }
            }
            res = res + "\n"
        }
        res
    }

    pub fn height(&self) -> i32 { self.height }

    pub fn width(&self) -> i32 { self.width }

    pub fn set(&mut self, pos: (i32, i32), value: Cell) {
        self.values.insert(pos, value);
    }

    pub fn get(&self, pos: &(i32, i32)) -> Option<Cell> {
        self.values.get(pos).cloned()
    }
}

#[derive(Debug)]
pub enum Cell {
    None,
    Bool(bool),
    Text(String),
    Char(char),
    Float(f64),
    Int(i32),
}

impl Cell {
    pub fn text(s: &str) -> Cell {
        Cell::Text(String::from(s))
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        match &self {
            Cell::Bool(v) => Cell::Bool(*v),
            Cell::Text(v) => Cell::Text(String::from(v)),
            Cell::Char(v) => Cell::Char(*v),
            Cell::Int(v) => Cell::Int(*v),
            Cell::Float(v) => Cell::Float(*v),
            _ => Cell::None,
        }
    }
}

//

#[derive(Debug)]
pub struct Grid2D {
    height: i32,
    width: i32,
    values: HashMap<(i32, i32), GridValue>,
}

impl Grid2D {
    pub fn create(height: i32, width: i32) -> Grid2D {
        let values: HashMap<(i32, i32), GridValue> = HashMap::with_capacity((height * width) as usize);
        Grid2D {
            height,
            width,
            values,
        }
    }

    pub fn size(&self) -> i32 {
        return self.height * self.width;
    }


    pub fn get(&self, x: i32, y: i32) -> Option<&GridValue> {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            //self.values.entry((x, y)).or_insert(GridValue::None);
            if let Some(found) = self.values.get(&(x, y)) {
                Some(found)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set(&mut self, x: i32, y: i32, value: GridValue) {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            //self.values.entry((x, y)).or_insert(GridValue::None);
            self.values.insert((x, y), value);
        }
    }

    pub fn each(&self) -> Vec<(Point2D, Option<&GridValue>)> {
        let mut result = Vec::with_capacity(self.size() as usize);
        let w = self.width;
        let h = self.height;
        for y in 0..h {
            for x in 0..w {
                result.push((Point2D::create(x, y), self.get(x, y)));
            }
        }
        return result;
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn width(&self) -> i32 {
        self.width
    }
}

#[derive(Debug)]
pub enum GridValue {
    None,
    Int(i32),
    Text(String),
}