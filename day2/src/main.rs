use regex::Regex;
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
    let idre = Regex::new(r"Game (\d+):").unwrap();
    let colorre = Regex::new(r"(\d+) (red|blue|green)").unwrap();
    const RED: u8 = 12;
    const GREEN: u8 = 13;
    const BLUE: u8 = 14;
    let mut sum = 0;
    for line in read_lines(input_path.as_path()).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let entry = line.unwrap();
        let idcap = idre.captures(&entry).unwrap();
        let id = &idcap[1];
        let validid: Vec<u32> = colorre
            .captures_iter(&entry)
            .map(|caps| {
                let color_lim = match caps.get(2).unwrap().as_str() {
                    "green" => GREEN,
                    "red" => RED,
                    "blue" => BLUE,
                    _ => panic!(
                        "color isnt a color... somehow. {}",
                        caps.get(2).unwrap().as_str()
                    ),
                };
                let count = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                if count > (color_lim as u32) {
                    count
                } else {
                    0 // so in theory a valid game should only ever have a count of 0 i think
                }
            })
            .collect();
        if validid.iter().sum::<u32>() > 0 {
            log::trace!("invalid game")
        } else {
            sum += id.parse::<u32>().unwrap();
        }
    }
    log::info!("sum of ids: {}", sum);
}

fn puzzle2() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");
    let colorre = Regex::new(r"(\d+) (red|blue|green)").unwrap();
    let mut sum = 0;
    for line in read_lines(input_path.as_path()).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let entry = line.unwrap();
        let mut color_vec = vec![0, 0, 0];
        log::trace!("{}", &entry);

        let _n: Vec<_> = colorre
            .captures_iter(&entry)
            .map(|caps| {
                let count = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let color_ind = match caps.get(2).unwrap().as_str() {
                    "green" => 0,
                    "red" => 1,
                    "blue" => 2,
                    _ => panic!(
                        "color isnt a color... somehow. {}",
                        caps.get(2).unwrap().as_str()
                    ),
                };
                log::trace!("count {count}, color {color_ind}, colors {:?}", color_vec);
                if color_vec[color_ind] < count {
                    color_vec[color_ind] = count;
                }
                count // gotta return something or else this gets optimized out
            })
            .collect();
        sum += color_vec.iter().product::<u32>();
    }
    log::info!("sum of ids: {}", sum);
}
