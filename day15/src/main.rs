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
    let input = read_lines(input_path)
        .unwrap()
        .map(|line| line.unwrap())
        .take(1)
        .collect::<String>();
    let input = input
        .split(',')
        .map(|cluster| cluster.as_bytes())
        .collect_vec();
    log::trace!("{:?}", input);
    let ans: u64 = input.iter().map(|cluster| hash_alg(cluster)).sum();
    log::error!("{ans}");
    ans
}

fn puzzle2(input_path: &Path) -> u64 {
    let input = read_lines(input_path)
        .unwrap()
        .map(|line| line.unwrap())
        .take(1)
        .collect::<String>();
    let input = input.split(',').collect_vec();

    let lens_re = Regex::new(r"(?<label>\w+)(?:(?<eq>=)(?<lens>\d)|(?<min>-))").unwrap();

    let mut map: Vec<Vec<(&str, u64)>> = vec![vec![]; 256];

    input.iter().for_each(|label| {
        let cap = lens_re.captures(label).unwrap();
        let label = cap.name("label").unwrap().as_str();
        let hash: usize = hash_alg(label.as_bytes()) as usize;
        match cap.name("eq") {
            Some(_) => {
                let lens = cap.name("lens").unwrap().as_str().parse::<u64>().unwrap();
                match map[hash].iter().position(|pair| pair.0 == label) {
                    Some(ind) => {
                        let _ = std::mem::replace(&mut map[hash][ind], (label, lens));
                    }
                    None => {
                        map[hash].push((label, lens));
                    }
                }
            }
            None => {
                // means minus operator
                map[hash].retain(|val| val.0 != label);
            }
        }
    });
    let ans = map
        .iter()
        .enumerate()
        .map(|(lidx, lensbox)| {
            lensbox
                .iter()
                .enumerate()
                .map(|(bidx, pair)| (1 + lidx as u64) * (1 + bidx as u64) * pair.1)
                .sum::<u64>()
        })
        .sum();
    log::error!("{ans}");
    ans
}

fn hash_alg(inp: &[u8]) -> u64 {
    let mut cur = 0;
    /*
       Determine the ASCII code for the current character of the string.
       Increase the current value by the ASCII code you just determined.
       Set the current value to itself multiplied by 17.
       Set the current value to the remainder of dividing itself by 256.
    */

    for c in inp.iter() {
        cur += *c as u64;
        cur *= 17;
        cur %= 256;
    }
    cur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_1() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle1(&input_path);
        assert_eq!(result, 52);
    }
    #[test]
    fn run_test_2() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test2");
        let result = puzzle1(&input_path);
        assert_eq!(result, 1320);
    }
    #[test]
    fn run_test_3() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test2");
        let result = puzzle2(&input_path);
        assert_eq!(result, 145);
    }
}
