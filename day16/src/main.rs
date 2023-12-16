use std::path::{Path, PathBuf};

use common::{init_logs, read_lines};

// Below imports some commonly used crates
// currently: (regex::Regex, num::Integer, rayon::prelude, itertools::Itertools)
use common::prelude::*;

extern crate queues;
use queues::*;

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    init_logs();
    puzzle1(&input_path);
    puzzle2(&input_path);
}

#[derive(PartialEq, Copy, Clone, Debug, Ord, Eq, PartialOrd)]
enum Direction {
    Right = 0,
    Up,
    Left,
    Down,
}

struct State {
    pos: Pos,
    dir: Direction,
}

impl State {
    pub fn move_state(&self) -> Pos {
        match self.dir {
            Direction::Right => (self.pos.0, self.pos.1 + 1),
            Direction::Up => (self.pos.0 - 1, self.pos.1),
            Direction::Left => (self.pos.0, self.pos.1 - 1),
            Direction::Down => (self.pos.0 + 1, self.pos.1),
        }
    }
}

type Pos = (i64, i64);

fn puzzle1(input_path: &Path) -> u64 {
    let input = read_lines(input_path)
        .unwrap()
        .map(|line| line.unwrap())
        .collect_vec();

    let visited = Vec::new();
    let path = trace_beam(&input, (0, -1), Direction::Right, &visited);
    let ans = path.iter().map(|(pos, _dir)| pos).sorted().dedup().count();
    log::error!("{ans}");
    ans.try_into().unwrap()
}

fn trace_beam(
    map: &[String],
    pos: Pos,
    dir: Direction,
    visited: &[(Pos, Direction)],
) -> Vec<(Pos, Direction)> {
    let mut visited: Vec<(Pos, Direction)> = visited.iter().cloned().sorted().dedup().collect_vec();
    let mut queue = queue![];
    let _ = queue.add((pos, dir));
    while queue.size() > 0 {
        log::trace!("{:?}", queue);
        let (cpost, cdir) = queue.remove().unwrap();
        // check_bounds retusn false if move would be bad
        if !check_bounds((map.len() as i64, map[0].len() as i64), cpost, cdir) {
            continue;
        }
        let cpos = State {
            pos: cpost,
            dir: cdir,
        }
        .move_state();
        log::trace!("{:?} {:?} {:?}", cpos, cdir, queue);
        if visited.contains(&(cpos, cdir)) {
            continue; // all possible paths explores from here
        }
        visited.push((cpos, cdir));
        /*
        If the beam encounters empty space (.), it continues in the same direction.
        If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would continue downward from the mirror's column.
        If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter as if the splitter were empty space. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
        If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues upward from the splitter's column and one that continues downward from the splitter's column.

             */
        match map[cpos.0 as usize].chars().nth(cpos.1 as usize).unwrap() {
            '.' => {
                let _ = queue.add((cpos, cdir));
            }
            '/' => match cdir {
                Direction::Right => {
                    let _ = queue.add((cpos, Direction::Up));
                }
                Direction::Left => {
                    let _ = queue.add((cpos, Direction::Down));
                }
                Direction::Up => {
                    let _ = queue.add((cpos, Direction::Right));
                }
                Direction::Down => {
                    let _ = queue.add((cpos, Direction::Left));
                }
            },
            '\\' => match cdir {
                Direction::Right => {
                    let _ = queue.add((cpos, Direction::Down));
                }
                Direction::Left => {
                    let _ = queue.add((cpos, Direction::Up));
                }
                Direction::Up => {
                    let _ = queue.add((cpos, Direction::Left));
                }
                Direction::Down => {
                    let _ = queue.add((cpos, Direction::Right));
                }
            },
            '|' => match cdir {
                Direction::Up | Direction::Down => {
                    let _ = queue.add((cpos, cdir));
                }
                Direction::Left | Direction::Right => {
                    let _ = queue.add((cpos, Direction::Up));
                    let _ = queue.add((cpos, Direction::Down));
                }
            },
            '-' => match cdir {
                Direction::Right | Direction::Left => {
                    let _ = queue.add((cpos, cdir));
                }
                Direction::Up | Direction::Down => {
                    let _ = queue.add((cpos, Direction::Left));
                    let _ = queue.add((cpos, Direction::Right));
                }
            },
            _ => panic!("invalid board"),
        }
    }
    visited
}

fn check_bounds(ends: Pos, pos: Pos, dir: Direction) -> bool {
    match (pos, dir) {
        ((_, 0), Direction::Left) | ((0, _), Direction::Up) => false,
        _ => {
            // log::error!("{:?} {:?} {:?}", ends, pos, dir);
            if pos.0 == ends.0 - 1 && dir == Direction::Down {
                return false;
            }
            if pos.1 == ends.1 - 1 && dir == Direction::Right {
                return false;
            }
            true
        }
    }
}

fn puzzle2(input_path: &Path) -> u64 {
    let input = read_lines(input_path)
        .unwrap()
        .map(|line| line.unwrap())
        .collect_vec();

    let visited = Vec::new();
    let mut paths = vec![];

    paths.par_extend((0..input.len()).into_par_iter().map(|i| {
        trace_beam(&input, (i as i64, -1), Direction::Right, &visited)
            .iter()
            .map(|(pos, _dir)| pos)
            .sorted()
            .dedup()
            .count()
    }));
    paths.par_extend((0..input.len()).into_par_iter().map(|i| {
        trace_beam(
            &input,
            (i as i64, input.len() as i64),
            Direction::Left,
            &visited,
        )
        .iter()
        .map(|(pos, _dir)| pos)
        .sorted()
        .dedup()
        .count()
    }));
    paths.par_extend((0..input[0].len()).into_par_iter().map(|i| {
        trace_beam(&input, (-1, i as i64), Direction::Down, &visited)
            .iter()
            .map(|(pos, _dir)| pos)
            .sorted()
            .dedup()
            .count()
    }));
    paths.par_extend((0..input[0].len()).into_par_iter().map(|i| {
        trace_beam(
            &input,
            (input[0].len() as i64, i as i64),
            Direction::Up,
            &visited,
        )
        .iter()
        .map(|(pos, _dir)| pos)
        .sorted()
        .dedup()
        .count()
    }));

    let ans = paths.iter().max().unwrap();
    log::error!("{ans}");
    (*ans).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_1() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle1(&input_path);
        assert_eq!(result, 46);
    }
}
