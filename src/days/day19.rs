use std::collections::HashMap;

use crate::puzzle::{io, File, Puzzle};
pub struct Day19;

enum ParseState {
    Rules,
    Messages,
}

// Check if an expression partly or fully matches a rule. Returns 'Some' vector of the match lengths
// when matches are found. Otherwise it returns 'None'.
fn get_matches(expr: &str, rule_id: usize, rules: &HashMap<usize, String>) -> Option<Vec<usize>> {
    let mut match_lengths = vec![];
    if expr == "" {
        return None;
    }
    let rule = rules.get(&rule_id).unwrap();
    if rule[..].contains("\"") {
        let c = rule[..].split("\"").nth(1).unwrap().chars().nth(0).unwrap();
        if expr.chars().nth(0).unwrap() == c {
            match_lengths.push(1);
            return Some(match_lengths);
        } else {
            return None;
        }
    }
    let or_rules: Vec<&str> = rule.split("|").collect();
    for or_rule in or_rules {
        let rule_ids: Vec<usize> = or_rule
            .trim()
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut start_idxs = vec![0];
        for sub_rule_id in rule_ids {
            let mut next_start_idxs = vec![];
            for i in start_idxs {
                if let Some(sub_match_lengths) = get_matches(&expr[i..], sub_rule_id, rules) {
                    for len in sub_match_lengths {
                        next_start_idxs.push(i + len);
                    }
                }
            }
            start_idxs = next_start_idxs;
        }
        match_lengths.append(&mut start_idxs);
    }
    if !match_lengths.is_empty() {
        Some(match_lengths)
    } else {
        None
    }
}

fn match_rule(expr: &str, rule_id: usize, rules: &HashMap<usize, String>) -> bool {
    if let Some(match_lengths) = get_matches(&expr, rule_id, &rules) {
        for len in match_lengths {
            if len == expr.len() {
                return true;
            }
        }
    }
    return false;
}

fn parse_input(input: &Vec<String>) -> (HashMap<usize, String>, Vec<String>) {
    let mut parse_state = ParseState::Rules;
    let mut rules = HashMap::new();
    let mut messages = vec![];
    for line in input {
        if line == "" {
            parse_state = ParseState::Messages;
            continue;
        }
        match parse_state {
            ParseState::Rules => {
                let tokens: Vec<&str> = line.split(":").collect();
                let id = tokens[0].parse::<usize>().unwrap();
                let expr = tokens[1].trim().to_string();
                rules.insert(id, expr);
            }
            ParseState::Messages => messages.push(line.to_string()),
        }
    }
    (rules, messages)
}

impl Day19 {
    fn solve_part1(&self, input: Vec<String>) -> usize {
        let (rules, messages) = parse_input(&input);
        messages.iter().fold(0, |nof_matches, message| {
            if match_rule(message, 0, &rules) {
                nof_matches + 1
            } else {
                nof_matches
            }
        })
    }

    fn solve_part2(&self, input: Vec<String>) -> usize {
        let (mut rules, messages) = parse_input(&input);
        rules.insert(8, "42 | 42 8".to_string());
        rules.insert(11, "42 31 | 42 11 31".to_string());
        messages.iter().fold(0, |nof_matches, message| {
            if match_rule(message, 0, &rules) {
                nof_matches + 1
            } else {
                nof_matches
            }
        })
    }
}

impl Puzzle for Day19 {
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
            Day19 {}.solve_part1(string_vec!(
                "0: 1 2",
                "1: \"a\"",
                "2: 1 3 | 3 1",
                "3: \"b\"",
                "",
                "aab",
                "aba",
                "abab",
                "a",
                "abb"
            )),
            2
        );
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            Day19 {}.solve_part1(string_vec!(
                "0: 4 1 5",
                "1: 2 3 | 3 2",
                "2: 4 4 | 5 5",
                "3: 4 5 | 5 4",
                "4: \"a\"",
                "5: \"b\"",
                "",
                "ababbb",
                "bababa",
                "abbbab",
                "aaabbb",
                "aaaabbb"
            )),
            2
        );
    }

    #[test]
    fn part1_example3() {
        assert_eq!(
            Day19 {}.solve_part1(string_vec!(
                "42: 9 14 | 10 1",
                "9: 14 27 | 1 26",
                "10: 23 14 | 28 1",
                "1: \"a\"",
                "11: 42 31",
                "5: 1 14 | 15 1",
                "19: 14 1 | 14 14",
                "12: 24 14 | 19 1",
                "16: 15 1 | 14 14",
                "31: 14 17 | 1 13",
                "6: 14 14 | 1 14",
                "2: 1 24 | 14 4",
                "0: 8 11",
                "13: 14 3 | 1 12",
                "15: 1 | 14",
                "17: 14 2 | 1 7",
                "23: 25 1 | 22 14",
                "28: 16 1",
                "4: 1 1",
                "20: 14 14 | 1 15",
                "3: 5 14 | 16 1",
                "27: 1 6 | 14 18",
                "14: \"b\"",
                "21: 14 1 | 1 14",
                "25: 1 1 | 1 14",
                "22: 14 14",
                "8: 42",
                "26: 14 22 | 1 20",
                "18: 15 15",
                "7: 14 5 | 1 21",
                "24: 14 1",
                "",
                "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
                "bbabbbbaabaabba",
                "babbbbaabbbbbabbbbbbaabaaabaaa",
                "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
                "bbbbbbbaaaabbbbaaabbabaaa",
                "bbbababbbbaaaaaaaabbababaaababaabab",
                "ababaaaaaabaaab",
                "ababaaaaabbbaba",
                "baabbaaaabbaaaababbaababb",
                "abbbbabbbbaaaababbbbbbaaaababb",
                "aaaaabbaabaaaaababaa",
                "aaaabbaaaabbaaa",
                "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
                "babaaabbbaaabaababbaabababaaab",
                "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            )),
            3
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day19 {}.solve_part2(string_vec!(
                "42: 9 14 | 10 1",
                "9: 14 27 | 1 26",
                "10: 23 14 | 28 1",
                "1: \"a\"",
                "11: 42 31",
                "5: 1 14 | 15 1",
                "19: 14 1 | 14 14",
                "12: 24 14 | 19 1",
                "16: 15 1 | 14 14",
                "31: 14 17 | 1 13",
                "6: 14 14 | 1 14",
                "2: 1 24 | 14 4",
                "0: 8 11",
                "13: 14 3 | 1 12",
                "15: 1 | 14",
                "17: 14 2 | 1 7",
                "23: 25 1 | 22 14",
                "28: 16 1",
                "4: 1 1",
                "20: 14 14 | 1 15",
                "3: 5 14 | 16 1",
                "27: 1 6 | 14 18",
                "14: \"b\"",
                "21: 14 1 | 1 14",
                "25: 1 1 | 1 14",
                "22: 14 14",
                "8: 42",
                "26: 14 22 | 1 20",
                "18: 15 15",
                "7: 14 5 | 1 21",
                "24: 14 1",
                "",
                "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
                "bbabbbbaabaabba",
                "babbbbaabbbbbabbbbbbaabaaabaaa",
                "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
                "bbbbbbbaaaabbbbaaabbabaaa",
                "bbbababbbbaaaaaaaabbababaaababaabab",
                "ababaaaaaabaaab",
                "ababaaaaabbbaba",
                "baabbaaaabbaaaababbaababb",
                "abbbbabbbbaaaababbbbbbaaaababb",
                "aaaaabbaabaaaaababaa",
                "aaaabbaaaabbaaa",
                "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
                "babaaabbbaaabaababbaabababaaab",
                "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            )),
            12
        );
    }
}
