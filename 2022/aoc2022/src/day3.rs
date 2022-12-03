use std::collections::HashSet;
use std::collections::hash_set::{Intersection, IntoIter};
use std::io::Result;
use std::iter::Sum;

use crate::file::line_reader_for_file;

pub fn main(filename: &str) -> Result<()> {
    let line_reader = line_reader_for_file(filename)?;

    let mut priority_letters: Vec<char> = Vec::new();

    for line in line_reader {
        let line = line?;
        assert!(line.len() % 2 == 0);

        let compartment_size: usize = line.len() / 2;

        let (left_compartment, right_compartment) = line.split_at(compartment_size);
        let left_compartment_set: HashSet<char> = HashSet::from_iter(left_compartment.chars());
        let right_compartment_set: HashSet<char> = HashSet::from_iter(right_compartment.chars());

        let intersection = left_compartment_set.intersection(&right_compartment_set);
        priority_letters.extend(intersection);
    }

    const LOWERCASE_A_SCORE: u32 = 'a' as u32;
    const UPPERCASE_A_SCORE: u32 = 'A' as u32;

    let sum_of_scored_priority_letters: u32 =
        priority_letters
            .iter()
            .map(|c| -> u32 {
                let char_value: u32 = (*c).into();
                if c.is_lowercase() {
                    char_value - LOWERCASE_A_SCORE + 1
                } else {
                    char_value - UPPERCASE_A_SCORE + 27
                }
            })
            .sum();

    println!("Part 1: sum of scores of all priority items: {}", sum_of_scored_priority_letters);

    Ok(())
}
