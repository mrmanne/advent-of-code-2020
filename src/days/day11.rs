use crate::puzzle::{io, File, Puzzle};

pub struct Day11;

fn get_adjacent_occupied_seats(x: i64, y: i64, seatmap: &Vec<String>) -> usize {
    let mut sum: usize = 0;
    let points = [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];
    for (x2, y2) in points.iter() {
        if (*x2 >= 0 && *x2 < seatmap[0].len() as i64) && (*y2 >= 0 && *y2 < seatmap.len() as i64) {
            if seatmap[*y2 as usize].chars().nth(*x2 as usize).unwrap() == '#' {
                sum += 1;
            }
        }
    }
    sum
}

fn get_visible_occupied_seats_(
    mut x: i64,
    mut y: i64,
    dx: i64,
    dy: i64,
    seatmap: &Vec<String>,
) -> usize {
    loop {
        x += dx;
        y += dy;
        if x < 0 || x >= seatmap[0].len() as i64 || y < 0 || y >= seatmap.len() as i64 {
            return 0;
        }
        let place = seatmap[y as usize].chars().nth(x as usize).unwrap();
        if place == '#' {
            return 1;
        }
        if place == 'L' {
            return 0;
        }
    }
}

fn get_visible_occupied_seats(x: i64, y: i64, seatmap: &Vec<String>) -> usize {
    get_visible_occupied_seats_(x, y, 0, 1, seatmap)
        + get_visible_occupied_seats_(x, y, 0, -1, seatmap)
        + get_visible_occupied_seats_(x, y, 1, 0, seatmap)
        + get_visible_occupied_seats_(x, y, -1, 0, seatmap)
        + get_visible_occupied_seats_(x, y, 1, 1, seatmap)
        + get_visible_occupied_seats_(x, y, -1, -1, seatmap)
        + get_visible_occupied_seats_(x, y, 1, -1, seatmap)
        + get_visible_occupied_seats_(x, y, -1, 1, seatmap)
}

fn next_turn<F>(seatmap: &Vec<String>, occupied_seats_fn: F, max_neighbours: usize) -> Vec<String>
where
    F: Fn(i64, i64, &Vec<String>) -> usize,
{
    let mut new_seatmap = seatmap.clone();
    for y in 0..seatmap.len() {
        for x in 0..seatmap[y].len() {
            let occouped_neighbours = occupied_seats_fn(x as i64, y as i64, seatmap);
            if seatmap[y].chars().nth(x).unwrap() == 'L' && occouped_neighbours == 0 {
                unsafe { new_seatmap[y][..].as_bytes_mut()[x] = '#' as u8 };
            } else if seatmap[y].chars().nth(x).unwrap() == '#'
                && occouped_neighbours >= max_neighbours
            {
                unsafe { new_seatmap[y][..].as_bytes_mut()[x] = 'L' as u8 };
            }
        }
    }
    new_seatmap
}

fn get_nof_occupied_seats(seatmap: &Vec<String>) -> usize {
    return seatmap.iter().fold(0, |acc, x| {
        acc + x
            .chars()
            .fold(0, |acc, y| if y == '#' { acc + 1 } else { acc })
    });
}

impl Day11 {
    fn solve_part1(&self, mut seatmap: Vec<String>) -> usize {
        loop {
            let new_seatmap = next_turn(&seatmap, get_adjacent_occupied_seats, 4);
            if new_seatmap == seatmap {
                return get_nof_occupied_seats(&new_seatmap);
            }
            seatmap = new_seatmap;
        }
    }

    fn solve_part2(&self, mut seatmap: Vec<String>) -> usize {
        loop {
            let new_seatmap = next_turn(&seatmap, get_visible_occupied_seats, 5);
            if new_seatmap == seatmap {
                return get_nof_occupied_seats(&new_seatmap);
            }
            seatmap = new_seatmap;
        }
    }
}

impl Puzzle for Day11 {
    fn solve(&self, lines: io::Lines<io::BufReader<File>>) -> (String, String) {
        let input: Vec<String> = lines.map(|l| l.unwrap()).collect();
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
            Day11 {}.solve_part1(
                vec!(
                    "L.LL.LL.LL",
                    "LLLLLLL.LL",
                    "L.L.L..L..",
                    "LLLL.LL.LL",
                    "L.LL.LL.LL",
                    "L.LLLLL.LL",
                    "..L.L.....",
                    "LLLLLLLLLL",
                    "L.LLLLLL.L",
                    "L.LLLLL.LL"
                )
                .iter()
                .map(|x| x.to_string())
                .collect()
            ),
            37
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day11 {}.solve_part2(
                vec!(
                    "L.LL.LL.LL",
                    "LLLLLLL.LL",
                    "L.L.L..L..",
                    "LLLL.LL.LL",
                    "L.LL.LL.LL",
                    "L.LLLLL.LL",
                    "..L.L.....",
                    "LLLLLLLLLL",
                    "L.LLLLLL.L",
                    "L.LLLLL.LL"
                )
                .iter()
                .map(|x| x.to_string())
                .collect()
            ),
            26
        );
    }

    #[test]
    fn part2_get_visible_occupied_seats1() {
        assert_eq!(
            get_visible_occupied_seats(
                3,
                4,
                &vec!(
                    ".......#.",
                    "...#.....",
                    ".#.......",
                    ".........",
                    "..#L....#",
                    "....#....",
                    ".........",
                    "#........",
                    "...#....."
                )
                .iter()
                .map(|x| x.to_string())
                .collect()
            ),
            8
        );
    }

    #[test]
    fn part2_get_visible_occupied_seats2() {
        assert_eq!(
            get_visible_occupied_seats(
                1,
                1,
                &vec!(".............", ".L.L.#.#.#.#.", ".............")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            0
        );
    }
}
