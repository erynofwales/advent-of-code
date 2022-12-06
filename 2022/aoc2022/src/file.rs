use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;

pub fn line_reader_for_file(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}
