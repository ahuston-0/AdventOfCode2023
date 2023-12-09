use std::path::{Path, PathBuf};

use common::{init_logs, read_lines};

// Below imports some commonly used crates
// currently: (regex::Regex, num::Integer, rayon::prelude, itertools::Itertools)
//use common::prelude::*;

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    init_logs();
    puzzle1(&input_path);
    puzzle2(&input_path);
}

fn puzzle1(input_path: &Path) -> i64 {
    let mut res: Vec<i64> = Vec::new();
    for line in read_lines(input_path).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let entry = line.unwrap();
        let mut extrapolated_series: Vec<_> = vec![entry
            .split_ascii_whitespace()
            .map(|d| d.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()];

        log::trace!("{:?}", extrapolated_series);
        while extrapolated_series.last().unwrap().iter().any(|e| *e != 0) {
            let mut new_series = Vec::new();
            let series = extrapolated_series.last().unwrap();
            for i in 0..(series.len() - 1) {
                new_series.push(series[i + 1] - series[i]);
            }
            log::trace!("{:?}", new_series);
            extrapolated_series.push(new_series);
        }

        for i in (0..extrapolated_series.len() - 1).rev() {
            let last = extrapolated_series[i + 1].iter().cloned().last().unwrap();
            let curr = extrapolated_series[i].iter().cloned().last().unwrap();
            extrapolated_series[i].push(curr + last);
        }

        res.push(*extrapolated_series[0].last().unwrap());
    }
    let sum = res.iter().sum();
    log::info!("{sum}");
    sum
}

fn puzzle2(input_path: &Path) -> i64 {
    let mut res: Vec<i64> = Vec::new();
    for line in read_lines(input_path).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let entry = line.unwrap();
        let mut extrapolated_series: Vec<_> = vec![entry
            .split_ascii_whitespace()
            .map(|d| d.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()];

        log::trace!("{:?}", extrapolated_series);
        while extrapolated_series.last().unwrap().iter().any(|e| *e != 0) {
            let mut new_series = Vec::new();
            let series = extrapolated_series.last().unwrap();
            for i in 0..(series.len() - 1) {
                new_series.push(series[i + 1] - series[i]);
            }
            log::trace!("{:?}", new_series);
            extrapolated_series.push(new_series);
        }

        for i in (0..extrapolated_series.len() - 1).rev() {
            log::trace!("{:?}", extrapolated_series);
            let last = extrapolated_series[i + 1].to_vec()[0];
            let curr = extrapolated_series[i].to_vec()[0];
            extrapolated_series[i].insert(0, curr - last);
        }

        log::trace!("{:?}", extrapolated_series);
        res.push(extrapolated_series[0][0]);
    }
    let sum = res.iter().sum();
    log::info!("{sum}");
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_1() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle1(&input_path);
        assert_eq!(result, 114);
    }
    #[test]
    fn run_test_2() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle2(&input_path);
        assert_eq!(result, 2);
    }
}
