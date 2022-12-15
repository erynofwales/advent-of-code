use std::env;
use std::path::Path;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod file;
mod grid;

fn main() {
    let days = [
        day1::main,
        day2::main,
        day3::main,
        day4::main,
        day5::main,
        day6::main,
        day7::main,
        day8::main,
        day9::main,
        day10::main,
    ];

    let args: Vec<String> = env::args().collect();

    let datafile_path = Path::new(&args[1]);
    if !datafile_path.is_dir() {
        println!("{} is not a path to a directory!", datafile_path.display());
        return;
    }

    args.iter().skip(2).for_each(|arg| {
        if let Ok(module_number) = usize::from_str_radix(arg, 10) {
            println!("----- Day {} -----", module_number);
            if let Some(datafile) = datafile_path
                .join(format!("day{}-input.txt", module_number))
                .as_path()
                .to_str()
            {
                days[module_number - 1](datafile).expect("Unable to process day1 data file");
            }
        }
    });
}
