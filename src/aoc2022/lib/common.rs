use std::time::Instant;

pub struct PuzzleScope {
    year: i32,
    day: i32,
    part: i32,
}

impl Clone for PuzzleScope {
    fn clone(&self) -> Self {
        PuzzleScope {
            year: self.year,
            day: self.day,
            part: self.part,
        }
    }
}

impl PuzzleScope {
    pub fn create(year: i32, day: i32, part: i32) -> PuzzleScope {
        PuzzleScope {
            year,
            day,
            part,
        }
    }

    pub fn year(&self) -> i32 {
        self.year
    }
    pub fn day(&self) -> i32 {
        self.day
    }
    pub fn part(&self) -> i32 {
        self.part
    }
}

pub fn parse_int(s: &str) -> i32 {
    let x: i32 = s.parse().unwrap();
    x
}

pub fn measure_time(scope: &PuzzleScope, f: fn(&PuzzleScope) ) -> u128 {
    let now = Instant::now();
    f(scope);
    now.elapsed().as_millis()
}

pub fn measure_time_and_print(scope: &PuzzleScope, f: fn(&PuzzleScope) ) {
    let elapsed_ms = measure_time(scope, f);
    println!("Elapsed time: {elapsed_ms}ms")
}