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
    P: AsRef<Path> + std::fmt::Debug + Copy,
{
    let file = File::open(filename)?;
    if file.metadata().unwrap().len() < 25 {
        log::error!("{:?} is less than 25 bytes, chances are you forgot to supply the input. Update the file and try again.", filename);
        quit::with_code(1);
    }

    Ok(BufReader::new(file).lines())
}
