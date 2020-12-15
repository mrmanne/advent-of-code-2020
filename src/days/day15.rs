use std::collections::HashMap;
use std::vec;

use crate::puzzle::{io, File, Puzzle};
pub struct Day15;

fn last_spoken(input: &Vec<usize>, nof_turns: usize) -> usize {
    let mut numbers = HashMap::new();
    let mut starting_no: usize = 1;
    let mut last_spoken: usize = 0;
    for n in input {
        if starting_no > 1 {
            numbers.insert(last_spoken, starting_no - 1);
        }
        last_spoken = *n;
        starting_no += 1;
    }
    for turn in starting_no..nof_turns + 1 {
        if let Some(prev_turn) = numbers.insert(last_spoken, turn - 1) {
            last_spoken = turn - 1 - prev_turn;
        } else {
            last_spoken = 0;
        }
    }
    last_spoken
}

impl Day15 {
    fn solve_part1(&self, input: Vec<usize>) -> usize {
        last_spoken(&input, 2020)
    }

    fn solve_part2(&self, input: Vec<usize>) -> usize {
        last_spoken(&input, 30000000)
    }
}

impl Puzzle for Day15 {
    fn solve(&self, _lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let input = vec![1, 0, 18, 10, 19, 6];
        return (
            self.solve_part1(input.clone()).to_string(),
            self.solve_part2(input.clone()).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(Day15 {}.solve_part1(vec!(0, 3, 6)), 436);
    }

    // Disable this unit test since its too slow to run for every build.
    // #[test]
    // fn part2_example1() {
    //     assert_eq!(
    //         Day15 {}.solve_part2(vec!(0,3,6)), 175594);
    // }
}
