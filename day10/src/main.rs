use std::path::{Path, PathBuf};

use common::{init_logs, read_lines};

extern crate queues;

use queues::*;

// Below imports some commonly used crates
// currently: (regex::Regex, num::Integer, rayon::prelude, itertools::Itertools)
// use common::prelude::*;

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/test5");

    init_logs();
    puzzle1(&input_path);
    puzzle2(&input_path);
}

fn puzzle1(input_path: &Path) -> u64 {
    let map: Vec<_> = read_lines(input_path)
        .unwrap()
        .map(|line| line.unwrap())
        .collect();

    let mut start = (0, 0);
    for (idx, line) in map.iter().enumerate() {
        if line.contains('S') {
            start = (idx, line.chars().position(|c| c == 'S').unwrap())
        }
    }

    let (_, num_steps) = bfs(&map, start);

    log::debug!("{:?}", start);
    log::debug!("{:?}", num_steps);
    log::info!("{:?}", num_steps / 2);

    num_steps / 2
}

fn get_char(map: &[String], node: (usize, usize)) -> char {
    map[node.0].chars().nth(node.1).unwrap()
}

fn bfs(map: &[String], start: (usize, usize)) -> (Vec<(usize, usize)>, u64) {
    let mut num_steps = 0;
    let mut q: Queue<(usize, usize)> = queue![];
    let mut visited: Vec<(usize, usize)> = vec![];

    let _ = q.add((start.0, start.1));

    while q.size() > 0 {
        let node = q.remove().unwrap();
        if visited.contains(&node) {
            continue;
        }
        let mut ootl = false;
        log::trace!("bfs node {:?}", node);
        match map[node.0].chars().nth(node.1).unwrap() {
            '-' => {
                let _ = q.add((node.0, node.1 + 1));
                let _ = q.add((node.0, node.1 - 1));
            }
            '7' => {
                let _ = q.add((node.0, node.1 - 1));
                let _ = q.add((node.0 + 1, node.1));
            }
            '|' => {
                let _ = q.add((node.0 - 1, node.1));
                let _ = q.add((node.0 + 1, node.1));
            }
            'J' => {
                let _ = q.add((node.0 - 1, node.1));
                let _ = q.add((node.0, node.1 - 1));
            }
            'L' => {
                let _ = q.add((node.0 - 1, node.1));
                let _ = q.add((node.0, node.1 + 1));
            }
            'F' => {
                let _ = q.add((node.0 + 1, node.1));
                let _ = q.add((node.0, node.1 + 1));
            }
            'S' => {
                if node.0 != 0
                    && ['S', '|', 'F', '7'].contains(&get_char(map, (node.0 - 1, node.1)))
                {
                    let _ = q.add((node.0 - 1, node.1));
                }
                if ['S', '|', 'L', 'J'].contains(&get_char(map, (node.0 + 1, node.1))) {
                    let _ = q.add((node.0 + 1, node.1));
                }
                if node.1 != 0
                    && ['S', '-', 'F', 'L'].contains(&get_char(map, (node.0, node.1 - 1)))
                {
                    let _ = q.add((node.0, node.1 - 1));
                }
                if ['S', '-', '7', 'J'].contains(&get_char(map, (node.0, node.1 + 1))) {
                    let _ = q.add((node.0, node.1 + 1));
                }
            }
            _ => ootl = true,
        }
        visited.push(node);
        if !ootl {
            num_steps += 1;
            log::trace!("ootl {:?}", map[node.0].chars().nth(node.1).unwrap());
        }
        // log::trace!("{:?}", visited);
    }
    log::error!("{}", visited.len());
    (visited, num_steps)
}

fn puzzle2(input_path: &Path) -> u64 {
    let map: Vec<_> = read_lines(input_path)
        .unwrap()
        .map(|line| line.unwrap())
        .collect();

    let mut exp_map: Vec<String> = Vec::<String>::with_capacity(map.len() * 3);
    for _i in 0..map.len() * 3 {
        exp_map.push(String::new());
    }
    map.iter().enumerate().for_each(|(i, s)| {
        s.chars().for_each(|c| match c {
            'S' => {
                for row in exp_map.iter_mut().skip(i * 3).take(3) {
                    row.push_str("SSS");
                }
            }
            '|' => {
                for row in exp_map.iter_mut().skip(i * 3).take(3) {
                    row.push_str(".|.");
                }
            }
            '-' => {
                exp_map[i * 3].push_str("...");
                exp_map[i * 3 + 1].push_str("---");
                exp_map[i * 3 + 2].push_str("...");
            }
            'L' => {
                exp_map[i * 3].push_str(".|.");
                exp_map[i * 3 + 1].push_str(".L-");
                exp_map[i * 3 + 2].push_str("...");
            }
            'J' => {
                exp_map[i * 3].push_str(".|.");
                exp_map[i * 3 + 1].push_str("-J.");
                exp_map[i * 3 + 2].push_str("...");
            }
            '7' => {
                exp_map[i * 3].push_str("...");
                exp_map[i * 3 + 1].push_str("-7.");
                exp_map[i * 3 + 2].push_str(".|.");
            }
            'F' => {
                exp_map[i * 3].push_str("...");
                exp_map[i * 3 + 1].push_str(".F-");
                exp_map[i * 3 + 2].push_str(".|.");
            }
            '.' => {
                for row in exp_map.iter_mut().skip(i * 3).take(3) {
                    row.push_str("...");
                }
            }
            _ => panic!("all edge cases should be covered..."),
        })
    });

    let mut start = (0, 0);
    for (idx, line) in exp_map.iter().enumerate() {
        if line.contains('S') {
            start = (idx, line.chars().position(|c| c == 'S').unwrap())
        }
    }

    dump_grid(&map);
    dump_grid(&exp_map);

    let (visited, _) = bfs(&exp_map, start);
    log::error!("{}", visited.len());
    let mut visited2 = vec![];
    let mut q: Queue<(usize, usize)> = queue![];
    let mut count = 0;

    for i in 0..exp_map[0].len() {
        if get_char(&exp_map, (0, i)) == '.' {
            let _ = q.add((0, i));
        }
        let len = exp_map.len() - 1;
        if get_char(&exp_map, (len, i)) == '.' {
            let _ = q.add((len, i));
        }
    }

    for (i, row) in exp_map.iter().enumerate() {
        if get_char(&exp_map, (i, 0)) == '.' {
            let _ = q.add((i, 0));
        }
        let len = row.len() - 1;
        if get_char(&exp_map, (i, len)) == '.' {
            let _ = q.add((i, len));
        }
    }

    log::trace!("second BFS on filler");
    while q.size() != 0 {
        let node = q.remove().unwrap();

        if visited2.contains(&node) {
            continue;
        }
        visited2.push(node);
        if visited.contains(&node) {
            continue; // hit another edge of the loop
        }
        if node.0 != 0 {
            let _ = q.add((node.0 - 1, node.1));
        }
        if node.1 != 0 {
            let _ = q.add((node.0, node.1 - 1));
        }
        if node.0 < exp_map.len() {
            let _ = q.add((node.0 + 1, node.1));
        }
        if node.1 < exp_map[0].len() {
            let _ = q.add((node.0, node.1 + 1));
        }
        if node.0 % 3 == 0 && node.1 % 3 == 0 && is_not_visited(&exp_map, &visited, node) {
            log::trace!("{} {}", node.0, node.1);
            count += 1;
        }
    }

    let visited_pipes = visited
        .iter()
        .filter(|d| get_char(&exp_map, **d) != '.')
        .collect::<Vec<_>>()
        .len()
        / 3;

    let mut ref_count = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i].chars().nth(j).unwrap() == '.' {
                ref_count += 1;
            }
        }
    }

    log::error!("c {count}");
    log::error!("rc {}", ref_count - 4);
    log::error!("s {}", map.len() * map[0].len());
    log::error!("vp {}", visited_pipes);
    let inside = (map.len() * map[0].len()) - visited_pipes - count + 2;
    log::error!(
        "m {}",
        (map.len() * map[0].len()) - visited_pipes - count + 2
    );
    log::error!("m {inside}",);
    (inside).try_into().unwrap()
}

fn is_not_visited(map: &[String], visited: &[(usize, usize)], node: (usize, usize)) -> bool {
    if node.0 + 3 > map.len() || node.1 + 3 > map[node.0].len() {
        return false;
    }
    for i in node.0..node.0 + 3 {
        for j in node.1..node.1 + 3 {
            if visited.contains(&(i, j)) {
                return false;
            }
        }
    }
    return true;
}

fn dump_grid(map: &[String]) {
    map.iter().for_each(|d| {
        log::error!("{d}");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_1() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle1(&input_path);
        assert_eq!(result, 4);
    }
    #[test]
    fn run_test_2() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test2");
        let result = puzzle1(&input_path);
        assert_eq!(result, 8);
    }
    #[test]
    fn run_test_3() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test3");
        let result = puzzle2(&input_path);
        assert_eq!(result, 4);
    }
    #[test]
    fn run_test_4() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test4");
        let result = puzzle2(&input_path);
        assert_eq!(result, 8);
    }
    #[test]
    fn run_test_5() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test5");
        let result = puzzle2(&input_path);
        assert_eq!(result, 10);
    }
    #[test]
    fn run_test_6() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test6");
        let result = puzzle2(&input_path);
        assert_eq!(result, 4);
    }
}
