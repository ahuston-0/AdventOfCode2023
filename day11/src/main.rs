use std::path::{Path, PathBuf};

use common::{init_logs, read_lines};

// Below imports some commonly used crates
// currently: (regex::Regex, num::Integer, rayon::prelude, itertools::Itertools)
use common::prelude::*;

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    init_logs();
    puzzle1(&input_path);
    puzzle2(&input_path);
}

fn puzzle1(input_path: &Path) -> u64 {
    let input: Vec<_> = read_lines(input_path)
        .unwrap()
        .map(|line| line.unwrap())
        .collect();

    // let input: Vec<_> = input.iter().map(|line| line.unwrap()).collect();

    let mut galaxies = Vec::new();

    input.iter().enumerate().for_each(|(idx, line)| {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((idx as i64, i as i64));
            }
        }
    });
    let sum = galaxies
        .iter()
        .combinations(2)
        .par_bridge()
        .map(|set| {
            let t = track(&input, *set[0], *set[1], 2);
            log::trace!("{:?} to {:?}, {:?}", set[0], set[1], t);
            t
        })
        .sum();
    log::info!("sum {}", sum);
    sum
}

type Node = (i64, i64);

fn track(map: &[String], start: Node, end: Node, jump: u64) -> u64 {
    let mut temp = start;
    let mut num_steps = 0;
    while temp != end {
        if temp.0 != end.0 {
            let vert = (temp.0 - end.0) / ((temp.0 - end.0).abs());
            log::trace!("vert {vert}");
            log::trace!("temp {:?}", temp);
            log::trace!("end {:?}", end);
            if is_void(map, 'v', (temp.0 - vert).try_into().unwrap()) {
                num_steps += jump;
            } else {
                num_steps += 1;
            }
            temp.0 -= vert;
        }
        if temp.1 != end.1 {
            let hor = (temp.1 - end.1) / ((temp.1 - end.1).abs());
            log::trace!("hor {hor}");
            log::trace!("temp {:?}", temp);
            log::trace!("end {:?}", end);
            if is_void(map, 'h', (temp.1 - hor).try_into().unwrap()) {
                num_steps += jump;
            } else {
                num_steps += 1;
            }
            temp.1 -= hor;
        }
    }
    num_steps
}

fn is_void(map: &[String], dir: char, loc: usize) -> bool {
    match dir {
        'v' => {
            return !map[loc].contains('#');
        }
        'h' => {
            return map.iter().all(|line| line.chars().nth(loc).unwrap() == '.');
        }
        _ => panic!("nope"),
    }
}

fn puzzle2(input_path: &Path) -> u64 {
    let input: Vec<_> = read_lines(input_path)
        .unwrap()
        .map(|line| line.unwrap())
        .collect();

    // let input: Vec<_> = input.iter().map(|line| line.unwrap()).collect();

    let mut galaxies = Vec::new();

    input.iter().enumerate().for_each(|(idx, line)| {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((idx as i64, i as i64));
            }
        }
    });
    let sum = galaxies
        .iter()
        .combinations(2)
        .par_bridge()
        .map(|set| {
            let t = track(&input, *set[0], *set[1], 100000);
            log::trace!("{:?} to {:?}, {:?}", set[0], set[1], t);
            t
        })
        .sum();
    log::info!("sum {}", sum);
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
        assert_eq!(result, 374);
    }
}
