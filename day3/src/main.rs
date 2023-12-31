use common::{init_logs, read_lines};
use fancy_regex::Regex;
use itertools::Itertools;
use std::path::PathBuf;

// parts stored as row, column start, column end, part number
type PartNum = (usize, usize, usize, String);
fn main() {
    init_logs();

    let (grid, valid_parts) = puzzle1();
    puzzle2(grid, valid_parts);
}

fn puzzle1() -> (Vec<String>, Vec<PartNum>) {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("resources/input");
    let mut grid: Vec<String> = vec![];
    let mut possible: Vec<PartNum> = vec![];
    let num = Regex::new(r"(\d+)").unwrap();

    let gridcheck = Regex::new(r"[^\d.]+").unwrap();
    for (i, line) in read_lines(input_path.as_path()).unwrap().enumerate() {
        let entry = line.unwrap();
        // build grid
        grid.push(entry.clone());

        // build list of possible parts
        num.captures_iter(grid.last().unwrap().as_str())
            .for_each(|caps| {
                log::trace!("{:?}", caps);
                let cap = caps.unwrap().get(1).unwrap();
                possible.push((i, cap.start(), cap.end(), cap.as_str().to_string().clone()));
            });
    }
    log::trace!("{:?}", possible);

    // Get list of valid parts
    let valid_parts: Vec<_> = possible
        .iter()
        .filter(|p| check_grid(&grid, p, &gridcheck))
        .cloned()
        .collect();

    // Get solution
    let sum: usize = valid_parts
        .iter()
        .map(|p| p.3.parse::<usize>().unwrap())
        .sum();
    log::info!("sum:{}", sum);

    (grid, valid_parts)
}

fn check_grid(grid: &Vec<String>, pos: &PartNum, gridre: &Regex) -> bool {
    let min = usize_min_one(pos.0);
    let max = if pos.0 + 2 >= grid.len() {
        pos.0 + 1
    } else {
        pos.0 + 2
    };
    log::trace!("pos {:?}", pos);
    for i in grid.iter().take(max).skip(min) {
        let min = if pos.1 == 0 { 0 } else { pos.1 - 1 };
        let max = if pos.2 + 1 >= i.len() {
            pos.2
        } else {
            pos.2 + 1
        };
        log::trace!("{} {} {} {}", i[min..max].len(), min, max, &i[min..max]);

        if gridre.is_match(&i[min..max]).unwrap() {
            return true;
        }
    }
    false
}

// stored as row, column
type GearLoc = (usize, usize);

fn puzzle2(grid: Vec<String>, valid_parts: Vec<PartNum>) {
    let gearre = Regex::new(r"(\*)").unwrap();
    let mut gear_locs: Vec<GearLoc> = vec![];
    for (i, row) in grid.iter().enumerate() {
        for caps in gearre.captures_iter(row.as_str()) {
            let cap = caps.unwrap().get(1).unwrap();
            gear_locs.push((i, cap.start()));
        }
    }
    let valid_gears = check_gears(&gear_locs, valid_parts);
    log::debug!("{}", gear_locs.len());
    log::debug!("{}", valid_gears.len());
    let prod: usize = valid_gears
        .iter()
        .map(|g| g.1 .3.parse::<usize>().unwrap() * g.2 .3.parse::<usize>().unwrap())
        .sum();

    log::info!("gear ratios {}", prod);
}

type GearRatio = (GearLoc, PartNum, PartNum);

fn check_gears(gears: &Vec<GearLoc>, parts: Vec<PartNum>) -> Vec<GearRatio> {
    let mut valid_gears: Vec<GearRatio> = vec![];
    for g in gears {
        let pot_parts: Vec<PartNum> = get_part(g.0, g.1, &parts);
        let pot_parts: Vec<_> = pot_parts.iter().unique().collect();
        if pot_parts.len() == 2 {
            valid_gears.push((*g, pot_parts[0].clone(), pot_parts[1].clone()));
        }
    }
    valid_gears
}

fn usize_min_one(x: usize) -> usize {
    if x == 0 {
        0
    } else {
        x - 1
    }
}

fn get_part(x: usize, y: usize, parts: &[PartNum]) -> Vec<PartNum> {
    let nu_parts: Vec<_> = parts
        .iter()
        .filter(|g| {
            let min = usize_min_one(g.0);
            (min..=g.0 + 1).contains(&x)
        })
        .filter(|g| {
            let min = usize_min_one(g.1);
            (min..=g.2).contains(&y)
        })
        .cloned()
        .collect();
    log::debug!("{}", nu_parts.len());
    log::trace!("{} {}", x, y);
    log::trace!("{:?}", nu_parts);
    if nu_parts.len() > 2 {
        panic!("My assumption about the placement of the gears is wrong :(");
    }
    nu_parts
}
