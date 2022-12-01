use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!("Command line args: {}", &args);

    let day1_datafile = &args[1];
    day1::main(day1_datafile)
        .expect("Unable to process day1 data file");
}
