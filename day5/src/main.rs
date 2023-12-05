use rayon::prelude::*;
use regex::{Captures, Regex};
use std::ops::Range;
use std::path::PathBuf;

use common::{init_logs, read_lines};

fn main() {
    init_logs();
    puzzle1();
    puzzle2();
}

#[derive(Debug)]
struct SeedMap {
    dest: Range<u64>,
    src: Range<u64>,
}

fn puzzle1() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    let input: Vec<_> = read_lines(input_path.as_path())
        .unwrap()
        .map(|d| d.unwrap())
        .collect();
    let seeds: Vec<_> = Regex::new(r"(\d+)")
        .unwrap()
        .captures_iter(&input[0])
        .map(|d| d.get(1).unwrap().as_str().parse::<u64>().unwrap())
        .collect();
    log::info!("{:?}", seeds);

    // get all the maps and strip the blank lines
    let maps: Vec<_> = input
        .iter()
        .skip(1)
        .filter(|d| !d.trim().is_empty())
        .collect();
    let map_re = Regex::new(r"(?<dst>\d+) (?<src>\d+) (?<len>\d+)").unwrap();
    // get each map
    let seed_soil = parse_map(&maps, &map_re);
    let maps = skip_map(&maps); // reset to next
    let soil_fertilizer = parse_map(&maps, &map_re);
    let maps = skip_map(&maps);
    let fertilizer_water = parse_map(&maps, &map_re);
    let maps = skip_map(&maps);
    let water_light = parse_map(&maps, &map_re);
    let maps = skip_map(&maps);
    let light_temp = parse_map(&maps, &map_re);
    let maps = skip_map(&maps);
    let temp_humid = parse_map(&maps, &map_re);
    let maps = skip_map(&maps);
    let humid_loc = parse_map(&maps, &map_re);
    log::error!("{:?}", seed_soil);

    let mut seed_locs: Vec<u64> = vec![];
    seeds.iter().for_each(|seed| {
        let soil = map_seed(seed, &seed_soil);
        let fert = map_seed(&soil, &soil_fertilizer);
        let water = map_seed(&fert, &fertilizer_water);
        let light = map_seed(&water, &water_light);
        let temp = map_seed(&light, &light_temp);
        let humidity = map_seed(&temp, &temp_humid);
        let loc = map_seed(&humidity, &humid_loc);
        seed_locs.push(loc);
    });
    log::error!("min seed loc {:?}", seed_locs.iter().min().unwrap());
}

fn map_seed(seed: &u64, map: &Vec<SeedMap>) -> u64 {
    let pos: Vec<_> = map.iter().filter(|soil| soil.src.contains(seed)).collect();
    if pos.iter().len() > 1 {
        panic!("why do we have two source ranges?")
    }

    if pos.iter().len() == 0 {
        return *seed;
    }
    let offset = seed - pos[0].src.start;
    pos[0].dest.start + offset
}

fn parse_map(maps: &Vec<&String>, map_re: &Regex) -> Vec<SeedMap> {
    maps.iter()
        .skip(1)
        .take_while(|x| !x.contains("map"))
        .map(|entry| {
            let caps = map_re.captures(entry).unwrap();
            let dst = extract_num(&caps, "dst");
            let src = extract_num(&caps, "src");
            let len = extract_num(&caps, "len");
            SeedMap {
                dest: dst..dst + len,
                src: src..src + len,
            }
        })
        .collect()
}
fn skip_map<'a>(maps: &'a Vec<&'a String>) -> Vec<&'a String> {
    maps.iter()
        .cloned()
        .skip(1)
        .skip_while(|entry| !entry.contains("map"))
        .collect()
}

fn extract_num(caps: &Captures, tag: &str) -> u64 {
    caps.name(tag).unwrap().as_str().parse::<u64>().unwrap()
}

fn puzzle2() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");

    let input: Vec<_> = read_lines(input_path.as_path())
        .unwrap()
        .map(|d| d.unwrap())
        .collect();
    let seeds: Vec<_> = Regex::new(r"(\d+)")
        .unwrap()
        .captures_iter(&input[0])
        .map(|d| d.get(1).unwrap().as_str().parse::<u64>().unwrap())
        .collect();
    log::error!("num chunks {}", seeds.iter().len() / 2);

    log::info!("{:?}", seeds);

    // get all the maps and strip the blank lines
    let maps: Vec<_> = input
        .iter()
        .skip(1)
        .filter(|d| !d.trim().is_empty())
        .collect();
    let map_re = Regex::new(r"(?<dst>\d+) (?<src>\d+) (?<len>\d+)").unwrap();
    // get each map
    let seed_soil = parse_map(&maps, &map_re);
    let maps = skip_map(&maps); // reset to next
    let soil_fertilizer = parse_map(&maps, &map_re);
    let maps = skip_map(&maps);
    let fertilizer_water = parse_map(&maps, &map_re);
    let maps = skip_map(&maps);
    let water_light = parse_map(&maps, &map_re);
    let maps = skip_map(&maps);
    let light_temp = parse_map(&maps, &map_re);
    let maps = skip_map(&maps);
    let temp_humid = parse_map(&maps, &map_re);
    let maps = skip_map(&maps);
    let humid_loc = parse_map(&maps, &map_re);
    log::debug!("{:?}", seed_soil);

    // doing this for all of the items (way too many)
    let seed_min: u64 = seeds
        .par_chunks(2)
        .enumerate()
        .map(|(idx, chunk)| { // expand (start, len) pairs from seed line into the full range
            let pair = chunk;
            log::trace!("{:?}", pair);
            let end = pair[0] + pair[1];
            let chunk: Vec<_> = (pair[0]..end).collect();
            log::error!("chunk {idx}");
            chunk
        })
        .flatten() // flatten because its a 2d array at this point
        .map(|seed| {
            // Apply the maps to get from seed to location
            log::trace!("seed {seed}");
            let soil = map_seed(&seed, &seed_soil);
            log::trace!("soil {soil}");
            let fert = map_seed(&soil, &soil_fertilizer);
            log::trace!("fert {fert}");
            let water = map_seed(&fert, &fertilizer_water);
            log::trace!("water {water}");
            let light = map_seed(&water, &water_light);
            log::trace!("light {light}");
            let temp = map_seed(&light, &light_temp);
            log::trace!("temp {temp}");
            let humidity = map_seed(&temp, &temp_humid);
            log::trace!("humidity {humidity}");
            let loc = map_seed(&humidity, &humid_loc);
            log::debug!("loc {loc}");
            log::trace!("-----------------");
            loc
        })
        .min() // get minimum seed location
        .unwrap();

    log::error!("min seed loc {:?}", seed_min);
}
