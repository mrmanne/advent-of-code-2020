use crate::puzzle::{io, File, Puzzle};
pub struct Day5;

fn get_seats(input: &Vec<String>) -> Vec<u16> {
    let mut seats: Vec<u16> = vec![];
    for line in input {
        let mut id: u16 = 0;
        let mut pos = 9;
        for c in line[0..7].chars() {
            if c == 'B' {
                id |= 1 << pos;
            }
            pos -= 1;
        }
        for c in line[7..].chars() {
            if c == 'R' {
                id |= 1 << pos;
            }
            pos -= 1;
        }
        seats.push(id);
    }
    seats.sort();
    seats
}

impl Day5 {
    fn solve_part1(&self, input: &Vec<String>) -> u16 {
        *get_seats(input).iter().max().unwrap()
    }

    fn solve_part2(&self, input: &Vec<String>) -> u16 {
        let seats = get_seats(input);
        let mut prev: u16 = 0;
        for (i, s) in seats.iter().enumerate() {
            if i > 0 {
                if *s != (prev + 1) {
                    break;
                }
            }
            prev = *s;
        }
        prev + 1
    }
}

impl Puzzle for Day5 {
    fn solve(&self, lines: io::Lines<io::BufReader<File>>) -> (String, String) {
        let numbers: Vec<String> = lines.map(|l| l.unwrap()).collect();
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
            Day5 {}.solve_part1(&vec!("FBFBBFFRLR").iter().map(|x| x.to_string()).collect()),
            357
        );
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            Day5 {}.solve_part1(
                &vec!("BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            820
        );
    }
}
