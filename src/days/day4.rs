use crate::puzzle::{io, File, Puzzle};
pub struct Day4;

enum Field {
    BYR = 0,
    IYR = 1,
    EYR = 2,
    HGT = 3,
    HCL = 4,
    ECL = 5,
    PID = 6,
    CID = 7,
}

fn is_year_valid(value: &str, min: usize, max: usize) -> bool {
    if let Ok(year) = value.parse::<usize>() {
        if year >= min && year <= max {
            return true;
        }
    }
    false
}

fn is_hgt_valid(value: &str) -> bool {
    let index = match value.find(char::is_alphabetic) {
        Some(val) => val,
        None => return false,
    };
    let (num, unit) = value.split_at(index);
    let num = num.parse::<usize>().unwrap();
    if unit == "cm" && num >= 150 && num <= 193 {
        return true;
    }
    if unit == "in" && num >= 59 && num <= 76 {
        return true;
    }
    false
}

fn is_hcl_valid(value: &str) -> bool {
    let valid_chars = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    if value.chars().nth(0).unwrap() != '#' {
        return false;
    }
    if value.len() != 7 {
        return false;
    }
    for c in value[1..].chars() {
        if !valid_chars.contains(&c) {
            return false;
        }
    }
    true
}

fn is_ecl_valid(value: &str) -> bool {
    match value {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn is_pid_valid(value: &str) -> bool {
    if value.len() != 9 {
        return false;
    }
    for c in value.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

fn get_key_id(key: &str) -> Option<Field> {
    match key {
        "byr" => Some(Field::BYR),
        "iyr" => Some(Field::IYR),
        "eyr" => Some(Field::EYR),
        "hgt" => Some(Field::HGT),
        "hcl" => Some(Field::HCL),
        "ecl" => Some(Field::ECL),
        "pid" => Some(Field::PID),
        "cid" => Some(Field::CID),
        _ => None,
    }
}

fn validate_entry(key: &str, value: &str, validate_value: bool) -> Option<Field> {
    let key_id = get_key_id(key);
    if !validate_value {
        return key_id;
    }
    let ok = match key_id {
        Some(Field::BYR) => is_year_valid(value, 1920, 2002),
        Some(Field::IYR) => is_year_valid(value, 2010, 2020),
        Some(Field::EYR) => is_year_valid(value, 2020, 2030),
        Some(Field::HGT) => is_hgt_valid(value),
        Some(Field::HCL) => is_hcl_valid(value),
        Some(Field::ECL) => is_ecl_valid(value),
        Some(Field::PID) => is_pid_valid(value),
        Some(Field::CID) => true,
        None => false,
    };
    if ok {
        key_id
    } else {
        None
    }
}

fn nof_valid_passports(input: &Vec<String>, validate_values: bool) -> usize {
    let mut nof_ok_passports: usize = 0;
    let mut fields: u8 = 0;
    for line in input {
        if line.is_empty() {
            if fields == 0xff || fields == 0x7f {
                nof_ok_passports += 1;
            }
            fields = 0;
        } else {
            let tokens = line.split(" ");
            for token in tokens {
                let mut key_value = token.split(":");
                let key = key_value.next().unwrap();
                let value = key_value.next().unwrap();
                if let Some(key_id) = validate_entry(key, value, validate_values) {
                    fields |= 1 << key_id as u8;
                }
            }
        }
    }
    if fields == 0xff || fields == 0x7f {
        nof_ok_passports += 1;
    }
    nof_ok_passports
}

impl Day4 {
    fn solve_part1(&self, input: &Vec<String>) -> usize {
        nof_valid_passports(input, false)
    }

    fn solve_part2(&self, input: &Vec<String>) -> usize {
        nof_valid_passports(input, true)
    }
}

impl Puzzle for Day4 {
    fn solve(&self, lines: io::Lines<io::BufReader<File>>) -> (String, String) {
        let input: Vec<String> = lines.map(|l| l.unwrap()).collect();
        return (
            self.solve_part1(&input).to_string(),
            self.solve_part2(&input).to_string(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            Day4 {}.solve_part1(
                &vec!(
                    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
                    "byr:1937 iyr:2017 cid:147 hgt:183cm",
                    "",
                    "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
                    "hcl:#cfa07d byr:1929",
                    "",
                    "hcl:#ae17e1 iyr:2013",
                    "eyr:2024",
                    "ecl:brn pid:760753108 byr:1931",
                    "hgt:179cm",
                    "",
                    "hcl:#cfa07d eyr:2025 pid:166559648",
                    "iyr:2011 ecl:brn hgt:59in",
                )
                .iter()
                .map(|x| x.to_string())
                .collect()
            ),
            2
        );
    }

    #[test]
    fn part2_hgt1() {
        assert_eq!(is_hgt_valid("60in"), true);
        assert_eq!(is_hgt_valid("190cm"), true);
        assert_eq!(is_hgt_valid("190in"), false);
        assert_eq!(is_hgt_valid("190"), false);
    }

    #[test]
    fn part2_hcl1() {
        assert_eq!(is_hcl_valid("#123abc"), true);
        assert_eq!(is_hcl_valid("#123abz"), false);
        assert_eq!(is_hcl_valid("123abc"), false);
        assert_eq!(is_hcl_valid("#623a2f"), true);
    }

    #[test]
    fn part2_ecl1() {
        assert_eq!(is_ecl_valid("brn"), true);
        assert_eq!(is_ecl_valid("wat"), false);
    }

    #[test]
    fn part2_pid1() {
        assert_eq!(is_pid_valid("000000001"), true);
        assert_eq!(is_pid_valid("0123456789"), false);
    }

    #[test]
    fn part2_invalid_examples() {
        assert_eq!(
            Day4 {}.solve_part2(
                &vec!(
                    "eyr:1972 cid:100",
                    "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
                    "",
                    "iyr:2019",
                    "hcl:#602927 eyr:1967 hgt:170cm",
                    "ecl:grn pid:012533040 byr:1946",
                    "",
                    "hcl:dab227 iyr:2012",
                    "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
                    "",
                    "hgt:59cm ecl:zzz",
                    "eyr:2038 hcl:74454a iyr:2023",
                    "pid:3556412378 byr:2007"
                )
                .iter()
                .map(|x| x.to_string())
                .collect()
            ),
            0
        );
    }

    #[test]
    fn part2_valid_examples() {
        assert_eq!(
            Day4 {}.solve_part2(
                &vec!(
                    "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
                    "hcl:#623a2f",
                    "",
                    "eyr:2029 ecl:blu cid:129 byr:1989",
                    "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
                    "",
                    "hcl:#888785",
                    "hgt:164cm byr:2001 iyr:2015 cid:88",
                    "pid:545766238 ecl:hzl",
                    "eyr:2022",
                    "",
                    "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
                )
                .iter()
                .map(|x| x.to_string())
                .collect()
            ),
            4
        );
    }
}
