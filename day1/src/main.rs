use common::{init_logs, read_lines};
use regex::Regex;
use std::path::PathBuf;

fn main() {
    init_logs();
    puzzle1();
    puzzle2();
}

fn puzzle1() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");
    let re = Regex::new(r"(\d).*(\d)").unwrap();
    let re_one = Regex::new(r"(\d)").unwrap();

    let mut sum = 0;

    for line in read_lines(input_path.as_path()).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let entry = line.unwrap();

        let captures = re
            .captures(&entry)
            .unwrap_or(re_one.captures(&entry).unwrap());
        let first = &captures[1];
        let last;

        if captures.get(2).is_none() {
            last = first;
        } else {
            last = &captures[captures.len() - 1];
        }
        let mut calibration = first.to_owned();
        calibration.push_str(last);
        let calint = calibration.parse::<u32>().unwrap();

        log::trace!("entry{entry}");
        log::trace!("capture{:?}", captures);
        log::trace!("f {first}");
        log::trace!("l {last}");
        log::trace!("c {calibration}");
        log::trace!("");
        sum += calint;
    }

    log::info!("sum:{sum}");
}

fn puzzle2() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut sum = 0;
    for line in read_lines(input_path.as_path()).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let entry = line.unwrap();
        let matches = long_find(&entry, &re);
        //let matches: Vec<_> = re.find_iter(&entry).map(|m| m.as_str()).collect();
        let first: &str;
        let last: &str;
        if matches[0].len() == 1 {
            first = matches[0];
        } else {
            first = str_to_digit(matches[0]);
        }
        let lasti = matches.len() - 1;
        if matches[lasti].len() == 1 {
            last = matches[lasti];
        } else {
            last = str_to_digit(matches[lasti]);
        }
        let mut calibration = first.to_owned();
        calibration.push_str(last);
        let calint = calibration.parse::<u32>().unwrap();

        log::trace!("entry{entry}");
        log::trace!("capture{:?}", matches);
        log::trace!("f {first}");
        log::trace!("l {last}");
        log::trace!("c {calibration}");
        log::trace!("");
        sum += calint;
    }
    log::info!("sum:{sum}");
}

fn str_to_digit(strmat: &str) -> &str {
    match strmat {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "zero" => "0",
        _ => panic!("Input is not a string repr of a digit"),
    }
}

// regex find_iter doesnt allow overlapping but the examples here have overlaps... time to do this the long way
fn long_find<'a>(haystack: &'a str, re: &'a Regex) -> Vec<&'a str> {
    let mut matches: Vec<_> = vec![];
    for i in 0..haystack.len() {
        if let Some(m) = re.find_at(haystack, i) {
            matches.push(m.into());
        }
    }
    matches
}
