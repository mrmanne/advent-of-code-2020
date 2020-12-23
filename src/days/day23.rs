use crate::puzzle::{io, File, Puzzle};

pub struct Day23;

fn play(cups: &mut Vec<usize>, mut cur: usize, turns: usize) {
    let max = cups.len() - 1;
    for _ in 0..turns {
        let mut removals = vec![];

        let mut next = cups[cur];
        for _ in 0..3 {
            removals.push(next);
            next = cups[next];
        }
        cups[cur] = next;

        let mut dest = 0;
        for i in 1..max + 1 {
            dest = cur as i32 - (i as i32);
            if dest <= 0 {
                dest = max as i32 + dest;
            }
            if !removals.contains(&(dest as usize)) {
                break;
            }
        }

        let tmp = cups[dest as usize];
        cups[dest as usize] = removals[0];
        cups[removals[2]] = tmp;
        cur = cups[cur];
    }
}

impl Day23 {
    fn solve_part1(&self, input: &str, turns: usize) -> String {
        let input: Vec<usize> = input
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect();

        // Build kind of a linked list vector where you can index a cup label to find the next cup.
        // This gives O(1) time complexity for insertions and removals.
        let mut cups: Vec<usize> = vec![0; input.len() + 1];
        for i in 0..cups.len() - 1 {
            cups[input[i]] = input[(i + 1) % input.len()];
        }

        play(&mut cups, input[0], turns);

        let mut result = String::new();

        let mut next = 1;
        for _ in 1..cups.len() - 1 {
            result += &cups[next].to_string();
            next = cups[next];
        }
        result
    }

    fn solve_part2(&self, input: &str, turns: usize) -> usize {
        let input: Vec<usize> = input
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect();

        // Build kind of a linked list vector where you can index a cup label to find the next cup.
        // This gives O(1) time complexity for insertions and removals.
        let mut cups: Vec<usize> = vec![0; input.len() + 1];
        for i in 0..cups.len() - 2 {
            cups[input[i]] = input[(i + 1)];
        }
        cups[input[input.len() - 1]] = 10;
        for i in 10..1000000 {
            cups.push(i + 1);
        }
        cups.push(input[0]);

        play(&mut cups, input[0], turns);

        let mut result = String::new();
        let mut next = 1;
        for _ in 1..cups.len() - 1 {
            result += &cups[next].to_string();
            next = cups[next];
        }
        cups[1] as usize * cups[cups[1]] as usize
    }
}

impl Puzzle for Day23 {
    fn solve(&self, _: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        return (
            self.solve_part1("792845136", 100),
            self.solve_part2("792845136", 10000000).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(Day23 {}.solve_part1("389125467", 10), "92658374");
    }

    // Disable this unit test since its too slow to run for every build.
    // #[test]
    // fn part2_example1() {
    //     assert_eq!(Day23 {}.solve_part2("389125467", 10000000), 149245887792);
    // }
}
