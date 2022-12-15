use std::env;
use std::fs;
use std::str::Lines;

fn get_calorie_totals(lines: Lines) -> Vec<u32> {
    let mut elves: Vec<u32> = Vec::new();
    let mut current_calorie_count: u32 = 0;

    for line in lines {
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
    elves
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Missing filename argument");

    let file_contents = fs::read_to_string(&filename).expect("Unable to read file");
    let lines = file_contents.lines();
    let elves = get_calorie_totals(lines);

    println!(
        "Part 1: Elf with highest calorie count in knapsack: {}",
        elves[0]
    );

    let sum_of_top_three = &elves[0] + &elves[1] + &elves[2];
    println!(
        "Part 2: Elves with top 3 highest calorie counts in their knapsacks: {}, {}, {} = {}",
        &elves[0], &elves[1], &elves[2], sum_of_top_three
    );
}
