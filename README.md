# Advent of Code Template for Rust

## Setup Instructions

*Note:* I recommend forking this repository before moving forward, as the script
used to generate the Advent of Code scripts is destructive and should only ever
need to be run once unless something drastic happens (ex. a change to the
template script that you want to cascade to the rest of the days). Better yet,
feel free to delete `generate.sh` once you've run it once! It will always be
available here.

Run the below script to dynamically generate scripts for all Advent of Code days

``` bash
chmod +x ./generate.sh
./generate.sh
```

Once you begin a challenge, you will need to first populate the inputs for that
day's challenge. For each day there is a input fule (`day1/resources/input`)
that you can paste the file into. Once done, `common::read_lines` will take
care of the rest and can start solving.

## Solving the Challenges

You'll be greeted with the following function for puzzles 1 and 2. As mentioned
in the comment, please change `_entry` to `entry` once you begin the challenge. 
You can then use `entry` to retrieve each line of the input file.

``` rust
fn puzzle1() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    for line in read_lines(input_path.as_path()).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let _entry = line.unwrap();
    }
}
```

## Running the Code

As this uses Cargo workspaces, you will need to manually specify which day you
want to run. To achieve this, run `cargo run -p day1` for whichever day you are
currently on.

Alternatively, I recommend the following workflow using `cargo-watch`.

``` bash
cargo install cargo-watch
cargo watch -x check -x fmt -x clippy -x 'run -p day1'
```
