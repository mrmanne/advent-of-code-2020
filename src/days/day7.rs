use std::collections::HashMap;
use std::vec;

use crate::puzzle::{io, File, Puzzle};
pub struct Day7;

#[derive(Debug)]
struct Bag {
    color: String,
    contents: Vec<(usize, String)>,
}

impl Bag {
    pub fn new(color: &str) -> Self {
        Bag {
            color: color.to_string(),
            contents: vec![],
        }
    }

    pub fn add_bag(&mut self, num: usize, color: &str) {
        self.contents.push((num, color.to_string()));
    }
}

fn contains(bag: &Bag, rules: &HashMap<&str, Bag>, find_color: &str) -> bool {
    if bag.color == find_color {
        return true;
    }
    for (_, color) in &bag.contents {
        if let Some(small_bag) = rules.get(&color[..]) {
            if contains(small_bag, rules, find_color) {
                return true;
            }
        }
    }
    false
}

fn count_bags(bag: &Bag, rules: &HashMap<&str, Bag>) -> usize {
    if bag.contents.is_empty() {
        return 1;
    }
    let mut sum: usize = 1;
    for (num, color) in &bag.contents {
        let small_bag = rules.get(&color[..]).unwrap();
        sum += num * count_bags(small_bag, rules);
    }
    sum
}

fn parse_rules(input: &Vec<String>) -> HashMap<&str, Bag> {
    let mut rules = HashMap::new();
    for line in input {
        let mut split1 = line[..].split(" bags contain ");
        let color = split1.next().unwrap().trim_end();
        let mut bag = Bag::new(&color);
        let rest = split1.next().unwrap();
        if rest != "no other bags." {
            let contents = rest.split(", ");
            for part in contents {
                let part_color = part
                    .split(" bag")
                    .next()
                    .unwrap()
                    .split(char::is_numeric)
                    .next_back()
                    .unwrap()
                    .trim();
                let num: usize = part.split(' ').next().unwrap().parse::<usize>().unwrap();
                bag.add_bag(num, part_color);
            }
        }
        rules.insert(color, bag);
    }
    rules
}

impl Day7 {
    fn solve_part1(&self, input: &Vec<String>, find_color: &str) -> usize {
        let rules = parse_rules(input);
        let mut count: usize = 0;
        for (_, bag) in &rules {
            if bag.color != find_color && contains(bag, &rules, find_color) {
                count += 1;
            }
        }
        count
    }

    fn solve_part2(&self, input: &Vec<String>, find_color: &str) -> usize {
        let rules = parse_rules(input);
        let bag = rules.get(find_color).unwrap();
        count_bags(bag, &rules) - 1
    }
}

impl Puzzle for Day7 {
    fn solve(&self, lines: io::Result<io::Lines<io::BufReader<File>>>) -> (String, String) {
        let rules: Vec<String> = lines.expect("No input file").map(|l| l.unwrap()).collect();
        return (
            self.solve_part1(&rules, "shiny gold").to_string(),
            self.solve_part2(&rules, "shiny gold").to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day7 {}.solve_part1(
                &vec!(
                    "light red bags contain 1 bright white bag, 2 muted yellow bags.",
                    "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
                    "bright white bags contain 1 shiny gold bag.",
                    "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
                    "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
                    "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
                    "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                    "faded blue bags contain no other bags.",
                    "dotted black bags contain no other bags."
                )
                .iter()
                .map(|x| x.to_string())
                .collect(),
                "shiny gold"
            ),
            4
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day7 {}.solve_part2(
                &vec!(
                    "light red bags contain 1 bright white bag, 2 muted yellow bags.",
                    "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
                    "bright white bags contain 1 shiny gold bag.",
                    "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
                    "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
                    "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
                    "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                    "faded blue bags contain no other bags.",
                    "dotted black bags contain no other bags."
                )
                .iter()
                .map(|x| x.to_string())
                .collect(),
                "shiny gold"
            ),
            32
        );
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            Day7 {}.solve_part2(
                &vec!(
                    "shiny gold bags contain 2 dark red bags.",
                    "dark red bags contain 2 dark orange bags.",
                    "dark orange bags contain 2 dark yellow bags.",
                    "dark yellow bags contain 2 dark green bags.",
                    "dark green bags contain 2 dark blue bags.",
                    "dark blue bags contain 2 dark violet bags.",
                    "dark violet bags contain no other bags."
                )
                .iter()
                .map(|x| x.to_string())
                .collect(),
                "shiny gold"
            ),
            126
        );
    }
}
