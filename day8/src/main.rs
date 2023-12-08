use common::{init_logs, read_lines};
use regex::Regex;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

fn main() {
    init_logs();
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");
    puzzle1(&input_path);
    puzzle2(&input_path);
}

fn puzzle1(input_path: &Path) -> u32 {
    const START: &str = "AAA";
    const END: &str = "ZZZ";

    let mut num_steps = 0;

    let input: Vec<_> = read_lines(&input_path)
        .unwrap()
        .map(|d| d.unwrap())
        .collect();

    let directions = &input[0];

    let node_re = Regex::new(r"(?<node>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();

    input.iter().skip(2).for_each(|line| {
        let caps = node_re.captures(&line).unwrap();
        let node = caps.name("node").unwrap().as_str();
        let left = caps.name("left").unwrap().as_str();
        let right = caps.name("right").unwrap().as_str();
        node_map.insert(node, (left, right));
    });

    let mut current = START;
    let mut current_instruction = 0;

    while current != END {
        let node = node_map.get(current).unwrap();
        current = match directions.chars().nth(current_instruction).unwrap() {
            'L' => node.0,
            'R' => node.1,
            _ => panic!("invalid input"),
        };
        num_steps += 1;
        current_instruction = (current_instruction + 1) % directions.len()
    }
    log::info!("num steps {num_steps}");
    num_steps
}

fn puzzle2(input_path: &Path) -> u32 {
    let mut num_steps = 0;

    let input: Vec<_> = read_lines(&input_path)
        .unwrap()
        .map(|d| d.unwrap())
        .collect();

    let directions = &input[0];

    let node_re = Regex::new(r"(?<node>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();

    input.iter().skip(2).for_each(|line| {
        let caps = node_re.captures(&line).unwrap();
        let node = caps.name("node").unwrap().as_str();
        let left = caps.name("left").unwrap().as_str();
        let right = caps.name("right").unwrap().as_str();
        node_map.insert(node, (left, right));
    });

    let mut currents: Vec<_> = node_map
        .keys()
        .cloned()
        .filter(|node| node.ends_with('A'))
        .collect();

    let mut current_instruction = 0;

    while !is_on_pos(&currents, 'Z') {
        log::debug!("currents {:?}", currents);
        for i in 0..currents.len() {
            let node = node_map.get(currents[i]).unwrap();
            currents[i] = match directions.chars().nth(current_instruction).unwrap() {
                'L' => node.0,
                'R' => node.1,
                _ => panic!("invalid input"),
            }
        }
        current_instruction = (current_instruction + 1) % directions.len();
        num_steps += 1;
    }
    log::info!("num steps {num_steps}");
    num_steps
}

fn is_on_pos(nodes: &[&str], node_end: char) -> bool {
    nodes.iter().all(|node| node.ends_with(node_end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_1() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle1(&input_path);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_result_2() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test2");
        let result = puzzle1(&input_path);
        assert_eq!(result, 6);
    }
    #[test]
    fn test_result_3() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test3");
        let result = puzzle2(&input_path);
        assert_eq!(result, 6);
    }
}
