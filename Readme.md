# Advent of Code 2022 Solutions
Welcome to my solutions of [Advent Of Code](http://adventofcode.com) 2022 (AOC 2022).

A huge thanks to @topaz and his team for providing this great service.

After three years using Go, it is time for a new one. Said that, 2022 is my year of Rust.
Hope, the solutions are not so rusty after all... 👴

## Disclaimer
These are my personal solutions of the Advent Of Code (AOC). The code is
*not indented* to be perfect in any kind of area. This year, my personal
competition was to learn the Rust language. These snippets are here for everyone
learning more, too.

If you think, there is a piece of improvement: Go to the code,
fill a PR, and we are all happy. Share the knowledge.

## Structure
The AOC contains 25 days with at least one puzzle/question per day (mostly there are two parts).

* Base path is the root folder.
* Each day is a submodule named `day01`, `day02` until `day25` with files `part1.rs` & `rust2.rs` having
  a function `run(PuzzleScope)`.
* A new day will be created by invoking `./create_day.sh <day>`.
* Depending on content, a day could import (exported) symbols of a (previous) day.

## Usage

For running the day `day00`
* CLI: just enter `DAY=0 PART=1 cargo run --package aoc2022 -bin aoc2022`

## License / Copyright
Everything is free for all.

Licensed under MIT. Copyright Jan Philipp.