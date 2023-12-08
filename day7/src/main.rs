use common::{init_logs, read_lines};
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::iter::zip;
use std::path::{Path, PathBuf};
use uuid::Uuid;

fn main() {
    init_logs();
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");
    puzzle1(&input_path);
    puzzle2(&input_path);
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    WJ = 1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Win {
    High = 0,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn puzzle1(input_path: &Path) -> u32 {
    let mut hands: Vec<(Uuid, Vec<Card>)> = Vec::new();
    let mut bids: HashMap<Uuid, u32> = HashMap::new();
    let mut win_cons: Vec<(Uuid, Win)> = Vec::new();

    let hand_re = Regex::new(r"(?<hand>\w+) (?<bid>\d+)").unwrap();

    for line in read_lines(input_path).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let entry = line.unwrap();
        hand_re.captures_iter(&entry).for_each(|cap| {
            let hand_uuid = Uuid::new_v4();
            let hand = parse_hand(cap.name("hand").unwrap().as_str());
            let bid = cap.name("bid").unwrap().as_str().parse::<u32>().unwrap();
            hands.push((hand_uuid, hand.clone()));
            bids.insert(hand_uuid, bid);
            let win = check_wins(hand.clone());
            log::trace!("hand{:?}", hand);
            log::trace!("bid{:?}", bid);
            log::trace!("win{:?}", win);
            win_cons.push((hand_uuid, win));
        });
    }

    let mut ranks: Vec<_> = zip(hands, win_cons)
        .map(|((h_uuid, hand), (h_uuid_copy, win))| {
            if h_uuid != h_uuid_copy {
                panic!("uuid not matching")
            }
            (h_uuid, hand, win)
        })
        .collect();
    ranks.sort_by(|(_uuid_a, hand_a, win_a), (_uuid_b, hand_b, win_b)| {
        if win_a == win_b {
            for (a, b) in zip(hand_a, hand_b) {
                if *a != *b {
                    return a.cmp(b);
                }
            }
            cmp::Ordering::Equal
        } else {
            win_a.cmp(win_b)
        }
    });
    log::trace!("{:?}", ranks); // this sorts least to greatest... its okay we use this for the bet calculation
    let mut winnings = Vec::new();

    ranks
        .iter()
        .enumerate()
        .for_each(|(idx, (uuid, _hand, _win))| {
            log::trace!("{idx} {uuid}");
            let bid = bids.get(uuid).unwrap();
            log::trace!("{bid} {}", bid * (idx as u32 + 1));
            winnings.push(bids.get(uuid).unwrap() * (idx as u32 + 1));
        });

    let winnings = winnings.iter().sum::<u32>();

    log::info!("puzzle 1 {:?}", winnings);
    winnings
}

fn parse_hand(hand: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    for card in hand.chars() {
        let c = match card {
            '2' => Card::C2,
            '3' => Card::C3,
            '4' => Card::C4,
            '5' => Card::C5,
            '6' => Card::C6,
            '7' => Card::C7,
            '8' => Card::C8,
            '9' => Card::C9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => {
                panic!("not a valid card for the hand");
            }
        };
        cards.push(c);
    }
    cards
}

fn parse_hand_p2(hand: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    for card in hand.chars() {
        let c = match card {
            '2' => Card::C2,
            '3' => Card::C3,
            '4' => Card::C4,
            '5' => Card::C5,
            '6' => Card::C6,
            '7' => Card::C7,
            '8' => Card::C8,
            '9' => Card::C9,
            'T' => Card::T,
            'J' => Card::WJ,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => {
                panic!("not a valid card for the hand");
            }
        };
        cards.push(c);
    }
    cards
}
fn check_wins(hand: Vec<Card>) -> Win {
    let mut hand_freq: HashMap<&Card, u32> = HashMap::new();
    hand.iter().for_each(|card| {
        if !hand_freq.contains_key(card) {
            hand_freq.insert(card, 0);
        }
        hand_freq.entry(card).and_modify(|c| *c += 1);
    });

    let mut hand_freq: Vec<_> = hand_freq.values().collect();
    hand_freq.sort_unstable();

    return match hand_freq.as_slice() {
        [5] => Win::FiveKind,
        [1, 4] => Win::FourKind,
        [2, 3] => Win::FullHouse,
        [1, 1, 3] => Win::ThreeKind,
        [1, 2, 2] => Win::TwoPair,
        [1, 1, 1, 2] => Win::OnePair,
        _ => {
            log::trace!("{:?} {:?}", hand, hand_freq);

            Win::High
        }
    };
}

fn check_wins_p2(hand: Vec<Card>) -> Win {
    let mut hand_freq: HashMap<&Card, u32> = HashMap::new();
    let mut num_jokers: u32 = 0;
    hand.iter().for_each(|card| {
        if *card != Card::WJ {
            if !hand_freq.contains_key(card) {
                hand_freq.insert(card, 0);
            }
            hand_freq.entry(card).and_modify(|c| *c += 1);
        } else {
            num_jokers += 1;
        }
    });

    if num_jokers == 5 {
        return Win::FiveKind; // Force a return here, as below processing fails and returns Win::High for this edge case
    }

    let mut hand_freq: Vec<_> = hand_freq.values().cloned().collect();
    hand_freq.sort_unstable();

    if num_jokers > 0 {
        log::trace!("{:?} {:?}", hand, hand_freq);
        if let Some(last) = hand_freq.last_mut() {
            *last += num_jokers;
        }
        log::trace!("{:?} {:?}", hand, hand_freq);
    }

    return match hand_freq.as_slice() {
        [5] => Win::FiveKind,
        [1, 4] => Win::FourKind,
        [2, 3] => Win::FullHouse,
        [1, 1, 3] => Win::ThreeKind,
        [1, 2, 2] => Win::TwoPair,
        [1, 1, 1, 2] => Win::OnePair,
        _ => Win::High,
    };
}
fn puzzle2(input_path: &Path) -> u32 {
    let mut hands: Vec<(Uuid, Vec<Card>)> = Vec::new();
    let mut bids: HashMap<Uuid, u32> = HashMap::new();
    let mut win_cons: Vec<(Uuid, Win)> = Vec::new();

    let hand_re = Regex::new(r"(?<hand>\w+) (?<bid>\d+)").unwrap();

    for line in read_lines(input_path).unwrap() {
        // Make below variable "entry" instead once starting the puzzle
        // This is mostly to avoid clippy complaining x50
        let entry = line.unwrap();
        hand_re.captures_iter(&entry).for_each(|cap| {
            let hand_uuid = Uuid::new_v4();
            let hand = parse_hand_p2(cap.name("hand").unwrap().as_str());
            let bid = cap.name("bid").unwrap().as_str().parse::<u32>().unwrap();
            hands.push((hand_uuid, hand.clone()));
            bids.insert(hand_uuid, bid);
            let win = check_wins_p2(hand.clone());
            log::trace!("hand{:?}", hand);
            log::trace!("bid{:?}", bid);
            log::trace!("win{:?}", win);
            win_cons.push((hand_uuid, win));
        });
    }

    let mut ranks: Vec<_> = zip(hands, win_cons)
        .map(|((h_uuid, hand), (h_uuid_copy, win))| {
            if h_uuid != h_uuid_copy {
                panic!("uuid not matching")
            }
            (h_uuid, hand, win)
        })
        .collect();
    ranks.sort_by(|(_uuid_a, hand_a, win_a), (_uuid_b, hand_b, win_b)| {
        if win_a == win_b {
            for (a, b) in zip(hand_a, hand_b) {
                if *a != *b {
                    return a.cmp(b);
                }
            }
            cmp::Ordering::Equal
        } else {
            win_a.cmp(win_b)
        }
    });
    log::trace!("{:?}", ranks); // this sorts least to greatest... its okay we use this for the bet calculation
    let mut winnings = Vec::new();

    ranks
        .iter()
        .enumerate()
        .for_each(|(idx, (uuid, _hand, _win))| {
            log::trace!("{idx} {uuid}");
            let bid = bids.get(uuid).unwrap();
            log::trace!("{bid} {}", bid * (idx as u32 + 1));
            winnings.push(bids.get(uuid).unwrap() * (idx as u32 + 1));
        });

    let winnings = winnings.iter().sum::<u32>();

    log::info!("puzzle 2 {:?}", winnings);
    winnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/tests");
        let result = puzzle2(&input_path);
        assert_eq!(result, 5905);
    }
}
