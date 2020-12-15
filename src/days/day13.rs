use crate::puzzle::{io, File, Puzzle};
use std::sync::{Arc, Mutex};
use std::thread;
pub struct Day13;

const NOF_WORKERS: i64 = 20;

fn worker(
    worker_id: i64,
    nof_workers: i64,
    buses: &Vec<(i64, i64)>,
    stop: Arc<Mutex<bool>>,
) -> Option<i64> {
    let mut i: i64 = worker_id;
    'outer: loop {
        let time = buses[0].1 * i;
        for (offset, id) in buses {
            if ((time + offset) % id) != 0 {
                i += nof_workers;
                if (i - worker_id) % 10000000 == 0 {
                    if *stop.lock().unwrap() {
                        return None;
                    }
                }
                continue 'outer;
            }
        }
        *stop.lock().unwrap() = true;
        return Some(time);
    }
}

impl Day13 {
    fn solve_part1(&self, input: Vec<String>) -> usize {
        let mut min: usize = std::usize::MAX;
        let mut min_id: usize = 0;
        let earliest = input.iter().nth(0).unwrap().parse::<usize>().unwrap();
        let buses: Vec<_> = input
            .iter()
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>())
            .collect();
        for bus in &buses {
            if let Ok(id) = bus {
                let depart = if (earliest % id) == 0 {
                    earliest
                } else {
                    ((earliest / id) + 1) * id
                };
                if depart < min {
                    min = depart;
                    min_id = *id;
                }
            }
        }
        min_id * (min - earliest)
    }

    fn solve_part2(&self, input: Vec<String>) -> i64 {
        let mut buses: Vec<_> = input
            .iter()
            .nth(1)
            .unwrap()
            .split(",")
            .enumerate()
            .map(|(i, x)| (i, x.parse::<i64>()))
            .filter(|(_, x)| x.is_ok())
            .map(|(i, x)| (i as i64, x.unwrap()))
            .collect();

        buses.sort_by(|(_, id1), (_, id2)| id2.partial_cmp(id1).unwrap());
        let offset_adjustment = buses[0].0;
        buses = buses
            .iter()
            .map(|(offset, id)| (*offset - buses[0].0, *id))
            .collect();

        let results = Arc::new(Mutex::new(vec![]));
        let stop = Arc::new(Mutex::new(false));
        let mut handles = vec![];
        for worker_id in 0..NOF_WORKERS {
            let buses = buses.clone();
            let results = Arc::clone(&results);
            let stop = Arc::clone(&stop);
            let handle = thread::spawn(move || {
                if let Some(timestamp) = worker(worker_id, NOF_WORKERS, &buses, stop) {
                    let mut results = results.lock().unwrap();
                    (*results).push(timestamp);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        (*results).lock().unwrap().sort();
        let timestamp = (*results).lock().unwrap()[0];
        timestamp - offset_adjustment
    }
}

impl Puzzle for Day13 {
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
            Day13 {}.solve_part1(
                vec!("939", "7,13,x,x,59,x,31,19")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            295
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            Day13 {}.solve_part2(
                vec!("939", "7,13,x,x,59,x,31,19")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            1068781
        );
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            Day13 {}.solve_part2(
                vec!("", "67,7,59,61")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            754018
        );
    }

    #[test]
    fn part2_example3() {
        assert_eq!(
            Day13 {}.solve_part2(
                vec!("", "67,x,7,59,61")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            779210
        );
    }

    #[test]
    fn part2_example4() {
        assert_eq!(
            Day13 {}.solve_part2(
                vec!("", "67,7,x,59,61")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            1261476
        );
    }

    #[test]
    fn part2_example5() {
        assert_eq!(
            Day13 {}.solve_part2(
                vec!("", "1789,37,47,1889")
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            1202161486
        );
    }
}
