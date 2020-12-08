use std::vec;

use crate::puzzle::{io, File, Puzzle};
pub struct Day8;

enum ExitStatus {
    Success(i64),
    Stuck(i64),
}

#[derive(Debug, Clone)]
enum Op {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

struct Computer {
    mem: Vec<(usize, Op)>,
    pc: usize,
    acc: i64,
}

impl Computer {
    pub fn new(program: Vec<(usize, Op)>) -> Self {
        Self {
            mem: program,
            pc: 0,
            acc: 0,
        }
    }

    pub fn execute_op(&mut self) -> Option<ExitStatus> {
        let pc_start = self.pc;
        if self.pc == self.mem.len() {
            return Some(ExitStatus::Success(self.acc));
        }

        let (calls, op) = &mut self.mem[self.pc];
        if *calls > 0 {
            return Some(ExitStatus::Stuck(self.acc));
        }
        *calls += 1;

        match op {
            Op::Acc(val) => {
                self.acc += *val;
            }
            Op::Jmp(offset) => {
                let new_pc = self.pc as i64 + *offset;
                if new_pc < 0 {
                    panic!("Illegal jump offset");
                }
                self.pc = new_pc as usize;
            }
            Op::Nop(_) => (),
        }
        if pc_start == self.pc {
            self.pc += 1;
        }
        None
    }

    pub fn run(&mut self) -> ExitStatus {
        loop {
            match self.execute_op() {
                Some(exit_status) => return exit_status,
                None => (),
            }
        }
    }
}

fn get_program(input: &Vec<String>) -> Vec<(usize, Op)> {
    let mut program = vec![];
    for line in input {
        let mut words = line.split(" ");
        let opstring = words.next().unwrap().trim();
        let argument = words.next().unwrap().trim();
        let op = match opstring {
            "acc" => Op::Acc(argument.parse().unwrap()),
            "jmp" => Op::Jmp(argument.parse().unwrap()),
            "nop" => Op::Nop(argument.parse().unwrap()),
            _ => panic!("Illegal instruction: {}", opstring),
        };
        program.push((0, op));
    }
    program
}

impl Day8 {
    fn solve_part1(&self, input: &Vec<String>) -> i64 {
        let mut c = Computer::new(get_program(input));
        match c.run() {
            ExitStatus::Success(acc) => acc,
            ExitStatus::Stuck(acc) => acc,
        }
    }

    fn solve_part2(&self, input: &Vec<String>) -> i64 {
        let program = get_program(input);
        for (i, (_, op)) in program.iter().enumerate() {
            let mut modified_program = program.clone();
            match op {
                Op::Jmp(val) => modified_program[i] = (0, Op::Nop(*val)),
                Op::Nop(val) => modified_program[i] = (0, Op::Jmp(*val)),
                _ => (),
            }
            let mut c = Computer::new(modified_program);
            match c.run() {
                ExitStatus::Success(acc) => return acc,
                ExitStatus::Stuck(_) => (),
            }
        }
        -1
    }
}

impl Puzzle for Day8 {
    fn solve(&self, lines: io::Lines<io::BufReader<File>>) -> (String, String) {
        let rules: Vec<String> = lines.map(|l| l.unwrap()).collect();
        return (
            self.solve_part1(&rules).to_string(),
            self.solve_part2(&rules).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day8 {}.solve_part1(
                &vec!(
                    "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1",
                    "jmp -4", "acc +6",
                )
                .iter()
                .map(|x| x.to_string())
                .collect(),
            ),
            5
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day8 {}.solve_part2(
                &vec!(
                    "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1",
                    "jmp -4", "acc +6",
                )
                .iter()
                .map(|x| x.to_string())
                .collect(),
            ),
            8
        );
    }
}
