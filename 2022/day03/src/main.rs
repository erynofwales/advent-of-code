use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter::Iterator;

fn priority_items_in_knapsack(knapsack: &str) -> Vec<char> {
    assert!(knapsack.len() % 2 == 0);

    let compartment_size: usize = knapsack.len() / 2;

    let (left_compartment, right_compartment) = knapsack.split_at(compartment_size);
    let left_compartment_set: HashSet<char> = HashSet::from_iter(left_compartment.chars());
    let right_compartment_set: HashSet<char> = HashSet::from_iter(right_compartment.chars());

    left_compartment_set
        .intersection(&right_compartment_set)
        .map(|c| *c)
        .collect()
}

fn badge_for_group(group: (&str, &str, &str)) -> Vec<char> {
    let a_set: HashSet<char> = HashSet::from_iter(group.0.chars());
    let b_set: HashSet<char> = HashSet::from_iter(group.1.chars());
    let c_set: HashSet<char> = HashSet::from_iter(group.2.chars());

    let ab_intersection: HashSet<char> = a_set.intersection(&b_set).map(|c| *c).collect();
    ab_intersection.intersection(&c_set).map(|c| *c).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Missing filename argument");

    let file_contents = fs::read_to_string(&filename).expect("Unable to read file");
    let mut lines = file_contents.lines().peekable();

    let mut priority_letters: Vec<char> = Vec::new();
    let mut badges: Vec<char> = Vec::new();

    while lines.peek().is_some() {
        let a = lines.next().unwrap();
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();

        priority_letters.extend(priority_items_in_knapsack(&a));
        priority_letters.extend(priority_items_in_knapsack(&b));
        priority_letters.extend(priority_items_in_knapsack(&c));

        badges.extend(badge_for_group((&a, &b, &c)));
    }

    const LOWERCASE_A_SCORE: u32 = 'a' as u32;
    const UPPERCASE_A_SCORE: u32 = 'A' as u32;

    let sum_of_scored_priority_letters: u32 = priority_letters
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

    println!(
        "Part 1: sum of scores of all priority items: {}",
        sum_of_scored_priority_letters
    );

    let sum_of_badges: u32 = badges
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

    println!("Part 2: sum of badges: {}", sum_of_badges);
}
