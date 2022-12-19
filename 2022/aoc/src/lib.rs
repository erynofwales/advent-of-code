use std::{env, fs};

pub fn read_input_file_to_string() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Missing filename argument");
    fs::read_to_string(&filename).expect("Unable to read file")
}

