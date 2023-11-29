use log::{self, LevelFilter};
use simple_logger::SimpleLogger;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path,
};

pub fn init_logs() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .env()
        .init()
        .unwrap();
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
