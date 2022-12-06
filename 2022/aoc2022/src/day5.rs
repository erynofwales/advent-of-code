use std::fmt;
use std::io::Result;

use crate::file::line_reader_for_file;

#[derive(PartialEq)]
enum State {
    StartingState,
    Instructions
}

#[derive(Debug)]
struct Stacks(Vec<Vec<String>>);

impl Stacks {
    fn part1_perform(&mut self, instruction: &Instruction) -> std::result::Result<(), &'static str> {
        for _ in 0..instruction.quantity {
            let item = self.0[instruction.from_stack].pop().unwrap();
            self.0[instruction.to_stack].push(item);
        }
        Ok(())
    }

    fn part2_perform(&mut self, instruction: &Instruction) -> std::result::Result<(), &'static str> {
        let from_stack_len: usize = self.0[instruction.from_stack].len();
        let index_of_last_n = from_stack_len - instruction.quantity;

        let last_n = Vec::from(self.0[instruction.from_stack].split_at(index_of_last_n).1);
        self.0[instruction.to_stack].extend(last_n.into_iter());

        let _ = self.0[instruction.from_stack].drain(index_of_last_n..);

        Ok(())
    }

    fn tops(&self) -> impl Iterator<Item = &str> {
        self.0.iter().filter_map(|s| s.last()).map(|s| s.as_str())
    }
}

impl fmt::Display for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().enumerate()
            .map(|(i, v)| format!("{}: {}", i, v.join(", ")))
            .collect::<Vec<String>>()
            .join("\n"))
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from_stack: usize,
    to_stack: usize
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(s: &str) -> std::result::Result<Instruction, Self::Error> {
        let quantity: usize;
        let from_stack: usize;
        let to_stack: usize;

        let mut split_str = s.split(" ");

        if split_str.next() != Some("move") {
            return Err("Missing 'move'");
        }

        match split_str.next().map(|s| usize::from_str_radix(s, 10)) {
            Some(Ok(parsed_quantity)) => quantity = parsed_quantity,
            _ => return Err("Missing quantity value"),
        }

        if split_str.next() != Some("from") {
            return Err("Missing 'from'");
        }

        match split_str.next().map(|s| usize::from_str_radix(s, 10)) {
            Some(Ok(parsed_from_stack)) => from_stack = parsed_from_stack - 1,
            _ => return Err("Missing from value"),
        }

        if split_str.next() != Some("to") {
            return Err("Missing 'to'");
        }

        match split_str.next().map(|s| usize::from_str_radix(s, 10)) {
            Some(Ok(parsed_to_stack)) => to_stack = parsed_to_stack - 1,
            _ => return Err("Missing to value"),
        }

        Ok(Instruction {
            quantity: quantity, 
            from_stack: from_stack,
            to_stack: to_stack,
        })
    }
}

pub fn main(filename: &str) -> Result<()> {
    let mut line_reader = line_reader_for_file(filename)?.peekable();

    let first_line = line_reader.peek().unwrap().as_ref().unwrap();
    let number_of_stacks = (first_line.len() as f32 / 4.0).ceil() as usize;
    let mut part1_stacks = Stacks(vec![vec![]; number_of_stacks]);
    let mut part2_stacks = Stacks(vec![vec![]; number_of_stacks]);

    let mut state = State::StartingState;

    for line in line_reader {
        let line = line?;

        if line.is_empty() {
            assert!(state == State::StartingState);
            state = State::Instructions;
            continue;
        }

        match state {
            State::StartingState => {
                let mut chars = line.chars().peekable();
                let mut index_of_stack = 0;
                while chars.peek() != None {
                    // Read the line in 4 character chunks.
                    let stack: Vec<char> = chars.by_ref().take(4).collect();
                    if stack[0] == '[' {
                        part1_stacks.0[index_of_stack].insert(0, String::from(stack[1]));
                        part2_stacks.0[index_of_stack].insert(0, String::from(stack[1]));
                    }

                    index_of_stack += 1;
                }
            },
            State::Instructions => {
                let instruction = Instruction::try_from(line.as_str()).unwrap();
                let _ = part1_stacks.part1_perform(&instruction);
                let _ = part2_stacks.part2_perform(&instruction);
            },
        }
    }

    println!("{}", part1_stacks);
    println!("Part 1: tops of stacks: {}", part1_stacks.tops().collect::<Vec<&str>>().join(""));
    println!("{}", part2_stacks);
    println!("Part 2: tops of stacks: {}", part2_stacks.tops().collect::<Vec<&str>>().join(""));

    Ok(())
}
