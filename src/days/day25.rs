use crate::puzzle::{io, File, Puzzle};

pub struct Day25;

impl Day25 {
    fn solve_part1(&self, input: Vec<String>) -> usize {
        let pub_keys: Vec<usize> = input.iter().map(|x| x.parse::<usize>().unwrap()).collect();
        let mut subject = 7;
        let mut i = 1;
        let mut val = i;
        loop {
            val = val * subject;
            val = val % 20201227;
            if pub_keys[1] == val {
                break;
            }
            i += 1;
        }
        let loop_size = i;
        subject = pub_keys[0];
        i = 1;
        val = i;
        for _ in 0..loop_size {
            val = val * subject;
            val = val % 20201227;
            i += 1;
        }
        val
    }

    fn solve_part2(&self, _input: Vec<String>) -> usize {
        0
    }
}

impl Puzzle for Day25 {
    fn solve(&self, lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let input: Vec<String> = lines.expect("No input file").map(|l| l.unwrap()).collect();
        return (
            self.solve_part1(input.clone()).to_string(),
            self.solve_part2(input.clone()).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! string_vec {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day25 {}.solve_part1(string_vec!("5764801", "17807724")),
            14897079
        );
    }
}
