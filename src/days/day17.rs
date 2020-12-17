use std::collections::HashSet;

use crate::puzzle::{io, File, Puzzle};
pub struct Day17;

enum Dimensions {
    D3,
    D4,
}

fn neighbours(x: i64, y: i64, z: i64, w: i64, cubes: &HashSet<(i64, i64, i64, i64)>) -> usize {
    let mut neighbours = 0;
    for x2 in x - 1..x + 2 {
        for y2 in y - 1..y + 2 {
            for z2 in z - 1..z + 2 {
                for w2 in w - 1..w + 2 {
                    if x == x2 && y == y2 && z == z2 && w == w2 {
                        continue;
                    }
                    if let Some(_) = cubes.get(&(x2, y2, z2, w2)) {
                        neighbours += 1;
                    }
                }
            }
        }
    }
    neighbours
}

fn parse_input(input: &Vec<String>) -> HashSet<(i64, i64, i64, i64)> {
    let mut cubes = HashSet::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    cubes.insert((x as i64, y as i64, 0i64, 0i64));
                }
                _ => panic!("Illegal input character"),
            }
        }
    }
    cubes
}

fn get_max(cubes: &HashSet<(i64, i64, i64, i64)>) -> (i64, i64, i64, i64) {
    cubes.iter().fold(
        (std::i64::MIN, std::i64::MIN, std::i64::MIN, std::i64::MIN),
        |(maxx, maxy, maxz, maxw), (x, y, z, w)| {
            (
                std::cmp::max(maxx, *x),
                std::cmp::max(maxy, *y),
                std::cmp::max(maxz, *z),
                std::cmp::max(maxw, *w),
            )
        },
    )
}

fn get_min(cubes: &HashSet<(i64, i64, i64, i64)>) -> (i64, i64, i64, i64) {
    cubes.iter().fold(
        (std::i64::MAX, std::i64::MAX, std::i64::MAX, std::i64::MAX),
        |(minx, miny, minz, minw), (x, y, z, w)| {
            (
                std::cmp::min(minx, *x),
                std::cmp::min(miny, *y),
                std::cmp::min(minz, *z),
                std::cmp::min(minw, *w),
            )
        },
    )
}

fn boot(cubes: &mut HashSet<(i64, i64, i64, i64)>, dimensions: Dimensions) {
    for _ in 0..6 {
        let mut new_state = cubes.clone();
        let (maxx, maxy, maxz, mut maxw) = get_max(&cubes);
        let (minx, miny, minz, mut minw) = get_min(&cubes);

        match dimensions {
            Dimensions::D3 => {
                minw = 0;
                maxw = 1;
            }
            Dimensions::D4 => {
                minw -= 1;
                maxw += 2;
            }
        }
        for x in minx - 1..maxx + 2 {
            for y in miny - 1..maxy + 2 {
                for z in minz - 1..maxz + 2 {
                    for w in minw..maxw {
                        let neighbours = neighbours(x, y, z, w, &cubes);
                        if let Some(_) = cubes.get(&(x, y, z, w)) {
                            if neighbours < 2 || neighbours > 3 {
                                new_state.remove(&(x, y, z, w));
                            }
                        } else {
                            if neighbours == 3 {
                                new_state.insert((x, y, z, w));
                            }
                        }
                    }
                }
            }
        }
        *cubes = new_state;
    }
}

impl Day17 {
    fn solve_part1(&self, input: Vec<String>) -> usize {
        let mut cubes = parse_input(&input);
        boot(&mut cubes, Dimensions::D3);
        cubes.len()
    }

    fn solve_part2(&self, input: Vec<String>) -> usize {
        let mut cubes = parse_input(&input);
        boot(&mut cubes, Dimensions::D4);
        cubes.len()
    }
}

impl Puzzle for Day17 {
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

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day17 {}.solve_part1(
                vec!(".#.", "..#", "###")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            112
        );
    }

    // Disable the unit test since it's a bit slow to run for all builds.
    // #[test]
    // fn part2_example1() {
    //     assert_eq!(
    //         Day17 {}.solve_part2(
    //             vec!(".#.", "..#", "###")
    //                 .iter()
    //                 .map(|x| x.to_string())
    //                 .collect()
    //         ),
    //         848
    //     );
    // }
}
