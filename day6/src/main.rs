use common::{init_logs, read_lines};
use std::iter::zip;
use std::path::{Path, PathBuf};

// Below imports some commonly used crates
// currently: (regex::Regex, num::Integer, rayon::prelude, itertools::Itertools)
// use common::prelude::*;

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    init_logs();
    puzzle1(&input_path);
    puzzle2(&input_path);
}

fn puzzle1(input_path: &Path) -> u64 {
    let input: Vec<_> = read_lines(input_path).unwrap().collect();
    let times: Vec<_> = input[0]
        .as_ref()
        .unwrap()
        .as_str()
        .split_ascii_whitespace()
        .skip(1)
        .map(|d| d.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<_> = input[1]
        .as_ref()
        .unwrap()
        .as_str()
        .split_ascii_whitespace()
        .skip(1)
        .map(|d| d.parse::<u64>().unwrap())
        .collect();

    let wincount = solve(&distances, &times);

    log::info!("wins: {wincount}");
    wincount
}

fn puzzle2(input_path: &Path) -> u64 {
    let input: Vec<_> = read_lines(input_path).unwrap().collect();
    let time: u64 = input[0]
        .as_ref()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance: u64 = input[1]
        .as_ref()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let wincount = solve(&[distance], &[time]);

    log::info!("wins: {wincount}");
    wincount
}

fn solve(distances: &[u64], times: &[u64]) -> u64 {
    let mut wins = vec![];

    for (distance, time) in zip(distances, times) {
        let mut count = 0;
        for speed in 0..*time {
            let rem_time = time - speed; // can do this since 1 ms gives 1mm/ms
            if rem_time * speed > *distance {
                count += 1;
            }
        }
        wins.push(count);
    }
    wins.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_1() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle1(&input_path);
        assert_eq!(result, 288);
    }

    #[test]
    fn run_test_2() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle2(&input_path);
        assert_eq!(result, 71503);
    }
}
