use crate::puzzle::{io, File, Puzzle};
pub struct Day18;

#[derive(Debug)]
enum Op {
    Val(i64),
    Multiply,
    Add,
}

fn parse_op(c: char) -> Op {
    match c {
        '*' => Op::Multiply,
        '+' => Op::Add,
        _ => panic!("Unexpected operation!"),
    }
}

fn get_ops<F>(expr: &str, calc: F) -> Vec<Op>
where
    F: Fn(&str) -> i64,
{
    let mut operations = vec![];
    let mut i = 0;
    while i < expr.len() {
        let (val, len) = if expr.chars().nth(i).unwrap() == '(' {
            let mut opens = 1;
            let mut len = 0;
            for (j, c) in expr[i + 1..].chars().enumerate() {
                match c {
                    '(' => opens += 1,
                    ')' => {
                        opens -= 1;
                        if opens == 0 {
                            len = j + 1;
                            break;
                        }
                    }
                    _ => (),
                }
            }
            (calc(&expr[i + 1..i + len]), len + 1)
        } else {
            (expr.chars().nth(i).unwrap().to_digit(10).unwrap() as i64, 1)
        };
        operations.push(Op::Val(val));
        i += len;

        if i >= expr.len() {
            break;
        }
        operations.push(parse_op(expr.chars().nth(i + 1).unwrap()));
        i += 3;
    }
    operations
}

fn calc_left_to_right(expr: &str) -> i64 {
    let operations = get_ops(&expr, calc_left_to_right);

    let mut result = match operations[0] {
        Op::Val(x) => x,
        _ => panic!("Unexpected operation!"),
    };
    for i in (1..operations.len() - 1).step_by(2) {
        let rhs = match operations[i + 1] {
            Op::Val(x) => x,
            _ => panic!("Unexpected operation!"),
        };
        match operations[i] {
            Op::Multiply => result = result * rhs,
            Op::Add => result = result + rhs,
            _ => panic!("Unexpected operation!"),
        }
    }
    result
}

fn calc_add_before_mul(expr: &str) -> i64 {
    let mut operations = get_ops(&expr, calc_add_before_mul);

    // First evaluate all 'add' operations.
    let mut i = 1;
    while i < operations.len() - 1 {
        let a = match operations[i - 1] {
            Op::Val(x) => x,
            _ => panic!("Unexpected operation!"),
        };
        let b = match operations[i + 1] {
            Op::Val(x) => x,
            _ => panic!("Unexpected operation!"),
        };
        match operations[i] {
            Op::Add => {
                operations[i - 1] = Op::Val(a + b);
                operations.remove(i);
                operations.remove(i);
            }
            _ => i += 2,
        }
    }

    // Now that only 'multiply' operations remain, filter out all Val(i64) then just multiply them with fold().
    operations
        .iter()
        .filter(|op| match op {
            Op::Val(_) => true,
            _ => false,
        })
        .map(|op| match op {
            Op::Val(v) => *v,
            _ => panic!("Unexpected operation!"),
        })
        .fold(1, |sum, x| sum * x)
}

impl Day18 {
    fn solve_part1(&self, input: Vec<String>) -> i64 {
        input
            .iter()
            .fold(0, |sum, expr| sum + calc_left_to_right(&expr))
    }

    fn solve_part2(&self, input: Vec<String>) -> i64 {
        input
            .iter()
            .fold(0, |sum, expr| sum + calc_add_before_mul(&expr))
    }
}

impl Puzzle for Day18 {
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
    fn part1_examples() {
        assert_eq!(
            Day18 {}.solve_part1(string_vec!("1 + 2 * 3 + 4 * 5 + 6")),
            71
        );
        assert_eq!(
            Day18 {}.solve_part1(string_vec!("1 + (2 * 3) + (4 * (5 + 6))")),
            51
        );
        assert_eq!(Day18 {}.solve_part1(string_vec!("2 * 3 + (4 * 5)")), 26);
        assert_eq!(
            Day18 {}.solve_part1(string_vec!("5 + (8 * 3 + 9 + 3 * 4 * 3)")),
            437
        );
        assert_eq!(
            Day18 {}.solve_part1(string_vec!("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
            12240
        );
        assert_eq!(
            Day18 {}.solve_part1(string_vec!(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            )),
            13632
        );
    }

    #[test]
    fn part2_examples() {
        assert_eq!(
            Day18 {}.solve_part2(string_vec!("1 + 2 * 3 + 4 * 5 + 6")),
            231
        );
        assert_eq!(
            Day18 {}.solve_part2(string_vec!("1 + (2 * 3) + (4 * (5 + 6))")),
            51
        );
        assert_eq!(Day18 {}.solve_part2(string_vec!("2 * 3 + (4 * 5)")), 46);
        assert_eq!(
            Day18 {}.solve_part2(string_vec!("5 + (8 * 3 + 9 + 3 * 4 * 3)")),
            1445
        );
        assert_eq!(
            Day18 {}.solve_part2(string_vec!("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
            669060
        );
        assert_eq!(
            Day18 {}.solve_part2(string_vec!(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            )),
            23340
        );
    }
}
