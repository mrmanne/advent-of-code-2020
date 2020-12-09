use crate::puzzle::{io, File, Puzzle};
pub struct Day9;

fn valid(preamble: &[usize], num: usize) -> bool {
    for i in 0..preamble.len() {
        for j in 0..preamble.len() {
            if (i != j) && ((preamble[i] + preamble[j]) == num) {
                return true;
            }
        }
    }
    false
}

impl Day9 {
    fn solve_part1(&self, numbers: &Vec<usize>, preamble_len: usize) -> usize {
        let offset = preamble_len;
        for i in offset..numbers.len() {
            if !valid(&numbers[i - preamble_len..i], numbers[i]) {
                return numbers[i];
            }
        }
        0
    }

    fn solve_part2(&self, numbers: &Vec<usize>, preamble_len: usize) -> usize {
        let invalid_no = self.solve_part1(numbers, preamble_len);
        for i in 0..numbers.len() {
            let mut sum: usize = 0;
            let mut min: usize = std::usize::MAX;
            let mut max: usize = std::usize::MIN;
            for j in i..numbers.len() {
                if numbers[j] < min {
                    min = numbers[j];
                }
                if numbers[j] > max {
                    max = numbers[j];
                }
                sum += numbers[j];
                if sum == invalid_no {
                    return min + max;
                }
                if sum > invalid_no {
                    break;
                }
            }
        }
        0
    }
}

impl Puzzle for Day9 {
    fn solve(&self, lines: io::Lines<io::BufReader<File>>) -> (String, String) {
        let numbers: Vec<usize> = lines
            .map(|l| l.unwrap().parse::<usize>().unwrap())
            .collect();
        return (
            self.solve_part1(&numbers, 25).to_string(),
            self.solve_part2(&numbers, 25).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day9 {}.solve_part1(
                &vec!(
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576,
                ),
                5
            ),
            127
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day9 {}.solve_part2(
                &vec!(
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576,
                ),
                5
            ),
            62
        );
    }
}
