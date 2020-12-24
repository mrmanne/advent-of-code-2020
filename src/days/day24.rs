use std::collections::HashSet;

use crate::puzzle::{io, File, Puzzle};

pub struct Day24;

#[derive(Debug)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::E,
    Direction::W,
    Direction::NE,
    Direction::SW,
    Direction::SE,
    Direction::NW,
];

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Tile(i64, i64, i64);

impl Tile {
    fn new(e_w: i64, nw_se: i64, ne_sw: i64) -> Self {
        let mut tile = Self(e_w, nw_se, ne_sw);
        tile.normalize();
        tile
    }

    fn add_tile(&self, other: &Tile) -> Tile {
        let mut result = Tile(self.0 + other.0, self.1 + other.1, self.2 + other.2);
        result.normalize();
        result
    }

    fn add_direction(&self, direction: &Direction) -> Tile {
        let direction_tile = match direction {
            Direction::NE => Tile(0, 0, 1),
            Direction::SW => Tile(0, 0, -1),
            Direction::SE => Tile(0, 1, 0),
            Direction::NW => Tile(0, -1, 0),
            Direction::E => Tile(1, 0, 0),
            Direction::W => Tile(-1, 0, 0),
        };
        self.add_tile(&direction_tile)
    }

    fn normalize(&mut self) {
        let diff = std::cmp::min(self.1, self.2);
        self.1 -= diff;
        self.2 -= diff;
        self.0 += diff;
    }
}

fn get_black_adjacent_tiles(tile: &Tile, floor: &HashSet<Tile>) -> usize {
    let mut count = 0;
    for direction in &ALL_DIRECTIONS {
        let adjacent_tile = tile.add_direction(&direction);
        if let Some(_) = floor.get(&adjacent_tile) {
            count += 1;
        }
    }
    count
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<Direction>> {
    let mut tiles = vec![];
    for line in input {
        let mut directions: Vec<Direction> = vec![];
        let mut i = 0;
        while i < line.len() {
            let keyword = &line[i..std::cmp::min(i + 2, line.len())];
            let direction = if keyword.starts_with("e") {
                i += 1;
                Direction::E
            } else if keyword.starts_with("se") {
                i += 2;
                Direction::SE
            } else if keyword.starts_with("sw") {
                i += 2;
                Direction::SW
            } else if keyword.starts_with("w") {
                i += 1;
                Direction::W
            } else if keyword.starts_with("nw") {
                i += 2;
                Direction::NW
            } else if keyword.starts_with("ne") {
                i += 2;
                Direction::NE
            } else {
                panic!("Illegal direction {}!", &keyword);
            };
            directions.push(direction);
        }
        tiles.push(directions);
    }
    tiles
}

fn get_dest_tile(directions: &Vec<Direction>) -> Tile {
    let mut tile = Tile::new(0, 0, 0);
    for direction in directions {
        tile = tile.add_direction(direction);
    }
    tile
}

impl Day24 {
    fn solve_part1(&self, input: Vec<String>) -> usize {
        let mut floor = HashSet::new();

        let tiles = parse_input(&input);
        for tile_directions in &tiles {
            let tile = get_dest_tile(&tile_directions);
            if let Some(_) = floor.get(&tile) {
                floor.remove(&tile);
            } else {
                floor.insert(tile);
            }
        }
        floor.len()
    }

    fn solve_part2(&self, input: Vec<String>) -> usize {
        let mut floor = HashSet::new();

        let tiles = parse_input(&input);
        for tile_directions in &tiles {
            let tile = get_dest_tile(&tile_directions);
            if let Some(_) = floor.get(&tile) {
                floor.remove(&tile);
            } else {
                floor.insert(tile);
            }
        }

        for _ in 0..100 {
            let mut new_floor = floor.clone();
            for black_tile in &floor {
                let black_adjacent_tiles = get_black_adjacent_tiles(black_tile, &floor);
                if black_adjacent_tiles == 0 || black_adjacent_tiles > 2 {
                    new_floor.remove(black_tile);
                }
                for direction in &ALL_DIRECTIONS {
                    let adjacent_tile = black_tile.add_direction(direction);
                    match floor.get(&adjacent_tile) {
                        Some(_) => (),
                        None => {
                            if get_black_adjacent_tiles(&adjacent_tile, &floor) == 2 {
                                new_floor.insert(adjacent_tile);
                            }
                        }
                    }
                }
            }
            floor = new_floor;
        }
        floor.len()
    }
}

impl Puzzle for Day24 {
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
            Day24 {}.solve_part1(string_vec!(
                "sesenwnenenewseeswwswswwnenewsewsw",
                "neeenesenwnwwswnenewnwwsewnenwseswesw",
                "seswneswswsenwwnwse",
                "nwnwneseeswswnenewneswwnewseswneseene",
                "swweswneswnenwsewnwneneseenw",
                "eesenwseswswnenwswnwnwsewwnwsene",
                "sewnenenenesenwsewnenwwwse",
                "wenwwweseeeweswwwnwwe",
                "wsweesenenewnwwnwsenewsenwwsesesenwne",
                "neeswseenwwswnwswswnw",
                "nenwswwsewswnenenewsenwsenwnesesenew",
                "enewnwewneswsewnwswenweswnenwsenwsw",
                "sweneswneswneneenwnewenewwneswswnese",
                "swwesenesewenwneswnwwneseswwne",
                "enesenwswwswneneswsenwnewswseenwsese",
                "wnwnesenesenenwwnenwsewesewsesesew",
                "nenewswnwewswnenesenwnesewesw",
                "eneswnwswnwsenenwnwnwwseeswneewsenese",
                "neswnwewnwnwseenwseesewsenwsweewe",
                "wseweeenwnesenwwwswnew"
            )),
            10
        );
    }

    #[test]
    fn part1_get_dest() {
        assert_eq!(
            get_dest_tile(&parse_input(&string_vec!("esenee"))[0]),
            Tile::new(3, 0, 0)
        );
        assert_eq!(
            get_dest_tile(&parse_input(&string_vec!("wswnww"))[0]),
            Tile::new(-3, 0, 0)
        );
        assert_eq!(
            get_dest_tile(&parse_input(&string_vec!("nwwswee"))[0]),
            Tile::new(0, 0, 0)
        );
        assert_eq!(
            get_dest_tile(&parse_input(&string_vec!("esew"))[0]),
            Tile::new(0, 1, 0)
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day24 {}.solve_part2(string_vec!(
                "sesenwnenenewseeswwswswwnenewsewsw",
                "neeenesenwnwwswnenewnwwsewnenwseswesw",
                "seswneswswsenwwnwse",
                "nwnwneseeswswnenewneswwnewseswneseene",
                "swweswneswnenwsewnwneneseenw",
                "eesenwseswswnenwswnwnwsewwnwsene",
                "sewnenenenesenwsewnenwwwse",
                "wenwwweseeeweswwwnwwe",
                "wsweesenenewnwwnwsenewsenwwsesesenwne",
                "neeswseenwwswnwswswnw",
                "nenwswwsewswnenenewsenwsenwnesesenew",
                "enewnwewneswsewnwswenweswnenwsenwsw",
                "sweneswneswneneenwnewenewwneswswnese",
                "swwesenesewenwneswnwwneseswwne",
                "enesenwswwswneneswsenwnewswseenwsese",
                "wnwnesenesenenwwnenwsewesewsesesew",
                "nenewswnwewswnenesenwnesewesw",
                "eneswnwswnwsenenwnwnwwseeswneewsenese",
                "neswnwewnwnwseenwseesewsenwsweewe",
                "wseweeenwnesenwwwswnew"
            )),
            2208
        );
    }
}
