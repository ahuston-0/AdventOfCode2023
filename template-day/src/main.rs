use std::path::PathBuf;

use common::{init_logs, read_lines};

fn main() {
    init_logs();
    puzzle1();
    puzzle2();
}

fn puzzle1() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    for line in read_lines(input_path.as_path()).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let _entry = line.unwrap();
    }
}

fn puzzle2() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    for line in read_lines(input_path.as_path()).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let _entry = line.unwrap();
    }
}
