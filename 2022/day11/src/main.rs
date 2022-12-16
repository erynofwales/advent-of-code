use std::cell::RefCell;
use std::{env, fs};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Verbosity {
    None,
    Full,
}

#[derive(Clone, Debug, PartialEq)]
enum Term {
    Old,
    Fixed(i64),
}

impl TryFrom<&str> for Term {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s == "old" {
            Ok(Term::Old)
        } else if let Ok(n) = i64::from_str_radix(s, 10) {
            Ok(Term::Fixed(n))
        } else {
            Err(format!("Unable to parse {s} as Term"))
        }
    }
}

#[derive(Clone, Debug)]
struct Operation {
    left: Term,
    operator: Operator,
    right: Term,
}

impl Operation {
    fn perform(&self, item: &i64, verbosity: Verbosity) -> i64 {
        assert!(self.left == Term::Old);
        match self.operator {
            Operator::Add => match self.right {
                Term::Old => {
                    let result = item + item;
                    if verbosity == Verbosity::Full {
                        println!("    Worry level increases by itself to {result}.");
                    }
                    item + item
                }
                Term::Fixed(value) => {
                    let result = item + value;
                    if verbosity == Verbosity::Full {
                        println!("    Worry level increases by {value} to {result}.");
                    }
                    result
                }
            },
            Operator::Sub => match self.right {
                Term::Old => {
                    let result = item - item;
                    if verbosity == Verbosity::Full {
                        println!("    Worry level decreases by itself to {result}.");
                    }
                    result
                }
                Term::Fixed(value) => {
                    let result = item - value;
                    if verbosity == Verbosity::Full {
                        println!("    Worry level decreases by {value} to {result}.");
                    }
                    result
                }
            },
            Operator::Mul => match self.right {
                Term::Old => {
                    let result = item * item;
                    if verbosity == Verbosity::Full {
                        println!("    Worry level is multiplied by itself to {result}.");
                    }
                    result
                }
                Term::Fixed(value) => {
                    let result = item * value;
                    if verbosity == Verbosity::Full {
                        println!("    Worry level is multiplied by {value} to {result}.");
                    }
                    result
                }
            },
            Operator::Div => match self.right {
                Term::Old => {
                    let result = item / item;
                    if verbosity == Verbosity::Full {
                        println!("    Worry level is divided by itself to {result}.");
                    }
                    result
                }
                Term::Fixed(value) => {
                    let result = item / value;
                    if verbosity == Verbosity::Full {
                        println!("    Worry level is divided by {value} to {result}.");
                    }
                    result
                }
            },
        }
    }
}

impl Operation {
    fn new(left: Term, operator: Operator, right: Term) -> Operation {
        Operation {
            left,
            operator,
            right,
        }
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<&str> for Operator {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            _ => Err(format!("Unable to format {s} as Operator")),
        }
    }
}

struct InspectionReport {
    target_monkey: usize,
    item: i64,
}

impl InspectionReport {
    fn new(target_monkey: usize, item: i64) -> InspectionReport {
        InspectionReport {
            target_monkey,
            item,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    n: usize,
    items: RefCell<Vec<i64>>,
    operation: Operation,
    test_divisor: i64,
    target_monkey_if_true: usize,
    target_monkey_if_false: usize,
}

impl Monkey {
    fn take_item(&self, item: i64) {
        self.items.borrow_mut().push(item);
    }

    fn inspect_items(
        &self,
        with_anxiety_easying: bool,
        verbosity: Verbosity,
    ) -> Vec<InspectionReport> {
        if verbosity == Verbosity::Full {
            println!("Monkey {}:", self.n);
        }
        let reports = self
            .items
            .borrow()
            .iter()
            .map(|item| self._inspect_item(item, with_anxiety_easying, verbosity))
            .collect();

        self.items.borrow_mut().clear();

        reports
    }

    fn _inspect_item(
        &self,
        item: &i64,
        should_ease_anxiety: bool,
        verbosity: Verbosity,
    ) -> InspectionReport {
        if verbosity == Verbosity::Full {
            println!("  Monkey inspects an item with a worry level of {}", item);
        }
        let mut modified_worry_level = self.operation.perform(item, verbosity);

        if should_ease_anxiety {
            if verbosity == Verbosity::Full {
                println!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {}",
                    modified_worry_level
                );
            }
            modified_worry_level = modified_worry_level / 3;
        } else {
            modified_worry_level = modified_worry_level % 9699690;
            if verbosity == Verbosity::Full {
                println!(
                    "    Monkey gets bored with item. Normalizing worry level to {}",
                    modified_worry_level
                );
            }
        }

        if modified_worry_level % self.test_divisor == 0 {
            if verbosity == Verbosity::Full {
                println!(
                    "    Current worry level is divisible by {}",
                    self.test_divisor
                );
                println!(
                    "    Item with worry level {} is thrown to {}",
                    modified_worry_level, self.target_monkey_if_true
                );
            }
            InspectionReport::new(self.target_monkey_if_true, modified_worry_level)
        } else {
            if verbosity == Verbosity::Full {
                println!(
                    "    Current worry level is not divisible by {}",
                    self.test_divisor
                );
                println!(
                    "    Item with worry level {} is thrown to {}",
                    modified_worry_level, self.target_monkey_if_false
                );
            }
            InspectionReport::new(self.target_monkey_if_false, modified_worry_level)
        }
    }
}

struct MonkeyBuilder {
    n: usize,
    items: Vec<i64>,
    operation: Option<Operation>,
    test_divisor: i64,
    target_monkey_if_true: usize,
    target_monkey_if_false: usize,
}

impl MonkeyBuilder {
    fn new(n: usize) -> MonkeyBuilder {
        MonkeyBuilder {
            n,
            items: Vec::new(),
            operation: None,
            test_divisor: 0,
            target_monkey_if_true: 0,
            target_monkey_if_false: 0,
        }
    }

    fn build(self) -> Monkey {
        Monkey {
            n: self.n,
            items: RefCell::new(self.items),
            operation: self.operation.unwrap(),
            test_divisor: self.test_divisor,
            target_monkey_if_true: self.target_monkey_if_true,
            target_monkey_if_false: self.target_monkey_if_false,
        }
    }

    fn items(mut self, items: Vec<i64>) -> MonkeyBuilder {
        self.items = items;
        self
    }

    fn operation(mut self, operation: Operation) -> MonkeyBuilder {
        self.operation = Some(operation);
        self
    }

    fn test_divisor(mut self, divisor: i64) -> MonkeyBuilder {
        self.test_divisor = divisor;
        self
    }

    fn target_monkey_if_true(mut self, target_monkey: usize) -> MonkeyBuilder {
        self.target_monkey_if_true = target_monkey;
        self
    }

    fn target_monkey_if_false(mut self, target_monkey: usize) -> MonkeyBuilder {
        self.target_monkey_if_false = target_monkey;
        self
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Missing filename");
    let file_contents = fs::read_to_string(filename).expect("Unable to read {filename}");

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_builder: Option<MonkeyBuilder> = None;

    for line in file_contents.lines() {
        if line.starts_with("Monkey") {
            let split_line: Vec<&str> = line.split(&[' ', ':'][..]).collect();
            let n = usize::from_str_radix(split_line[1], 10).unwrap();
            assert!(monkey_builder.is_none());
            monkey_builder = Some(MonkeyBuilder::new(n));
        } else if line.starts_with("  Starting items:") {
            let mut split = line.split(": ").skip(1);
            let items: Vec<i64> = split
                .next()
                .unwrap()
                .split(", ")
                .map(|s| i64::from_str_radix(s, 10).unwrap())
                .collect();
            monkey_builder = Some(monkey_builder.unwrap().items(items));
        } else if line.starts_with("  Operation:") {
            let mut split = line.split(": ").skip(1);

            let operation_terms: Vec<&str> = split.next().unwrap().split(" ").collect();
            let left_term = Term::try_from(operation_terms[2]).unwrap();
            let right_term = Term::try_from(operation_terms[4]).unwrap();
            let operator = Operator::try_from(operation_terms[3]).unwrap();

            let operation = Operation::new(left_term, operator, right_term);
            monkey_builder = Some(monkey_builder.unwrap().operation(operation));
        } else if line.starts_with("  Test:") {
            let mut split = line.split(": ").skip(1);
            let divisor =
                i64::from_str_radix(split.next().unwrap().split(" ").skip(2).next().unwrap(), 10)
                    .unwrap();
            monkey_builder = Some(monkey_builder.unwrap().test_divisor(divisor));
        } else if line.starts_with("    If true:") {
            let split: Vec<&str> = line.split(" ").collect();
            let target_monkey = usize::from_str_radix(split.last().unwrap(), 10).unwrap();
            monkey_builder = Some(monkey_builder.unwrap().target_monkey_if_true(target_monkey));
        } else if line.starts_with("    If false:") {
            let split: Vec<&str> = line.split(" ").collect();
            let target_monkey = usize::from_str_radix(split.last().unwrap(), 10).unwrap();
            monkey_builder = Some(
                monkey_builder
                    .unwrap()
                    .target_monkey_if_false(target_monkey),
            );
        } else if line == "" {
            let builder = monkey_builder.take();
            monkeys.push(builder.unwrap().build())
        }
    }

    if let Some(builder) = monkey_builder.take() {
        monkeys.push(builder.build());
    }

    {
        let part1_monkeys = monkeys.clone();
        let mut part1_monkey_inspection_counts: Vec<u32> = vec![0; part1_monkeys.len()];

        for round in 1..=20 {
            println!("----- Round {round} -----");

            for monkey in &part1_monkeys {
                let reports = monkey.inspect_items(true, Verbosity::Full);
                part1_monkey_inspection_counts[monkey.n] += reports.len() as u32;

                for r in reports {
                    part1_monkeys[r.target_monkey].take_item(r.item);
                }
            }

            println!("After round {round}, the monkeys are holding items with these worry levels:");
            for monkey in &part1_monkeys {
                println!(
                    "Monkey {}: {}",
                    monkey.n,
                    monkey
                        .items
                        .borrow()
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
        }

        println!("----- Final Counts -----");
        for (i, c) in part1_monkey_inspection_counts.iter().enumerate() {
            println!("Monkey {i} inspected items {c} times.");
        }

        let mut part1_sorted_counts = part1_monkey_inspection_counts.clone();
        part1_sorted_counts.sort_by(|a, b| b.cmp(a));
        println!(
            "Part 1: monkey business: {} * {} = {}",
            part1_sorted_counts[0],
            part1_sorted_counts[1],
            part1_sorted_counts[0] * part1_sorted_counts[1]
        );
    }

    {
        let part2_monkeys = monkeys.clone();
        let mut part2_monkey_inspection_counts: Vec<u64> = vec![0; part2_monkeys.len()];
        for round in 1..=10000 {
            println!("----- Round {round} -----");

            for monkey in &part2_monkeys {
                let reports = monkey.inspect_items(false, Verbosity::None);
                part2_monkey_inspection_counts[monkey.n] += reports.len() as u64;

                for r in reports {
                    part2_monkeys[r.target_monkey].take_item(r.item);
                }
            }

            for (i, c) in part2_monkey_inspection_counts.iter().enumerate() {
                println!("Monkey {i} inspected items {c} times.");
            }
        }

        let mut part2_sorted_counts = part2_monkey_inspection_counts.clone();
        part2_sorted_counts.sort_by(|a, b| b.cmp(a));
        println!(
            "Part 2: monkey business: {} * {} = {}",
            part2_sorted_counts[0],
            part2_sorted_counts[1],
            part2_sorted_counts[0] * part2_sorted_counts[1]
        );
    }
}
