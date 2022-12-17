use std::collections::HashSet;

use itertools::Itertools;

use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let input = parse_input(scope, "puzzle1");
    let mut direction_generator = build_direction_generator(input);
    let mut chamber = Chamber::new(7, 0);
    let mut shape_generator = build_shape_generator();
    let size = tower_height(2022, &mut chamber, &mut shape_generator, &mut direction_generator);
    write_solution(&scope, format!("height = {}", size).as_str());
}

pub fn tower_height(limit: usize, chamber: &mut Chamber, shape_generator: &mut MyGenerator<Shape>, direction_generator: &mut MyGenerator<Direction>) -> usize {
    //println!("{}", chamber.to_string());

    let mut lengths: Vec<usize> = vec![];
    let mut last_length = 0;

    for i in 0..limit {
        let shape = shape_generator.next();
        let mut position = (3, chamber.height + shape.height + 3);

        //println!();
        //println!("{}", "=".repeat(100));
        //println!();

        loop {
            //chamber._with_shape_at(shape.clone(), position);
            //println!("{}", chamber.to_string());
            let dir_type = direction_generator.next();

            match dir_type {
                Direction::Left => {
                    if !chamber.conflicts(&shape, &(position.0 - 1, position.1)) {
                        position = (position.0 - 1, position.1);
                        //println!("left {:?}", position);
                    }
                }
                Direction::Right => {
                    if !chamber.conflicts(&shape, &(position.0 + 1, position.1)) {
                        position = (position.0 + 1, position.1);
                        //println!("right {:?}", position);
                    }
                }
            }

            //chamber._with_shape_at(shape.clone(), position);
            //println!("{}", chamber.to_string());

            if chamber.conflicts(&shape, &(position.0, position.1 - 1)) {
                //println!("fill");
                chamber.fill_at(shape, position);
                break;
            } else {
                position = (position.0, position.1 - 1);
            }

            //println!("down {:?}", position);
        }

        //println!();
        //println!("{}", "=".repeat(100));
        //println!();

        //chamber._remove_shape();
        //println!("{}", chamber.to_string());
        //println!();

        lengths.push(chamber.height - last_length);
        last_length = chamber.height;

        if i > 5000 {
            // ok, that dynamic programming stuff is borred here: https://gist.githubusercontent.com/p-a/bef2f3f58b6907196083adf28fbee636/raw/1909a87cfe119b6f04d65cb63c7d00c01d81a27f/aoc_2022_day17.js
            let (sequence, seq_index) = find_sequence(lengths.as_slice());
            let pattern = find_pattern(sequence);
            let pattern_height: usize = pattern.iter().sum();
            let repetitions = (limit - seq_index) / pattern.len();
            let steps_per_sequence = (limit - seq_index) % pattern.len();
            let x: usize = pattern[0..steps_per_sequence].iter().sum::<usize>() + lengths.as_slice()[0..seq_index].iter().sum::<usize>();

            let res = x + repetitions * pattern_height;
            return res;
        }
    }

    //println!();
    //println!("FINISH");
    //println!("{}", "=".repeat(100));
    //println!();

    //chamber._remove_shape();
    //println!("{}", chamber.to_string());
    //println!();

    chamber.height
}

fn find_sequence(arr: &[usize]) -> (&[usize], usize) {
    let len = arr.len();
    let mut dp: Vec<Vec<isize>> = (0..=len)
        .map(|_| (0..=len)
            .map(|_| 0)
            .collect_vec()
        )
        .collect_vec();
    let mut seq_len = 0;
    let mut index = 0;
    for i in 0..len {
        let a = arr[i];
        for j in i + 2..=len {
            if a == arr[j - 1] && dp[i][j - 1] < (j - i) as isize {
                dp[i + 1][j] = dp[i][j - 1] + 1;
                if dp[i + 1][j] > seq_len {
                    seq_len = dp[i + 1][j];
                    index = index.max(i + 1);
                }
            } else {
                dp[i + 1][j] = 0;
            }
        }
    }
    let from = index as usize - seq_len as usize;
    let to = index as usize;
    let arr2 = &arr[from..to];
    (arr2, from)
}

fn find_pattern(arr: &[usize]) -> &[usize] {
    let mut dp = arr.iter()
        .map(|_| 0isize)
        .collect_vec();
    for i in 1..dp.len() {
        let mut k = dp[i - 1];
        let mut done = false;
        while !done {
            if arr[i] == arr[k as usize] {
                dp[i] = k + 1;
                done = true;
            } else if k == 0 {
                dp[i] = 0;
                done = true;
            } else {
                k = dp[k as usize - 1];
            }
        }
    }
    &arr[0..arr.len() - dp.last().unwrap().clone() as usize]
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

pub struct Shape {
    height: usize,
    width: usize,
    data: Vec<(usize, usize)>,
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        Shape::new(self.height, self.width, self.data.clone())
    }
}

impl Shape {
    pub fn new(height: usize, width: usize, data: Vec<(usize, usize)>) -> Self {
        Self { height, width, data }
    }
    pub fn contains(&self, coord: &(usize, usize)) -> bool {
        self.data.contains(coord)
    }
    pub fn parse_coords(str: &str) -> Vec<(usize, usize)> {
        let split = str.split("\n").collect_vec();
        (0..split.len())
            .into_iter()
            .flat_map(|y| {
                let chars = split[y].chars().collect_vec();
                (0..chars.len())
                    .into_iter()
                    .filter(|x| match chars[*x] {
                        '#' => true,
                        _ => false,
                    })
                    .map(|x| (x, y))
                    .collect_vec()
            })
            .collect_vec()
    }
    pub fn offsets(&self) -> Vec<(usize, usize)> {
        self.data.clone()
    }
}

pub struct MyGenerator<T> {
    items: Vec<T>,
    next: usize,
}

impl<T: Clone> MyGenerator<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items, next: 0 }
    }
    pub fn next(&mut self) -> T {
        let shape = self.items[self.next].clone();
        self.next = (self.next + 1) % self.items.len();
        shape
    }
}

pub fn build_shape_generator() -> MyGenerator<Shape> {
    MyGenerator::new(vec!(
        Shape::new(1, 4, Shape::parse_coords("####")),
        Shape::new(3, 3, Shape::parse_coords(".#.\n###\n.#.")),
        Shape::new(3, 3, Shape::parse_coords("..#\n..#\n###")),
        Shape::new(4, 1, Shape::parse_coords("#\n#\n#\n#")),
        Shape::new(2, 2, Shape::parse_coords("##\n##")),
    ))
}

pub fn build_direction_generator(items: Vec<Direction>) -> MyGenerator<Direction> {
    MyGenerator::new(items)
}

pub struct Chamber {
    width: usize,
    height: usize,
    data: HashSet<(usize, usize)>,
    shape: Option<Shape>,
    shape_at: (usize, usize),
}

impl Chamber {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: HashSet::new(),
            shape: None,
            shape_at: (0, 0),
        }
    }

    pub fn _remove_shape(&mut self) {
        self.shape = None;
    }

    pub fn _with_shape_at(&mut self, shape: Shape, at: (usize, usize)) {
        self.shape = Some(shape);
        self.shape_at = at;
    }

    pub fn conflicts(&self, shape: &Shape, at: &(usize, usize)) -> bool {
        if !(1..=self.width).contains(&at.0) {
            return true;
        }
        if !(1..=self.width).contains(&(at.0 + shape.width - 1)) {
            return true;
        }
        if at.1 < 1 {
            return true;
        }
        if at.1 - (shape.height - 1) > self.height {
            return false;
        }
        let map: Vec<(usize, usize)> = shape.offsets()
            .iter()
            .map(|(x, y)| (*x + at.0, at.1 - *y))
            .collect();
        map.into_iter()
            .any(|(x, y)| {
                self.data.contains(&(x, y))
            })
    }

    pub fn fill_at(&mut self, shape: Shape, at: (usize, usize)) {
        shape.offsets()
            .iter()
            .for_each(|p| {
                self.data.insert((at.0 + p.0, at.1 - p.1));
            });
        self.height = self.data.iter().map(|(_, y)| *y).max().unwrap();
    }
}

impl ToString for Chamber {
    fn to_string(&self) -> String {
        let mut result = "".to_owned();
        let mut height = self.height;
        if let Some(_) = &self.shape {
            height = self.shape_at.1;
        }
        for y in (1..=height).rev() { // reversed!
            result += "|";
            for x in 1..=self.width {
                let mut tmp: Option<&str> = None;
                if let Some(_) = self.data.get(&(x, y)) {
                    tmp = Some("#");
                } else if let Some(shape) = &self.shape {
                    let offset_x = (x as isize) - (self.shape_at.0 as isize);
                    let offset_y = height as isize - y as isize;
                    if offset_y >= 0 && offset_x >= 0 && shape.contains(&(offset_x as usize, offset_y as usize)) {
                        tmp = Some("@");
                    }
                }
                result += tmp.unwrap_or(".");
            }
            result += "|";
            result += "\n";
        }
        result += "+";
        result += "-".repeat(self.width).as_str();
        result += "+\n";
        result
    }
}

pub fn parse_input(scope: &PuzzleScope, puzzle: &str) -> Vec<Direction> {
    io::read_puzzle_first_line(scope.day(), puzzle)
        .chars()
        .into_iter()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!(),
        })
        .collect()
}
