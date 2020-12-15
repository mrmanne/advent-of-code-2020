use std::collections::HashMap;
use std::vec;

use crate::puzzle::{io, File, Puzzle};
pub struct Day14;

#[derive(Debug, Clone, Copy)]
enum Op {
    Mask(u64, u64, u64),
    Write(u64, u64),
}

fn get_program(input: &Vec<String>) -> Vec<Op> {
    let mut program = vec![];
    for line in input {
        let mut words = line.split(" ");
        let cmd = words.nth(0).unwrap();
        if cmd == "mask" {
            let mask = words.nth(1).unwrap();
            let mut clear_mask = 0;
            let mut set_mask = 0;
            let mut x_mask = 0;
            for i in 0..mask.len() {
                match mask.chars().nth(i).unwrap() {
                    'X' => x_mask |= 1 << (35 - i),
                    '0' => clear_mask |= 1 << (35 - i),
                    '1' => set_mask |= 1 << (35 - i),
                    _ => panic!("Illegal mask!"),
                }
            }
            program.push(Op::Mask(clear_mask, set_mask, x_mask));
        } else {
            let val = words.nth(1).unwrap().parse::<u64>().unwrap();
            let mut addr_split = cmd.split(|c| c == '[' || c == ']');
            let addr = addr_split.nth(1).unwrap().parse::<u64>().unwrap();
            program.push(Op::Write(addr, val));
        }
    }
    program
}

fn floating_write(
    val: u64,
    addr: u64,
    floating_mask: u64,
    offset: u64,
    mut mem: &mut HashMap<u64, u64>,
) {
    if offset == 36 {
        mem.insert(addr, val);
    } else if (floating_mask >> offset) & 0x1 == 1 {
        floating_write(
            val,
            addr | (1 << offset),
            floating_mask,
            offset + 1,
            &mut mem,
        );
        floating_write(
            val,
            addr & !(1 << offset),
            floating_mask,
            offset + 1,
            &mut mem,
        );
    } else {
        floating_write(val, addr, floating_mask, offset + 1, &mut mem);
    }
}

impl Day14 {
    fn solve_part1(&self, input: Vec<String>) -> usize {
        let program = get_program(&input);
        let mut mem: HashMap<u64, u64> = HashMap::new();
        let mut clear_mask: u64 = 0;
        let mut set_mask: u64 = 0;
        for op in program {
            match op {
                Op::Mask(clear_mask_op, set_mask_op, _) => {
                    clear_mask = !clear_mask_op;
                    set_mask = set_mask_op;
                }
                Op::Write(addr, mut val) => {
                    val &= clear_mask;
                    val |= set_mask;
                    mem.insert(addr, val);
                }
            }
        }
        mem.iter().fold(0, |acc, (_, v)| acc + *v) as usize
    }

    fn solve_part2(&self, input: Vec<String>) -> usize {
        let program = get_program(&input);
        let mut mem: HashMap<u64, u64> = HashMap::new();
        let mut floating_mask: u64 = 0;
        let mut set_mask: u64 = 0;
        for op in &program {
            match op {
                Op::Mask(_, set_mask_op, floating_mask_op) => {
                    set_mask = *set_mask_op;
                    floating_mask = *floating_mask_op;
                }
                Op::Write(mut addr, val) => {
                    addr |= set_mask;
                    floating_write(*val, addr, floating_mask, 0, &mut mem);
                }
            }
        }
        mem.iter().fold(0, |acc, (_, v)| acc + *v) as usize
    }
}

impl Puzzle for Day14 {
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
            Day14 {}.solve_part1(
                vec!(
                    "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
                    "mem[8] = 11",
                    "mem[7] = 101",
                    "mem[8] = 0"
                )
                .iter()
                .map(|x| x.to_string())
                .collect()
            ),
            165
        );
    }
    #[test]
    fn part2_example1() {
        assert_eq!(
            Day14 {}.solve_part2(
                vec!(
                    "mask = 000000000000000000000000000000X1001X",
                    "mem[42] = 100",
                    "mask = 00000000000000000000000000000000X0XX",
                    "mem[26] = 1"
                )
                .iter()
                .map(|x| x.to_string())
                .collect()
            ),
            208
        );
    }
}
