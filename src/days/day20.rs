use crate::puzzle::{io, File, Puzzle};
use std::fmt;
pub struct Day20;

#[derive(Clone)]
struct Tile {
    p: Vec<Vec<char>>,
    id: usize,
}

impl Tile {
    fn new(id: usize, pixels: Vec<Vec<char>>) -> Self {
        Self { p: pixels, id: id }
    }

    fn pixels_active(&self) -> usize {
        let mut count = 0;
        for y in 0..self.p.len() {
            for x in 0..self.p[0].len() {
                if self.p[y][x] == '#' {
                    count += 1;
                }
            }
        }
        count
    }

    fn rotate_clockwise(&self, turns: usize) -> Tile {
        let size_x = self.p[0].len();
        let size_y = self.p.len();
        let mut result = self.clone();
        for _ in 0..turns {
            let mut next = result.clone();
            for (y, row) in result.p.iter().enumerate() {
                for (x, p) in row.iter().enumerate() {
                    next.p[(size_x - 1) - ((size_x - 1) - x)][(size_y - 1) - y] = *p;
                }
            }
            result = next;
        }
        result
    }

    fn flip_horizontal(&self) -> Tile {
        let size_y = self.p.len();
        let mut result = self.clone();
        for (y, row) in self.p.iter().enumerate() {
            for (x, p) in row.iter().enumerate() {
                result.p[(size_y - 1) - y][x] = *p;
            }
        }
        result
    }

    fn left_id(&self) -> u32 {
        let mut id = 0;
        for (i, row) in self.p.iter().enumerate() {
            if row[0] == '#' {
                id |= 1 << i;
            }
        }
        id
    }

    fn right_id(&self) -> u32 {
        let mut id = 0;
        for (i, row) in self.p.iter().enumerate() {
            if row[row.len() - 1] == '#' {
                id |= 1 << i;
            }
        }
        id
    }

    fn top_id(&self) -> u32 {
        let mut id = 0;
        for (i, p) in self.p[0].iter().enumerate() {
            if *p == '#' {
                id |= 1 << i;
            }
        }
        id
    }

    fn bottom_id(&self) -> u32 {
        let mut id = 0;
        for (i, p) in self.p[self.p.len() - 1].iter().enumerate() {
            if *p == '#' {
                id |= 1 << i;
            }
        }
        id
    }

    fn contains(&self, pattern: &Tile) -> usize {
        let mut count = 0;
        if pattern.p.len() > self.p.len() {
            return 0;
        }
        if pattern.p[0].len() > self.p[0].len() {
            return 0;
        }
        let xoffsets = self.p[0].len() - pattern.p[0].len();
        let yoffsets = self.p.len() - pattern.p.len();
        for yoffset in 0..yoffsets {
            'next: for xoffset in 0..xoffsets {
                for y in 0..pattern.p.len() {
                    for x in 0..pattern.p[0].len() {
                        if pattern.p[y][x] == '#' && self.p[y + yoffset][x + xoffset] != '#' {
                            continue 'next;
                        }
                    }
                }
                count += 1;
            }
        }
        count
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tile {}:\n", self.id)?;
        for row in &self.p {
            for c in row {
                write!(f, "{}", *c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn place(
    pos: usize,
    tiles: &Vec<Tile>,
    image: &Vec<Vec<Option<Tile>>>,
) -> Option<Vec<Vec<Option<Tile>>>> {
    let size = image.len();
    let y = pos / size;
    let x = pos % size;

    if tiles.len() == 0 {
        return Some(image.clone());
    }
    for i in 0..tiles.len() {
        for turns in 0..4 {
            for flip in 0..2 {
                let tile = if flip == 1 {
                    tiles[i].flip_horizontal().rotate_clockwise(turns)
                } else {
                    tiles[i].rotate_clockwise(turns)
                };
                if x > 0
                    && image[y][x - 1].is_some()
                    && tile.left_id() != image[y][x - 1].as_ref().unwrap().right_id()
                {
                    continue;
                }
                if x < (size - 1)
                    && image[y][x + 1].is_some()
                    && tile.right_id() != image[y][x + 1].as_ref().unwrap().left_id()
                {
                    continue;
                }
                if y > 0
                    && image[y - 1][x].is_some()
                    && tile.top_id() != image[y - 1][x].as_ref().unwrap().bottom_id()
                {
                    continue;
                }
                if y < (size - 1)
                    && image[y + 1][x].is_some()
                    && tile.bottom_id() != image[y + 1][x].as_ref().unwrap().top_id()
                {
                    continue;
                }
                let mut tiles_left = tiles.clone();
                tiles_left.remove(i);
                let mut updated_image = image.clone();
                updated_image[y][x] = Some(tile);
                if let Some(result) = place(pos + 1, &tiles_left, &updated_image) {
                    return Some(result);
                }
            }
        }
    }
    return None;
}

fn parse_input(input: &Vec<String>) -> (usize, Vec<Tile>) {
    let mut tiles = vec![];
    let mut id = 0;
    let mut rows = vec![];
    for line in input {
        if line.contains("Tile") {
            id = line
                .split(" ")
                .nth(1)
                .unwrap()
                .split(":")
                .nth(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
        } else if line == "" {
            tiles.push(Tile::new(id, rows));
            rows = vec![];
        } else {
            rows.push(line.chars().collect());
        }
    }
    if !rows.is_empty() {
        tiles.push(Tile::new(id, rows));
    }
    let size = (tiles.len() as f64).sqrt() as usize;
    (size, tiles)
}

fn merge_tiles(image: &Vec<Vec<Option<Tile>>>) -> Tile {
    let size = image.len();
    let mut rows = vec![];
    let tile_size = image[0][0].as_ref().unwrap().p.len();
    for y in 1..size * tile_size {
        if y % tile_size == 0 || (y + 1) % tile_size == 0 {
            continue;
        }
        let tile_y = y / tile_size;
        let mut columns = vec![];
        for x in 0..size * tile_size {
            if x % tile_size == 0 || (x + 1) % tile_size == 0 {
                continue;
            }
            let tile_x = x / tile_size;
            let tile = image[tile_y][tile_x].as_ref().unwrap();
            columns.push(tile.p[y % tile_size][x % tile_size]);
        }
        rows.push(columns);
    }
    Tile::new(0, rows)
}

impl Day20 {
    fn solve_part1(&self, input: Vec<String>) -> usize {
        let (size, tiles) = parse_input(&input);
        let image = place(0, &tiles, &mut vec![vec![None; size]; size]).unwrap();
        image[0][0].as_ref().unwrap().id
            * image[size - 1][0].as_ref().unwrap().id
            * image[0][size - 1].as_ref().unwrap().id
            * image[size - 1][size - 1].as_ref().unwrap().id
    }

    fn solve_part2(&self, input: Vec<String>) -> usize {
        let (size, tiles) = parse_input(&input);
        let image = place(0, &tiles, &mut vec![vec![None; size]; size]).unwrap();
        let image_tile = merge_tiles(&image);
        let monster_pattern: Vec<Vec<char>> = vec![
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ]
        .iter()
        .map(|x| x.to_string().chars().collect())
        .collect();
        let monster = Tile::new(0, monster_pattern);

        let mut nof_monsters = std::usize::MIN;
        let mut rough_waters = 0;
        for turns in 0..4 {
            for flip in 0..2 {
                let changed_image = if flip == 1 {
                    image_tile.flip_horizontal().rotate_clockwise(turns)
                } else {
                    image_tile.rotate_clockwise(turns)
                };
                let count = changed_image.contains(&monster);
                if count > nof_monsters {
                    nof_monsters = count;
                    rough_waters =
                        changed_image.pixels_active() - monster.pixels_active() * nof_monsters;
                }
            }
        }
        rough_waters
    }
}

impl Puzzle for Day20 {
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
            Day20 {}.solve_part1(string_vec!(
                "Tile 2311:",
                "..##.#..#.",
                "##..#.....",
                "#...##..#.",
                "####.#...#",
                "##.##.###.",
                "##...#.###",
                ".#.#.#..##",
                "..#....#..",
                "###...#.#.",
                "..###..###",
                "",
                "Tile 1951:",
                "#.##...##.",
                "#.####...#",
                ".....#..##",
                "#...######",
                ".##.#....#",
                ".###.#####",
                "###.##.##.",
                ".###....#.",
                "..#.#..#.#",
                "#...##.#..",
                "",
                "Tile 1171:",
                "####...##.",
                "#..##.#..#",
                "##.#..#.#.",
                ".###.####.",
                "..###.####",
                ".##....##.",
                ".#...####.",
                "#.##.####.",
                "####..#...",
                ".....##...",
                "",
                "Tile 1427:",
                "###.##.#..",
                ".#..#.##..",
                ".#.##.#..#",
                "#.#.#.##.#",
                "....#...##",
                "...##..##.",
                "...#.#####",
                ".#.####.#.",
                "..#..###.#",
                "..##.#..#.",
                "",
                "Tile 1489:",
                "##.#.#....",
                "..##...#..",
                ".##..##...",
                "..#...#...",
                "#####...#.",
                "#..#.#.#.#",
                "...#.#.#..",
                "##.#...##.",
                "..##.##.##",
                "###.##.#..",
                "",
                "Tile 2473:",
                "#....####.",
                "#..#.##...",
                "#.##..#...",
                "######.#.#",
                ".#...#.#.#",
                ".#########",
                ".###.#..#.",
                "########.#",
                "##...##.#.",
                "..###.#.#.",
                "",
                "Tile 2971:",
                "..#.#....#",
                "#...###...",
                "#.#.###...",
                "##.##..#..",
                ".#####..##",
                ".#..####.#",
                "#..#.#..#.",
                "..####.###",
                "..#.#.###.",
                "...#.#.#.#",
                "",
                "Tile 2729:",
                "...#.#.#.#",
                "####.#....",
                "..#.#.....",
                "....#..#.#",
                ".##..##.#.",
                ".#.####...",
                "####.#.#..",
                "##.####...",
                "##..#.##..",
                "#.##...##.",
                "",
                "Tile 3079:",
                "#.#.#####.",
                ".#..######",
                "..#.......",
                "######....",
                "####.#..#.",
                ".#...#.##.",
                "#.#####.##",
                "..#.###...",
                "..#.......",
                "..#.###..."
            )),
            20899048083289
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day20 {}.solve_part2(string_vec!(
                "Tile 2311:",
                "..##.#..#.",
                "##..#.....",
                "#...##..#.",
                "####.#...#",
                "##.##.###.",
                "##...#.###",
                ".#.#.#..##",
                "..#....#..",
                "###...#.#.",
                "..###..###",
                "",
                "Tile 1951:",
                "#.##...##.",
                "#.####...#",
                ".....#..##",
                "#...######",
                ".##.#....#",
                ".###.#####",
                "###.##.##.",
                ".###....#.",
                "..#.#..#.#",
                "#...##.#..",
                "",
                "Tile 1171:",
                "####...##.",
                "#..##.#..#",
                "##.#..#.#.",
                ".###.####.",
                "..###.####",
                ".##....##.",
                ".#...####.",
                "#.##.####.",
                "####..#...",
                ".....##...",
                "",
                "Tile 1427:",
                "###.##.#..",
                ".#..#.##..",
                ".#.##.#..#",
                "#.#.#.##.#",
                "....#...##",
                "...##..##.",
                "...#.#####",
                ".#.####.#.",
                "..#..###.#",
                "..##.#..#.",
                "",
                "Tile 1489:",
                "##.#.#....",
                "..##...#..",
                ".##..##...",
                "..#...#...",
                "#####...#.",
                "#..#.#.#.#",
                "...#.#.#..",
                "##.#...##.",
                "..##.##.##",
                "###.##.#..",
                "",
                "Tile 2473:",
                "#....####.",
                "#..#.##...",
                "#.##..#...",
                "######.#.#",
                ".#...#.#.#",
                ".#########",
                ".###.#..#.",
                "########.#",
                "##...##.#.",
                "..###.#.#.",
                "",
                "Tile 2971:",
                "..#.#....#",
                "#...###...",
                "#.#.###...",
                "##.##..#..",
                ".#####..##",
                ".#..####.#",
                "#..#.#..#.",
                "..####.###",
                "..#.#.###.",
                "...#.#.#.#",
                "",
                "Tile 2729:",
                "...#.#.#.#",
                "####.#....",
                "..#.#.....",
                "....#..#.#",
                ".##..##.#.",
                ".#.####...",
                "####.#.#..",
                "##.####...",
                "##..#.##..",
                "#.##...##.",
                "",
                "Tile 3079:",
                "#.#.#####.",
                ".#..######",
                "..#.......",
                "######....",
                "####.#..#.",
                ".#...#.##.",
                "#.#####.##",
                "..#.###...",
                "..#.......",
                "..#.###..."
            )),
            273
        );
    }
}
