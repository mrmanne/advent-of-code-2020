use crate::puzzle::{io, File, Puzzle};
use std::collections::HashMap;

pub struct Day10;

fn get_nof_difference(numbers: &[usize], difference: usize) -> usize {
    numbers.iter().enumerate().fold(0, |acc, (i, x)| {
        if i > 0 && x - numbers[i - 1] == difference {
            acc + 1
        } else {
            acc
        }
    })
}

fn get_combinations(
    adapters: &[usize],
    prev: usize,
    mut part_results: &mut HashMap<usize, usize>,
) -> usize {
    if adapters[0] > prev + 3 {
        return 0;
    }
    if adapters.len() == 1 {
        return 1;
    }

    let included = match part_results.get(&adapters[0]) {
        Some(result) => *result,
        None => {
            let result = get_combinations(&adapters[1..], adapters[0], &mut part_results);
            part_results.insert(adapters[0], result);
            result
        }
    };

    let excluded = get_combinations(&adapters[1..], prev, &mut part_results);
    included + excluded
}

impl Day10 {
    fn solve_part1(&self, numbers: &mut Vec<usize>) -> usize {
        numbers.sort();
        let max = *numbers.iter().max().unwrap();
        numbers.push(max + 3);
        numbers.insert(0, 0);

        get_nof_difference(&numbers[..], 1) * get_nof_difference(&numbers[..], 3)
    }

    fn solve_part2(&self, numbers: &mut Vec<usize>) -> usize {
        let mut part_results: HashMap<usize, usize> = HashMap::new();
        numbers.sort();
        let max = *numbers.iter().max().unwrap();
        numbers.push(max + 3);

        get_combinations(&numbers[..], 0, &mut part_results)
    }
}

impl Puzzle for Day10 {
    fn solve(&self, lines: io::Lines<io::BufReader<File>>) -> (String, String) {
        let input: Vec<usize> = lines
            .map(|l| l.unwrap().parse::<usize>().unwrap())
            .collect();
        return (
            self.solve_part1(&mut input.clone()).to_string(),
            self.solve_part2(&mut input.clone()).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day10 {}.solve_part1(&mut vec!(16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4)),
            35
        );
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            Day10 {}.solve_part1(&mut vec!(
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            )),
            220
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day10 {}.solve_part2(&mut vec!(16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4)),
            8
        );
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            Day10 {}.solve_part2(&mut vec!(
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            )),
            19208
        );
    }
}
