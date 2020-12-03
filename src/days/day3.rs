use crate::puzzle::{io, File, Puzzle};
pub struct Day3;

fn get_trees(input: &Vec<String>, step_right: usize, step_down: usize) -> usize {
    let mut trees = 0;
    for (i, line) in input.iter().enumerate() {
        if i != 0 && (i % step_down) == 0 {
            if line
                .chars()
                .nth(((i / step_down) * step_right) % line.len())
                .unwrap()
                == '#'
            {
                trees += 1;
            }
        }
    }
    trees
}

impl Day3 {
    fn solve_part1(&self, input: &Vec<String>) -> usize {
        get_trees(input, 3, 1)
    }

    fn solve_part2(&self, input: &Vec<String>) -> usize {
        get_trees(input, 1, 1)
            * get_trees(input, 3, 1)
            * get_trees(input, 5, 1)
            * get_trees(input, 7, 1)
            * get_trees(input, 1, 2)
    }
}

impl Puzzle for Day3 {
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
            Day3 {}.solve_part1(
                &vec!(
                    "..##.......",
                    "#...#...#..",
                    ".#....#..#.",
                    "..#.#...#.#",
                    ".#...##..#.",
                    "..#.##.....",
                    ".#.#.#....#",
                    ".#........#",
                    "#.##...#...",
                    "#...##....#",
                    ".#..#...#.#"
                )
                .iter()
                .map(|x| x.to_string())
                .collect()
            ),
            7
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day3 {}.solve_part2(
                &vec!(
                    "..##.......",
                    "#...#...#..",
                    ".#....#..#.",
                    "..#.#...#.#",
                    ".#...##..#.",
                    "..#.##.....",
                    ".#.#.#....#",
                    ".#........#",
                    "#.##...#...",
                    "#...##....#",
                    ".#..#...#.#"
                )
                .iter()
                .map(|x| x.to_string())
                .collect()
            ),
            336
        );
    }
}
