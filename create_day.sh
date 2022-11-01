#!/usr/bin/env bash

FMT_RED="\033[0;31m"
FMT_GREEN="\033[0;32m"
FMT_RESET="\033[0m"

function errecho() {
  echo >&2 -ne "${FMT_RED}"
  echo >&2 -n "$@"
  echo >&2 -e "${FMT_RESET}"
}

function okecho() {
  echo >&2 -ne "${FMT_GREEN}"
  echo >&2 -n "$@"
  echo >&2 -e "${FMT_RESET}"
}

function initDay() {
  local day
  local id
  day="$1"
  id=$(printf "%02d" "$day")
  if [ -d "day${id}" ]; then
    errecho "day directory already exist"
    return 1
  fi
  if [ -r "day${id}/init.go" ]; then
    errecho "day file already exist"
    return 1
  fi
  mkdir -p "src/aoc2022/day${id}"
  cat <<EOF >"src/aoc2022/day${id}/part1.rs"
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let ints = io::read_puzzle_as_ints(scope.day(), "puzzle1");
    let sum: i32 = ints.iter().sum();
    write_solution(&scope, format!("sum = {}", sum).as_str());
}
EOF

  cat <<EOF >"src/aoc2022/day${id}/part2.rs"
use crate::aoc2022::lib::common::{measure_time_and_print, PuzzleScope};
use crate::aoc2022::lib::io;
use crate::aoc2022::lib::style::{write_header, write_solution};

pub fn run(scope: PuzzleScope) {
    write_header(&scope);
    measure_time_and_print(&scope, execute);
}

fn execute(scope: &PuzzleScope) {
    let ints = io::read_puzzle_as_ints(scope.day(), "puzzle2");
    let sum: i32 = ints.iter().sum();
    write_solution(&scope, format!("sum = {}", sum).as_str());
}
EOF

  cat <<EOF >"src/aoc2022/day${id}.rs"
pub mod part1;
pub mod part2;
EOF

  cat <<EOF >>"src/aoc2022.rs"
pub mod day${id};
EOF

  # GEN_HIVE_REGISTER >> hive.register(Day::D00, Part::P01, aoc2022::day00::part1::run);
  ESCAPED_REPLACE=$(printf '%s\n' "hive.register(Day::D${id}, Part::P01, aoc2022::day${id}::part1::run);" | sed -e 's/[\/&]/\\&/g')
  if [[ "$OSTYPE" == "darwin"* ]] || [[ "$OSTYPE" == "freebsd"* ]]; then
      sed -i '' -E "s#^(.*)//GEN_HIVE_REGISTER#\1${ESCAPED_REPLACE}\n\1//GEN_HIVE_REGISTER#g" src/main.rs
    else
      sed -i -E "s#^(.*)//GEN_HIVE_REGISTER#\1${ESCAPED_REPLACE}\n\1//GEN_HIVE_REGISTER#g" src/main.rs
    fi
  ESCAPED_REPLACE=$(printf '%s\n' "hive.register(Day::D${id}, Part::P02, aoc2022::day${id}::part2::run);" | sed -e 's/[\/&]/\\&/g')
  if [[ "$OSTYPE" == "darwin"* ]] || [[ "$OSTYPE" == "freebsd"* ]]; then
    sed -i '' -E "s#^(.*)//GEN_HIVE_REGISTER#\1${ESCAPED_REPLACE}\n\1//GEN_HIVE_REGISTER#g" src/main.rs
  else
    sed -i -E "s#^(.*)//GEN_HIVE_REGISTER#\1${ESCAPED_REPLACE}\n\1//GEN_HIVE_REGISTER#g" src/main.rs
  fi

  # GEN_DAY_ITEM >> D01,
  ESCAPED_REPLACE=$(printf '%s\n' "D${id}," | sed -e 's/[\/&]/\\&/g')
  if [[ "$OSTYPE" == "darwin"* ]] || [[ "$OSTYPE" == "freebsd"* ]]; then
    sed -i '' -E "s#^(.*)//GEN_DAY_ITEM#\1${ESCAPED_REPLACE}\n\1//GEN_DAY_ITEM#g" src/aoc2022/hive.rs
  else
    sed -i -E "s#^(.*)//GEN_DAY_ITEM#\1${ESCAPED_REPLACE}\n\1//GEN_DAY_ITEM#g" src/aoc2022/hive.rs
  fi

  # GEN_DAY_STR >> Day::D01 => String::from("d01"),
  ESCAPED_REPLACE=$(printf '%s\n' "Day::D${id} => String::from(\"d${id}\")," | sed -e 's/[\/&]/\\&/g')
  if [[ "$OSTYPE" == "darwin"* ]] || [[ "$OSTYPE" == "freebsd"* ]]; then
    sed -i '' -E "s#^(.*)//GEN_DAY_STR#\1${ESCAPED_REPLACE}\n\1//GEN_DAY_STR#g" src/aoc2022/hive.rs
  else
    sed -i -E "s#^(.*)//GEN_DAY_STR#\1${ESCAPED_REPLACE}\n\1//GEN_DAY_STR#g" src/aoc2022/hive.rs
  fi

  # GEN_DAY_INT >> Day::D01 => 1,
  ESCAPED_REPLACE=$(printf '%s\n' "Day::D${id} => ${day}," | sed -e 's/[\/&]/\\&/g')
  if [[ "$OSTYPE" == "darwin"* ]] || [[ "$OSTYPE" == "freebsd"* ]]; then
    sed -i '' -E "s#^(.*)//GEN_DAY_INT#\1${ESCAPED_REPLACE}\n\1//GEN_DAY_INT#g" src/aoc2022/hive.rs
  else
    sed -i -E "s#^(.*)//GEN_DAY_INT#\1${ESCAPED_REPLACE}\n\1//GEN_DAY_INT#g" src/aoc2022/hive.rs
  fi

  # GEN_DAY_PARSE >> "d01" => Ok(Day::D01),
  ESCAPED_REPLACE=$(printf '%s\n' "\"d${id}\" => Ok(Day::D${id})," | sed -e 's/[\/&]/\\&/g')
  if [[ "$OSTYPE" == "darwin"* ]] || [[ "$OSTYPE" == "freebsd"* ]]; then
    sed -i '' -E "s#^(.*)//GEN_DAY_PARSE#\1${ESCAPED_REPLACE}\n\1//GEN_DAY_PARSE#g" src/aoc2022/hive.rs
  else
    sed -i -E "s#^(.*)//GEN_DAY_PARSE#\1${ESCAPED_REPLACE}\n\1//GEN_DAY_PARSE#g" src/aoc2022/hive.rs
  fi

}

if ! initDay "$1"; then
  errecho "Failed initializing day"
  exit 1
fi