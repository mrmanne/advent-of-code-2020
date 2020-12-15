use crate::puzzle::{io, File, Puzzle};
pub struct Day1;

fn sum_to(val: i64, limit: i64, data: &[i64]) -> Option<Vec<i64>> {
    if data.len() == 0 || val <= 0 || limit <= 0 {
        return None;
    }
    if val == data[0] && limit == 1 {
        return Some(vec![data[0]]);
    }
    if let Some(mut included) = sum_to(val - data[0], limit - 1, &data[1..]) {
        included.push(data[0]);
        return Some(included);
    }
    if let Some(excluded) = sum_to(val, limit, &data[1..]) {
        return Some(excluded);
    }
    None
}

impl Day1 {
    fn solve_part1(&self, input: &Vec<i64>) -> i64 {
        let parts = sum_to(2020, 2, input).unwrap();
        parts.iter().fold(1, |answer, i| answer * i)
    }

    fn solve_part2(&self, input: &Vec<i64>) -> i64 {
        let parts = sum_to(2020, 3, input).unwrap();
        parts.iter().fold(1, |answer, i| answer * i)
    }
}

impl Puzzle for Day1 {
    fn solve(&self, lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let numbers: Vec<i64> = lines
            .expect("No input file")
            .map(|l| l.unwrap().parse::<i64>().unwrap())
            .collect();
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
            Day1 {}.solve_part1(&vec!(1721, 979, 366, 299, 675, 1456)),
            514579
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day1 {}.solve_part2(&vec!(1721, 979, 366, 299, 675, 1456)),
            241861950
        );
    }
}
