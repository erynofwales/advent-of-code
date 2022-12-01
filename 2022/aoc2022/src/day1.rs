use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Read;
use std::io::Result;

fn get_calorie_totals<R: Read + BufRead>(lines: Lines<R>) -> Vec<u32> {
    let mut elves: Vec<u32> = Vec::new();
    let mut current_calorie_count: u32 = 0;

    for line in lines {
        let unwrapped_line = line.unwrap();
        if unwrapped_line.is_empty() {
            elves.push(current_calorie_count);
            println!("Elf {}: {} calories", elves.len(), current_calorie_count);
            current_calorie_count = 0;
            continue;
        }

        let u32_value = u32::from_str_radix(&unwrapped_line, 10)
            .expect(format!("Couldn't read u32 value from string: {}", unwrapped_line).as_str());
        current_calorie_count += u32_value;
    }

    elves.sort_unstable_by(|a, b| b.cmp(a));
    elves
}

pub fn main(input_filename: &String) -> Result<()> {
    let file = File::open(input_filename)?; 
    let reader = BufReader::new(file);

    let elves = get_calorie_totals(reader.lines());

    println!("Elf with highest calorie count in knapsack: {}", elves[0]);

    Ok(())
}
