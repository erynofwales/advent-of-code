use std::env;

mod day1;
mod day2;
mod day3;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() >= 3, "Missing command line arguments");

    println!("----- Day 1 -----");
    let day1_datafile = &args[1];
    day1::main(day1_datafile)
        .expect("Unable to process day1 data file");

    println!("----- Day 2 -----");
    let day2_datafile = &args[2];
    day2::main(day2_datafile.as_str())
        .expect("Unable to process day2 data file");

    println!("----- Day 3 -----");
    let day3_datafile = &args[3];
    day3::main(day3_datafile.as_str())
        .expect("Unable to process day3 data file");
}
