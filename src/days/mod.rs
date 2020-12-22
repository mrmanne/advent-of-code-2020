use crate::puzzle::Puzzle;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn puzzle_factory(day: u8) -> Option<Box<dyn Puzzle>> {
    match day {
        1 => Some(Box::new(day1::Day1 {})),
        2 => Some(Box::new(day2::Day2 {})),
        3 => Some(Box::new(day3::Day3 {})),
        4 => Some(Box::new(day4::Day4 {})),
        5 => Some(Box::new(day5::Day5 {})),
        6 => Some(Box::new(day6::Day6 {})),
        7 => Some(Box::new(day7::Day7 {})),
        8 => Some(Box::new(day8::Day8 {})),
        9 => Some(Box::new(day9::Day9 {})),
        10 => Some(Box::new(day10::Day10 {})),
        11 => Some(Box::new(day11::Day11 {})),
        12 => Some(Box::new(day12::Day12 {})),
        13 => Some(Box::new(day13::Day13 {})),
        14 => Some(Box::new(day14::Day14 {})),
        15 => Some(Box::new(day15::Day15 {})),
        16 => Some(Box::new(day16::Day16 {})),
        17 => Some(Box::new(day17::Day17 {})),
        18 => Some(Box::new(day18::Day18 {})),
        19 => Some(Box::new(day19::Day19 {})),
        20 => Some(Box::new(day20::Day20 {})),
        21 => Some(Box::new(day21::Day21 {})),
        22 => Some(Box::new(day22::Day22 {})),
        _ => None,
    }
}
