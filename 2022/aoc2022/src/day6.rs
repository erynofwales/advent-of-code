use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main(filename: &str) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let file_contents: Vec<char> = reader
        .lines()
        .map(|l| {
            l.expect("Couldn't read line")
                .chars()
                .collect::<Vec<char>>()
        })
        .flatten()
        .collect();

    let (sop_offset, start_of_packet_marker) = file_contents
        .windows(4)
        .enumerate()
        .find(|(_, w)| HashSet::<&char>::from_iter(w.iter()).len() == 4)
        .expect("Couldn't find start-of-packet marker");

    let (sow_offset, start_of_window_marker) = file_contents
        .windows(14)
        .enumerate()
        .find(|(_, w)| HashSet::<&char>::from_iter(w.iter()).len() == 14)
        .expect("Couldn't find start-of-message marker");

    println!(
        "Part 1: Start-of-packet message is ‘{}’ at offset {}",
        start_of_packet_marker
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(""),
        sop_offset + 4
    );

    println!(
        "Part 2: Start-of-packet message is ‘{}’ at offset {}",
        start_of_window_marker
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(""),
        sow_offset + 14
    );

    Ok(())
}
