use crate::puzzle::{io, File, Puzzle};
pub struct Day2;

impl Day2 {
    fn solve_part1(&self, input: &Vec<String>) -> i64 {
        let mut nof_ok_passwords: i64 = 0;
        for line in input {
            let mut tokens = line.split(" ");
            let mut range = tokens.next().unwrap().split("-");
            let min = range.next().unwrap().parse::<usize>().unwrap();
            let max = range.next().unwrap().parse::<usize>().unwrap();
            let c = tokens.next().unwrap().as_bytes()[0];
            let pwd = tokens.next().unwrap();
            let count = pwd.as_bytes().iter().filter(|x| **x == c).count();
            if count >= min && count <= max {
                nof_ok_passwords += 1;
            }
        }
        nof_ok_passwords
    }

    fn solve_part2(&self, input: &Vec<String>) -> i64 {
        let mut nof_ok_passwords: i64 = 0;
        for line in input {
            let mut tokens = line.split(" ");
            let mut indexes = tokens.next().unwrap().split("-");
            let index1 = indexes.next().unwrap().parse::<usize>().unwrap() - 1;
            let index2 = indexes.next().unwrap().parse::<usize>().unwrap() - 1;
            let c = tokens.next().unwrap().as_bytes()[0];
            let pwd = tokens.next().unwrap();
            let mut count = 0;
            if pwd.as_bytes()[index1] == c {
                count += 1;
            }
            if pwd.as_bytes()[index2] == c {
                count += 1;
            }
            if count == 1 {
                nof_ok_passwords += 1;
            }
        }
        nof_ok_passwords
    }
}

impl Puzzle for Day2 {
    fn solve(&self, lines: io::Lines<io::BufReader<File>>) -> (String, String) {
        let input: Vec<String> = lines.map(|l| l.unwrap()).collect();
        return (
            self.solve_part1(&input).to_string(),
            self.solve_part2(&input).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day2 {}.solve_part1(
                &vec!("1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            2
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day2 {}.solve_part2(
                &vec!("1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            1
        );
    }
}
