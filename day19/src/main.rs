use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use common::{init_logs, read_lines};

// Below imports some commonly used crates
// currently: (regex::Regex, num::Integer, rayon::prelude, itertools::Itertools)
use common::prelude::*;

fn main() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    init_logs();
    puzzle1(&input_path);
    puzzle2(&input_path);
}

fn puzzle1(input_path: &Path) -> u64 {
    let input = read_lines(input_path)
        .unwrap()
        .map(|line| line.unwrap())
        .collect_vec();
    let rules = build_rule_engine(
        &input
            .iter()
            .take_while(|line| !line.is_empty())
            .map(|line| line.as_str())
            .collect_vec(),
    );
    let parts = parse_parts(
        &input
            .iter()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .map(|line| line.as_str())
            .collect_vec(),
    );
    log::debug!("{:?}", rules);
    log::debug!("{:?}", parts);
    let results = parts
        .iter()
        .map(|part| state_machine(part, &rules))
        .filter(|out| out.1 == 'A')
        .map(|out| out.0.iter().map(|catrat| catrat.1 as u64).sum::<u64>())
        .sum();
    log::error!("{results}");
    results
}

fn puzzle2(input_path: &Path) -> u128 {
    let input = read_lines(input_path)
        .unwrap()
        .map(|line| line.unwrap())
        .collect_vec();
    let rules = build_rule_engine(
        &input
            .iter()
            .take_while(|line| !line.is_empty())
            .map(|line| line.as_str())
            .collect_vec(),
    );
    let parts = std::array::from_fn(|_| (1..=4000).collect_vec());
    log::debug!("{}", parts.len());
    log::debug!("{}", parts[0].len());
    let results = accepted(&rules, "in", parts);
    log::error!("{results}");
    results
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Rule<'a> {
    Condition(&'a str, &'a str, &'a str, &'a str),
    Default(&'a str),
}

type RuleOutput = (Part, char);

type Part = [(char, u16); 4];

type RuleSet<'a> = HashMap<&'a str, Vec<Rule<'a>>>;

fn get_rating(part: &Part, cat: char) -> u16 {
    match cat {
        'x' => part[0].1,
        'm' => part[1].1,
        'a' => part[2].1,
        's' => part[3].1,
        _ => panic!("invalid part"),
    }
}

fn parse_parts(parts: &[&str]) -> Vec<Part> {
    let part_re = Regex::new(r"(?<cat>\w)=(?<rating>\d+)").unwrap();
    parts
        .iter()
        .map(|part| {
            part_re
                .captures_iter(part)
                .map(|catrat| {
                    let cat = catrat.name("cat").unwrap().as_str().chars().next().unwrap();
                    let rat = catrat
                        .name("rating")
                        .unwrap()
                        .as_str()
                        .parse::<u16>()
                        .unwrap();
                    (cat, rat)
                })
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
}

fn build_rule_engine<'a>(rules: &[&'a str]) -> RuleSet<'a> {
    let mut rulemap = HashMap::new();
    let rule_re = Regex::new(r"(?<cat>\w)(?<op><|>)(?<val>\d+):(?<next_rule>\w+)").unwrap();
    for ruleset in rules.iter() {
        let mut rule_iter = rule_re
            .captures_iter(ruleset)
            .map(|rule| {
                log::trace!("{:?}", rule);
                let cat = rule.name("cat").unwrap().as_str();
                let op = rule.name("op").unwrap().as_str();
                let val = rule.name("val").unwrap().as_str();
                let next_rule = rule.name("next_rule").unwrap().as_str();
                Rule::Condition(cat, op, val, next_rule)
            })
            .collect_vec();
        let rule_name = ruleset.split_once('{').unwrap().0;

        let default_rule = Rule::Default(
            ruleset
                .split_terminator(',')
                .last()
                .unwrap()
                .split_once('}')
                .unwrap()
                .0,
        );
        rule_iter.push(default_rule);
        rulemap.insert(rule_name, rule_iter);
    }
    rulemap.insert("A", Vec::from([Rule::Default("A")]));
    rulemap.insert("R", Vec::from([Rule::Default("R")]));
    rulemap
}

fn state_machine(part: &Part, rules: &RuleSet) -> RuleOutput {
    let mut cur_rule = rules.get("in").unwrap();
    log::trace!("{:?}", part);
    'outer: loop {
        for rule in cur_rule.iter() {
            log::trace!("{:?}", rule);
            match rule {
                Rule::Default(next_rule) => {
                    if *next_rule == "A" || *next_rule == "R" {
                        return (*part, next_rule.chars().next().unwrap());
                    }
                    cur_rule = rules.get(next_rule).unwrap();
                    continue 'outer;
                }
                Rule::Condition(cat, op, val, next_rule) => {
                    let rating = get_rating(part, cat.chars().next().unwrap());
                    let val = val.parse::<u16>().unwrap();
                    match *op {
                        ">" => {
                            log::debug!("gt {rating} {val}");
                            if rating > val {
                                cur_rule = rules.get(next_rule).unwrap();
                                continue 'outer;
                            }
                        }
                        "<" => {
                            log::trace!("lt {rating} {val}");
                            if rating < val {
                                cur_rule = rules.get(next_rule).unwrap();
                                continue 'outer;
                            }
                        }
                        _ => panic!("invalid operator"),
                    }
                }
            }
        }
    }
}

fn accepted(rules: &RuleSet, curr: &str, mut ranges: [Vec<u16>; 4]) -> u128 {
    let mut result = 0;
    let ruleset = &rules[curr];
    log::debug!("range len {} x {}", ranges.len(), ranges[0].len());
    for rule in ruleset.iter() {
        log::debug!("{:?}", rule);
        log::debug!("result {:?}", result);
        match rule {
            Rule::Default(def) if *def == "A" => {
                return result + ranges.iter().map(|v| v.len() as u128).product::<u128>();
            }
            Rule::Default(def) if *def == "R" => {
                return result;
            }
            Rule::Default(def) => {
                return result + accepted(rules, def, ranges);
            }
            Rule::Condition(cat, op, val, next_rule) => {
                let i = "xmas"
                    .chars()
                    .position(|c| c == cat.chars().next().unwrap())
                    .unwrap();
                let mut newranges = ranges.clone();
                let val = val.parse().unwrap();
                (newranges[i], ranges[i]) = ranges[i].iter().partition(|part| match *op {
                    "<" => **part < val,
                    ">" => **part > val,
                    _ => panic!("invalid op"),
                });
                result += accepted(rules, next_rule, newranges);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_1() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle1(&input_path);
        assert_eq!(result, 19114);
    }
    #[test]
    fn run_test_2() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle2(&input_path);
        assert_eq!(result, 167409079868000);
    }
}
