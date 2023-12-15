use std::path::{Path, PathBuf};

use common::{init_logs, read_lines};

// Below imports some commonly used crates
// currently: (regex::Regex, num::Integer, rayon::prelude, itertools::Itertools)
use common::prelude::*;
use memoize::memoize;

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    init_logs();
    puzzle1(&input_path);
    puzzle2(&input_path);
}

fn puzzle1(input_path: &Path) -> u64 {
    let mut sum_counts = 0;
    let line_re = Regex::new(r"(?<subgroup>\d+)").unwrap();
    for line in read_lines(input_path).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let entry = line.unwrap();
        let split: Vec<_> = entry.split(" ").collect();
        let conditions = split[0];
        let groups: Vec<_> = line_re
            .captures_iter(split[1])
            .map(|subgroup| {
                subgroup
                    .name("subgroup")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();

        let scount = get_num_groups(conditions.to_string(), groups, false);
        log::error!("{:?}", scount);
        sum_counts += scount;
    }
    log::error!("{:?}", sum_counts);
    sum_counts.try_into().unwrap()
}

#[memoize]
fn get_num_groups(conditions: String, groups: Vec<usize>, prev_damaged: bool) -> usize {
    if groups.is_empty() || groups == [0] {
        return !conditions.contains('#') as usize;
    }
    if conditions.is_empty() {
        return 0; // should have been caught last time if this was a success;
    }

    let cond_char;
    let next_str;
    if conditions.len() == 1 {
        cond_char = conditions.chars().next().unwrap();
        next_str = "";
    } else {
        let condition_spl = conditions.split_at(1);
        cond_char = condition_spl.0.chars().next().unwrap();
        next_str = condition_spl.1;
    }
    match (cond_char, prev_damaged) {
        ('#', _) => {
            if groups[0] == 0 {
                return 0;
            } else {
                let mut ngroups = groups.clone();
                ngroups[0] -= 1;
                return get_num_groups(next_str.to_string(), ngroups, true);
            }
        }
        ('.', _) => {
            if groups.is_empty() {
                return 1;
            }
            if groups[0] == 0 {
                let groups: Vec<_> = groups.iter().cloned().skip(1).collect();
                return get_num_groups(next_str.to_string(), groups, false);
            } else if prev_damaged {
                return 0;
            }
            get_num_groups(next_str.to_string(), groups, false)
        }
        ('?', true) => {
            if groups[0] != 0 {
                let mut ngroups = groups.clone();
                ngroups[0] -= 1;
                get_num_groups(next_str.to_string(), ngroups, true)
            } else {
                let groups = groups.iter().cloned().skip(1).collect();
                get_num_groups(next_str.to_string(), groups, false)
            }
        }
        ('?', false) => {
            let mut num_pos = 0;
            if groups[0] != 0 {
                let mut ngroups = groups.clone();
                ngroups[0] -= 1;
                num_pos += get_num_groups(next_str.to_string(), ngroups.clone(), true)
            }
            num_pos += get_num_groups(next_str.to_string(), groups.clone(), false);
            num_pos
        }
        _ => panic!("invalid input"),
    }
}

fn puzzle2(input_path: &Path) -> u64 {
    let mut sum_counts = 0;
    let line_re = Regex::new(r"(?<subgroup>\d+)").unwrap();
    for line in read_lines(input_path).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let entry = line.unwrap();
        let split: Vec<_> = entry.split(" ").collect();
        let conditions = split[0];
        let groups: Vec<_> = line_re
            .captures_iter(split[1])
            .map(|subgroup| {
                subgroup
                    .name("subgroup")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();
        let mut spring = String::new();
        let mut spring_groups = Vec::new();
        for i in 0..5 {
            spring.push_str(conditions);
            if i != 4 {
                spring.push('?');
            }

            spring_groups.extend(groups.clone());
        }

        log::error!("{:?}", conditions);
        log::error!("{:?}", spring);
        log::error!("{:?}", groups);
        log::error!("{:?}", spring_groups);
        let scount = get_num_groups(spring, spring_groups, false);
        log::error!("{:?}", scount);
        sum_counts += scount;
    }
    log::error!("{:?}", sum_counts);
    sum_counts.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_1() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle1(&input_path);
        assert_eq!(result, 21);
    }
    #[test]
    fn run_test_2() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle2(&input_path);
        assert_eq!(result, 525152);
    }
}
