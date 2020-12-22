use std::collections::HashSet;

use crate::puzzle::{io, File, Puzzle};

pub struct Day22;

fn play(decks: &mut Vec<Vec<usize>>, recursive: bool) -> usize {
    let mut dp: HashSet<Vec<Vec<usize>>> = HashSet::new();
    let mut winner = 0;
    loop {
        if let Some(_) = dp.get(decks) {
            winner = 0;
            break;
        }
        dp.insert(decks.clone());
        if decks[0].is_empty() || decks[1].is_empty() {
            break;
        }
        winner = 0;
        let card0 = decks[0].remove(0);
        let card1 = decks[1].remove(0);
        if recursive && decks[0].len() >= card0 && decks[1].len() >= card1 {
            let mut new_deck = vec![];
            new_deck.push(decks[0][..card0].to_vec());
            new_deck.push(decks[1][..card1].to_vec());
            winner = play(&mut new_deck, recursive);
        } else if card1 > card0 {
            winner = 1;
        }
        if winner == 0 {
            decks[winner].push(card0);
            decks[winner].push(card1);
        } else {
            decks[winner].push(card1);
            decks[winner].push(card0);
        }
    }
    winner
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<usize>> {
    let mut decks = vec![vec![]; 2];
    let mut current_deck = 0;
    for line in input {
        if line.contains("Player 2:") {
            current_deck = 1;
            continue;
        }
        if line == "" || line.contains("Player 1:") {
            continue;
        }
        decks[current_deck].push(line.parse::<usize>().unwrap())
    }
    decks
}

impl Day22 {
    fn solve_part1(&self, input: Vec<String>) -> usize {
        let mut decks = parse_input(&input);
        let winner = play(&mut decks, false);
        decks[winner]
            .iter()
            .enumerate()
            .fold(0, |sum, (i, card)| sum + card * (decks[winner].len() - i))
    }

    fn solve_part2(&self, input: Vec<String>) -> usize {
        let mut decks = parse_input(&input);
        let winner = play(&mut decks, true);
        decks[winner]
            .iter()
            .enumerate()
            .fold(0, |sum, (i, card)| sum + card * (decks[winner].len() - i))
    }
}

impl Puzzle for Day22 {
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
            Day22 {}.solve_part1(string_vec!(
                "Player 1:",
                "9",
                "2",
                "6",
                "3",
                "1",
                "",
                "Player 2:",
                "5",
                "8",
                "4",
                "7",
                "10"
            )),
            306
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day22 {}.solve_part2(string_vec!(
                "Player 1:",
                "9",
                "2",
                "6",
                "3",
                "1",
                "",
                "Player 2:",
                "5",
                "8",
                "4",
                "7",
                "10"
            )),
            291
        );
    }
}
