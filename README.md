# Advent of Code 2020

## Introduction
I'm taking the opportunity to learn a new language, Rust, in this years [Advent of Code](https://adventofcode.com/).

## Prerequisites
Install Rust with [rustup](https://rustup.rs/).

## How to build
```bash
$ cargo build --release
   Compiling advent-of-code-2020 v0.1.0 (/home/erik/src/advent-of-code-2020)
    Finished release [optimized] target(s) in 2.34s
```

## How to run the unit tests
```bash
$ cargo test
   Compiling advent-of-code-2020 v0.1.0 (/home/erik/src/advent-of-code-2020)
    Finished test [unoptimized + debuginfo] target(s) in 0.33s
     Running target/debug/deps/advent_of_code_2020-0d429d52e5b10714

running 17 tests
test days::day1::tests::part1_example1 ... ok
test days::day1::tests::part2_example1 ... ok
test days::day2::tests::part1_example1 ... ok
test days::day4::tests::part2_ecl1 ... ok
test days::day3::tests::part2_example1 ... ok
test days::day4::tests::part2_hcl1 ... ok
test days::day2::tests::part2_example1 ... ok
test days::day4::tests::part2_pid1 ... ok
test days::day4::tests::part2_hgt1 ... ok
test days::day5::tests::part1_example1 ... ok
test days::day4::tests::part2_invalid_examples ... ok
test days::day5::tests::part1_example2 ... ok
test days::day4::tests::part2_valid_examples ... ok
test days::day6::tests::part1_example1 ... ok
test days::day3::tests::part1_example1 ... ok
test days::day4::tests::part1_example1 ... ok
test days::day6::tests::part2_example1 ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## How to solve the puzzle for day 1
```bash
$ cargo run --release 1
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/advent-of-code-2020 1`
First answer found: 1007104
Second answer found: 18847752
Execution time: 0 ms
```
