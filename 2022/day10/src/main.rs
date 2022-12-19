use std::{env, fs};

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn number_of_cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

struct CPU<'a> {
    instructions: Box<dyn Iterator<Item = Instruction> + 'a>,
    current_instruction: Option<Instruction>,
    cycle: u32,
    x: i32,
}

impl<'a> CPU<'a> {
    fn new<T: Iterator<Item = Instruction> + 'a>(instructions: T) -> CPU<'a> {
        CPU {
            instructions: Box::new(instructions),
            current_instruction: None,
            cycle: 0,
            x: 1,
        }
    }
}

impl Iterator for CPU<'_> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_instruction.is_none() {
            self.current_instruction = self.instructions.next();
            self.cycle = 0;
        }

        if let Some(current_instruction) = &self.current_instruction {
            let state = Some(State::from_cpu(&self));
            self.cycle += 1;

            if self.cycle >= current_instruction.number_of_cycles() {
                match current_instruction {
                    Instruction::Noop => {}
                    Instruction::AddX(value) => {
                        self.x += value;
                    }
                }

                self.current_instruction = self.instructions.next();
                self.cycle = 0;
            }

            state
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    x: i32,
}

impl State {
    fn from_cpu(cpu: &CPU) -> State {
        State { x: cpu.x }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Missing filename argument");

    let file_contents = fs::read_to_string(&filename).expect("Unable to read file");
    let lines = file_contents.lines();

    let cpu = CPU::new(lines.map(|line| -> Instruction {
        if line.starts_with("addx") {
            let value = i32::from_str_radix(line.split(" ").collect::<Vec<&str>>()[1], 10).unwrap();
            Instruction::AddX(value)
        } else {
            Instruction::Noop
        }
    }));

    let mut number_of_cycles = 0;
    let mut signal_strengths = 0;
    let mut cycles: Vec<State> = vec![];

    for (i, cycle) in cpu.enumerate() {
        cycles.push(cycle.clone());

        let x = cycle.x;

        let cycle_number = i as i32 + 1;
        if cycle_number % 40 == 20 {
            signal_strengths += x * cycle_number;
        }
        number_of_cycles = i;

        let i = i as i32;
        if i > 0 && i % 40 == 0 {
            println!("");
        }
        let horizontal_beam_position = i % 40;
        if horizontal_beam_position == (x - 1)
            || horizontal_beam_position == x
            || horizontal_beam_position == (x + 1)
        {
            print!("#");
        } else {
            print!(".");
        }
    }

    println!("");
    println!("There were {} instructions", number_of_cycles + 1);
    println!("Part 1: sum of signal strengths: {}", signal_strengths);
}
