use common::{init_logs, read_lines};
use regex::{Captures, Regex};
use std::cmp;
use std::path::PathBuf;

fn main() {
    init_logs();
    puzzle1();
    puzzle2();
}

fn puzzle1() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    let card_re =
        Regex::new(r"Card[  ]+(?<id>\d+): (?<winning>[\d ]+) \| (?<possible>[\d ]+)").unwrap();
    let mut scores: Vec<u32> = vec![];
    for line in read_lines(input_path.as_path()).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let entry = line.unwrap();
        let score = get_card_score(&entry, &card_re);
        log::debug!("score {}", score);
        log::debug!("{:?}", scores);
        scores.push(score);
    }
    let ans: u32 = scores
        .iter()
        .cloned()
        .filter(|d| *d > 0)
        .map(|d| (2_u32).pow(d - 1))
        .sum();
    log::info!("sum {}", ans);
    assert_eq!(ans, 24542);
}

fn get_card_score(card: &str, card_re: &Regex) -> u32 {
    card_re
        .captures(card)
        .map(|caps| {
            log::trace!("{:?}", caps);
            log::debug!("id {}", caps.get(1).unwrap().as_str());
            let winning: Vec<_> = parse_score_chunk(&caps, "winning");
            let possible: Vec<_> = parse_score_chunk(&caps, "possible");
            let confirmed =
                possible.iter().copied().fold(
                    0,
                    |acc, e| {
                        if winning.contains(&e) {
                            acc + 1
                        } else {
                            acc
                        }
                    },
                );
            log::debug!("confirmed {}", confirmed);
            confirmed
        })
        .unwrap()
}

fn puzzle2() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");
    let card_re =
        Regex::new(r"Card[  ]+(?<id>\d+): (?<winning>[\d ]+) \| (?<possible>[\d ]+)").unwrap();
    let cards: Vec<_> = read_lines(input_path.as_path())
        .unwrap()
        .map(|c| c.unwrap())
        .collect();
    let mut copies: Vec<u32> = vec![0; cards.iter().len()];
    for (i, scorecard) in cards.iter().enumerate() {
        copies[i] += 1;
        let score = {
            let card_re = &card_re;
            card_re
                .captures(scorecard)
                .map(|caps| {
                    log::trace!("{:?}", caps);
                    log::debug!("id {}", caps.get(1).unwrap().as_str());
                    let winning: Vec<_> = parse_score_chunk(&caps, "winning");
                    let possible: Vec<_> = parse_score_chunk(&caps, "possible");
                    let confirmed = possible.iter().filter(|d| winning.contains(d)).count();
                    log::debug!("confirmed {}", confirmed);
                    confirmed
                })
                .unwrap()
        };
        let max_len = cmp::min(copies.iter().len() - i - 1, score);
        for j in i + 1..=i + max_len {
            copies[j] += copies[i];
        }
    }
    let scorecards: u32 = copies.iter().sum();
    log::info!("scorecard count {}", scorecards);
    assert_eq!(scorecards, 8736438);
}

fn parse_score_chunk(caps: &Captures, tag: &str) -> Vec<usize> {
    log::trace!("{}", tag);
    caps.name(tag)
        .unwrap()
        .as_str()
        .split(' ')
        .map(|d| d.trim())
        .filter(|d| !d.is_empty())
        .map(|d| {
            log::trace!("{}", d);
            d.parse::<usize>().unwrap()
        })
        .collect()
}
