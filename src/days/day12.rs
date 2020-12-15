use crate::puzzle::{io, File, Puzzle};
use std::mem;

pub struct Day12;

#[derive(Debug)]
enum Action {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Ship {
    pos: Coord,
    bearing: i64,
    waypoint: Coord,
}

impl Ship {
    fn new() -> Self {
        Self {
            pos: Coord { x: 0, y: 0 },
            bearing: 90,
            waypoint: Coord { x: 10, y: 1 },
        }
    }

    fn navigate_bearing(&mut self, action: &Action) {
        match action {
            Action::North(distance) => self.pos.y += *distance as i64,
            Action::South(distance) => self.pos.y -= *distance as i64,
            Action::East(distance) => self.pos.x += *distance as i64,
            Action::West(distance) => self.pos.x -= *distance as i64,
            Action::Left(degrees) => {
                self.bearing -= degrees;
                if self.bearing < 0 {
                    self.bearing += 360;
                }
            }
            Action::Right(degrees) => {
                self.bearing += degrees;
                if self.bearing >= 360 {
                    self.bearing -= 360;
                }
            }
            Action::Forward(distance) => match self.bearing {
                0 => self.navigate_bearing(&Action::North(*distance)),
                90 => self.navigate_bearing(&Action::East(*distance)),
                180 => self.navigate_bearing(&Action::South(*distance)),
                270 => self.navigate_bearing(&Action::West(*distance)),
                _ => panic!("Illegal ship bearing!"),
            },
        }
    }

    fn navigate_waypoint(&mut self, action: &Action) {
        match action {
            Action::North(distance) => self.waypoint.y += *distance as i64,
            Action::South(distance) => self.waypoint.y -= *distance as i64,
            Action::East(distance) => self.waypoint.x += *distance as i64,
            Action::West(distance) => self.waypoint.x -= *distance as i64,
            Action::Left(degrees) => match degrees % 360 {
                0 => (),
                90 => {
                    mem::swap(&mut self.waypoint.x, &mut self.waypoint.y);
                    self.waypoint.x = -self.waypoint.x;
                }
                180 => {
                    self.waypoint.x = -self.waypoint.x;
                    self.waypoint.y = -self.waypoint.y;
                }
                270 => {
                    mem::swap(&mut self.waypoint.x, &mut self.waypoint.y);
                    self.waypoint.y = -self.waypoint.y;
                }
                _ => panic!("Illegal ship bearing!"),
            },

            Action::Right(degrees) => match degrees % 360 {
                0 => (),
                90 => {
                    mem::swap(&mut self.waypoint.x, &mut self.waypoint.y);
                    self.waypoint.y = -self.waypoint.y;
                }
                180 => {
                    self.waypoint.x = -self.waypoint.x;
                    self.waypoint.y = -self.waypoint.y;
                }
                270 => {
                    mem::swap(&mut self.waypoint.x, &mut self.waypoint.y);
                    self.waypoint.x = -self.waypoint.x;
                }
                _ => panic!("Illegal ship bearing!"),
            },
            Action::Forward(distance) => {
                self.pos.x += distance * self.waypoint.x;
                self.pos.y += distance * self.waypoint.y;
            }
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.pos.x.abs() + self.pos.y.abs()) as usize
    }
}

fn parse_action(action_str: &str) -> Action {
    let val = action_str[1..].parse::<i64>().unwrap();
    match action_str.chars().nth(0).unwrap() {
        'N' => Action::North(val),
        'S' => Action::South(val),
        'E' => Action::East(val),
        'W' => Action::West(val),
        'L' => Action::Left(val),
        'R' => Action::Right(val),
        'F' => Action::Forward(val),
        _ => panic!("Illegal action string!"),
    }
}

impl Day12 {
    fn solve_part1(&self, input: Vec<String>) -> usize {
        let mut ship = Ship::new();
        for line in &input {
            let action = parse_action(line);
            ship.navigate_bearing(&action);
        }
        ship.manhattan_distance()
    }

    fn solve_part2(&self, input: Vec<String>) -> usize {
        let mut ship = Ship::new();
        for line in &input {
            let action = parse_action(line);
            ship.navigate_waypoint(&action);
        }
        ship.manhattan_distance()
    }
}

impl Puzzle for Day12 {
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
            Day12 {}.solve_part1(
                vec!("F10", "N3", "F7", "R90", "F11")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            25
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day12 {}.solve_part2(
                vec!("F10", "N3", "F7", "R90", "F11")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            286
        );
    }
}
