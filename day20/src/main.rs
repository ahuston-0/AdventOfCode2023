use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use common::{init_logs, read_lines};

// Below imports some commonly used crates
// currently: (regex::Regex, num::Integer, rayon::prelude, itertools::Itertools)
use common::prelude::*;
use queues::*;

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
    let input = input.iter().map(|line| line.as_str()).collect_vec();
    let mut modset = build_module_engine(&input);
    dump_modset(&modset);
    let out = simulate(&mut modset, 1000);
    log::error!("{:?}", out);
    let res = (out.0 * out.1).try_into().unwrap();
    log::error!("{res}");
    res
}

fn puzzle2(input_path: &Path) -> u64 {
    let input = read_lines(input_path)
        .unwrap()
        .map(|line| line.unwrap())
        .collect_vec();
    let input = input.iter().map(|line| line.as_str()).collect_vec();
    let modset = build_module_engine(&input);
    dump_modset(&modset);
    let out = dump_modset_graphviz(&modset);
    log::debug!("{out}");
    let mut outpath = PathBuf::from(input_path.parent().unwrap());
    outpath.push("out.dot");
    log::debug!("{:?}", outpath);
    std::fs::write(outpath, out).unwrap();
    // rest of this was done with pen and paper... will code a solution to this in the future
    0
}

type OutType<'a> = Vec<&'a str>;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone)]
enum Module<'a> {
    FlipFlop(bool, OutType<'a>),
    Conjunction(OutType<'a>, Vec<bool>, OutType<'a>),
    Broadcaster(OutType<'a>),
}

type ModuleSet<'a> = HashMap<&'a str, Module<'a>>;

fn build_module_engine<'a>(modconfig: &[&'a str]) -> ModuleSet<'a> {
    let mut modset = HashMap::new();
    let mod_re = Regex::new(r"(?<op>%|&)*(?<module>\w+) -> (?<out>[\w ,]+)").unwrap();
    for module in modconfig.iter() {
        let caps = mod_re.captures(module).unwrap();
        let op = caps.name("op");
        let module = caps.name("module").unwrap().as_str();
        let out = caps
            .name("out")
            .unwrap()
            .as_str()
            .split(',')
            .map(|out| out.trim())
            .collect_vec();
        match op {
            Some(opr) if opr.as_str() == "%" => {
                // flipflop
                modset.insert(module, Module::FlipFlop(false, out.clone()));
            }
            Some(opr) if opr.as_str() == "&" => {
                //conjunction
                modset.insert(
                    module,
                    Module::Conjunction(Vec::new(), Vec::new(), out.clone()),
                );
            }
            None => {
                if module == "broadcaster" {
                    modset.insert(module, Module::Broadcaster(out.clone()));
                } else {
                    panic!("non-broadcast module without operator")
                }
            }
            _ => panic!("invalid operator"),
        }
    }
    build_conjunctions(&modset)
}

fn build_conjunctions<'a>(modset: &ModuleSet<'a>) -> ModuleSet<'a> {
    let mut conjunctions: HashMap<&&str, (&Module, Vec<&'a str>)> = modset
        .iter()
        .filter(|(_k, v)| matches!(*v, Module::Conjunction { .. }))
        .map(|(k, v)| (k, (v, Vec::new())))
        .collect();

    for (modname, module) in modset.iter() {
        match module {
            Module::FlipFlop(_, outs)
            | Module::Conjunction(_, _, outs)
            | Module::Broadcaster(outs) => {
                for &out in outs.iter() {
                    if conjunctions.contains_key(&out) {
                        conjunctions.get_mut(&out).unwrap().1.push(modname);
                    }
                }
            }
        }
    }

    let mut modset_mut = modset.clone();
    for (modname, (_, inputs)) in conjunctions.iter_mut() {
        match modset_mut.get_mut(*modname).unwrap() {
            Module::Conjunction(modinputs, signals, _) => {
                modinputs.extend(inputs.iter());
                signals.extend(std::iter::repeat(false).take(inputs.len()));
            }
            _ => panic!("Module type somehow changed"),
        }
    }

    modset_mut
}

fn simulate(modset: &mut ModuleSet<'_>, rounds: usize) -> (usize, usize) {
    let mut queue = queue![];
    let mut highcount = 0;
    let mut lowcount = 0;
    for _ in 0..rounds {
        let _ = queue.add((false, "broadcaster", "button"));
        lowcount += 1;

        while queue.size() > 0 {
            let (signal, current, prev) = queue.remove().unwrap();
            let module = modset.get_mut(current);

            match module {
                Some(Module::Broadcaster(outs)) => {
                    for out in outs {
                        log::debug!("{current} -{signal}-> {out}");
                        let _ = queue.add((false, out, current));
                        lowcount += 1;
                    }
                }
                Some(Module::FlipFlop(state, outs)) => {
                    if !signal {
                        *state = !*state;
                        //since i just inverted it..
                        for out in outs {
                            log::debug!("{current} -{state}-> {out}");
                            let _ = queue.add((*state, out, current));
                            if *state {
                                highcount += 1;
                            } else {
                                lowcount += 1;
                            }
                        }
                    }
                }
                Some(Module::Conjunction(inputs, states, outs)) => {
                    let stateind = inputs.iter().position(|&input| input == prev).unwrap();
                    states[stateind] = signal;
                    let outstate = !states.iter().all(|&state| state);
                    for out in outs {
                        log::debug!("{current} -{outstate}-> {out}");
                        let _ = queue.add((outstate, out, current));
                        if outstate {
                            highcount += 1;
                        } else {
                            lowcount += 1;
                        }
                    }
                }
                None => {
                    log::error!("Receiving signal {signal} for {current}");
                }
            }
        }
        dump_modset(modset); //
    }
    (highcount, lowcount)
}

fn dump_modset(modset: &ModuleSet<'_>) {
    for module in modset.iter() {
        log::trace!("{:?}", module);
    }
}

fn dump_modset_graphviz(modset: &ModuleSet<'_>) -> String {
    let mut out = Vec::new();
    out.push(String::from("digraph modules {"));
    for module in modset.iter() {
        match module {
            (modname, Module::Broadcaster(outs)) => {
                let outnodes = outs.join(",");
                out.push(format!("{modname} -> {{{}}} [color=red]", outnodes));
                log::trace!("{:?}", out);
            }
            (modname, Module::Conjunction(_, _, outs)) => {
                let outnodes = outs.join(",");
                out.push(format!("{modname} -> {{{}}} [color=green]", outnodes));
                log::trace!("{:?}", out);
            }
            (modname, Module::FlipFlop(_, outs)) => {
                let outnodes = outs.join(",");
                out.push(format!("{modname} -> {{{}}} [color=blue]", outnodes));
                log::trace!("{:?}", out);
            }
        }
    }
    out.push(String::from("}"));
    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_1() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test1");
        let result = puzzle1(&input_path);
        assert_eq!(result, 32000000);
    }
    #[test]
    fn run_test_2() {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("resources/test2");
        let result = puzzle1(&input_path);
        assert_eq!(result, 11687500);
    }
}
