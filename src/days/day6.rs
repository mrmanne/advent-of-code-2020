use std::vec;

use crate::puzzle::{io, File, Puzzle};
pub struct Day6;

fn get_group_answers(input: &Vec<String>) -> Vec<(usize, [usize; 26])> {
    let mut groups = vec![];
    let mut answers: [usize; 26] = [0; 26];
    let mut group_size = 0;
    for line in input {
        if line.is_empty() {
            groups.push((group_size, answers));
            group_size = 0;
            answers = [0; 26];
        } else {
            group_size += 1;
            for c in line.as_bytes() {
                let pos: usize = (c - ('a' as u8)) as usize;
                answers[pos] += 1;
            }
        }
    }
    if group_size > 0 {
        groups.push((group_size, answers));
    }
    groups
}

impl Day6 {
    fn solve_part1(&self, input: &Vec<String>) -> usize {
        get_group_answers(input)[..]
            .iter()
            .fold(0, |sum, (_, answers)| {
                sum + answers[..]
                    .iter()
                    .fold(0, |sum, x| if *x > 0 { sum + 1 } else { sum })
            })
    }

    fn solve_part2(&self, input: &Vec<String>) -> usize {
        get_group_answers(input)[..]
            .iter()
            .fold(0, |sum, (size, answers)| {
                sum + answers[..]
                    .iter()
                    .fold(0, |sum, x| if *x == *size { sum + 1 } else { sum })
            })
    }
}

impl Puzzle for Day6 {
    fn solve(&self, lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let numbers: Vec<String> = lines.expect("No input file").map(|l| l.unwrap()).collect();
        return (
            self.solve_part1(&numbers).to_string(),
            self.solve_part2(&numbers).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day6 {}.solve_part1(
                &vec!("abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            11
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day6 {}.solve_part2(
                &vec!("abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            6
        );
    }
}
