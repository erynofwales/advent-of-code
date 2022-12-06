use std::io::BufRead;
use std::io::Lines;
use std::io::Read;
use std::io::Result;

use crate::file::line_reader_for_file;

fn get_calorie_totals<R: Read + BufRead>(lines: Lines<R>) -> Result<Vec<u32>> {
    let mut elves: Vec<u32> = Vec::new();
    let mut current_calorie_count: u32 = 0;

    for line in lines {
        let line = line?;
        if line.is_empty() {
            elves.push(current_calorie_count);
            current_calorie_count = 0;
            continue;
        }

        let u32_value = u32::from_str_radix(&line, 10)
            .expect(format!("Couldn't read u32 value from string: {}", line).as_str());
        current_calorie_count += u32_value;
    }

    elves.sort_unstable_by(|a, b| b.cmp(a));
    Ok(elves)
}

pub fn main(input_filename: &str) -> Result<()> {
    let line_reader = line_reader_for_file(input_filename)
        .expect(format!("Unable to create line reader for file: {}", input_filename).as_str());
    let elves = get_calorie_totals(line_reader).expect("Unable to get knapsack calorie totals");

    println!(
        "Part 1: Elf with highest calorie count in knapsack: {}",
        elves[0]
    );

    let sum_of_top_three = &elves[0] + &elves[1] + &elves[2];
    println!(
        "Part 2: Elves with top 3 highest calorie counts in their knapsacks: {}, {}, {} = {}",
        &elves[0], &elves[1], &elves[2], sum_of_top_three
    );

    Ok(())
}
