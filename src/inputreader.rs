use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines};

type LineResult = io::Result<Lines<BufReader<File>>>;

pub fn get_lines_from(filename: &str) -> LineResult {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}
