use std::vec;

use crate::puzzle::{io, File, Puzzle};
pub struct Day16;

enum ParseState {
    Field,
    MyTicket,
    NearbyTickets,
}

#[derive(Debug, Clone)]
struct Field {
    name: String,
    valid_ranges: Vec<(usize, usize)>,
}

impl Field {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            valid_ranges: vec![],
        }
    }
    fn parse(data: &str) -> Field {
        let mut split1 = data.split(":");
        let mut field = Field::new(split1.next().unwrap());
        let ranges_split = split1.next().unwrap().split("or");
        for range in ranges_split {
            let mut range_split = range.split("-");
            let min = range_split.next().unwrap().trim().parse::<usize>().unwrap();
            let max = range_split.next().unwrap().trim().parse::<usize>().unwrap();
            field.valid_ranges.push((min, max));
        }
        field
    }
    fn is_valid_value(&self, val: usize) -> bool {
        for (min, max) in &self.valid_ranges {
            if val >= *min && val <= *max {
                return true;
            }
        }
        return false;
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<Field>, Vec<usize>, Vec<Vec<usize>>) {
    let mut fields = vec![];
    let mut my_ticket = vec![];
    let mut nearby_tickets: Vec<Vec<usize>> = vec![];
    let mut state = ParseState::Field;
    for line in input {
        if line == "" {
            continue;
        }
        match state {
            ParseState::Field => match &line[..] {
                "your ticket:" => state = ParseState::MyTicket,
                _ => fields.push(Field::parse(&line)),
            },
            ParseState::MyTicket => match &line[..] {
                "nearby tickets:" => state = ParseState::NearbyTickets,
                _ => {
                    my_ticket = line
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect()
                }
            },
            ParseState::NearbyTickets => {
                nearby_tickets.push(
                    line.split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect(),
                );
            }
        }
    }
    (fields, my_ticket, nearby_tickets)
}

impl Day16 {
    fn solve_part1(&self, input: Vec<String>) -> usize {
        let (fields, _, nearby_tickets) = parse_input(&input);
        let mut ticket_scanning_error_rate = 0;
        for ticket in &nearby_tickets {
            'value_loop: for val in ticket {
                for field in &fields {
                    if field.is_valid_value(*val) {
                        continue 'value_loop;
                    }
                }
                ticket_scanning_error_rate += val;
            }
        }
        ticket_scanning_error_rate
    }

    fn solve_part2(&self, input: Vec<String>, field_name: &str) -> usize {
        // Parse input
        let (fields, my_ticket, mut nearby_tickets) = parse_input(&input);

        // Initialize each position in the vector with all possible candidate fields.
        let mut positions = vec![fields.clone(); my_ticket.len()];

        // Clean out invalid tickets from nearby tickets vector.
        nearby_tickets.retain(|ticket| {
            'value_loop: for val in ticket {
                for field in &fields {
                    if field.is_valid_value(*val) {
                        continue 'value_loop;
                    }
                }
                return false;
            }
            return true;
        });

        // Go through nearby tickets and remove position candidates for which values are not valid.
        for ticket in &nearby_tickets {
            for (pos, val) in ticket.iter().enumerate() {
                positions[pos].retain(|field| field.is_valid_value(*val));
            }
        }

        // When there is a single candidate left in one position, remove that candidate from all other positions.
        // Keep doing this until no changes are made.
        let mut go_again = true;
        while go_again {
            go_again = false;
            for (i, candidates) in positions.clone().iter().enumerate() {
                if candidates.len() == 1 {
                    let field_found = &candidates[0];
                    for (i2, candidates2) in positions.iter_mut().enumerate() {
                        if i != i2 {
                            candidates2.retain(|field| {
                                if field.name != field_found.name {
                                    true
                                } else {
                                    go_again = true;
                                    false
                                }
                            });
                        }
                    }
                }
            }
        }

        // Assert that there is only a single field left for each position.
        // Also map from vector of candidates to a single Field.
        let positions: Vec<Field> = positions
            .iter()
            .map(|candidates| {
                assert_eq!(1, candidates.len());
                candidates[0].clone()
            })
            .collect();

        // Go through my tickets values and multiply all fields where the field name starts with
        // the specified string.
        my_ticket
            .iter()
            .zip(positions.iter())
            .fold(1, |acc, (val, field)| {
                if field.name.starts_with(field_name) {
                    acc * val
                } else {
                    acc
                }
            })
    }
}

impl Puzzle for Day16 {
    fn solve(&self, lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let input: Vec<String> = lines.expect("No input file").map(|l| l.unwrap()).collect();
        return (
            self.solve_part1(input.clone()).to_string(),
            self.solve_part2(input.clone(), "departure").to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day16 {}.solve_part1(
                vec!(
                    "class: 1-3 or 5-7",
                    "row: 6-11 or 33-44",
                    "seat: 13-40 or 45-50",
                    "",
                    "your ticket:",
                    "7,1,14",
                    "",
                    "nearby tickets:",
                    "7,3,47",
                    "40,4,50",
                    "55,2,20",
                    "38,6,12"
                )
                .iter()
                .map(|x| x.to_string())
                .collect()
            ),
            71
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day16 {}.solve_part2(
                vec!(
                    "class: 0-1 or 4-19",
                    "row: 0-5 or 8-19",
                    "seat: 0-13 or 16-19",
                    "",
                    "your ticket:",
                    "11,12,13",
                    "",
                    "nearby tickets:",
                    "3,9,18",
                    "15,1,5",
                    "5,14,9",
                )
                .iter()
                .map(|x| x.to_string())
                .collect(),
                "class"
            ),
            12
        );
    }
}
