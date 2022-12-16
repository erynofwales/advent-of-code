use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Missing filename argument");

    let file_contents = fs::read_to_string(&filename).expect("Unable to read file");
    let character_stream: Vec<char> = file_contents
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .flatten()
        .collect();

    let (sop_offset, start_of_packet_marker) = character_stream
        .windows(4)
        .enumerate()
        .find(|(_, w)| HashSet::<&char>::from_iter(w.iter()).len() == 4)
        .expect("Couldn't find start-of-packet marker");

    let (sow_offset, start_of_window_marker) = character_stream
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
}
